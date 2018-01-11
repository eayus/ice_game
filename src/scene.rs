use sfml::graphics::RenderWindow;
use sfml::window::Event;

use resources::Resources;
use ::SplashScene;
use menu::MainMenu;
use level::Level;

// TODO: Add transitions

pub trait Sceneable {
    fn update(&mut self, resources: &Resources) -> SceneAction;
    fn draw(&self, window: &mut RenderWindow);
    fn handle_event(&mut self, event: Event, resources: &Resources) -> SceneAction;
}

#[derive(Clone)]
pub enum Scene {
    Splash,
    Level(usize), // Level ID
    MainMenu,
}

impl Scene {
    fn to_obj<'a>(self, resources: &'a Resources) -> Box<Sceneable + 'a> {
        match self {
            Scene::Splash => SplashScene::new(resources),
            Scene::Level(id) => Level::new(id),
            Scene::MainMenu => MainMenu::new(resources),
        }
    }
}

#[derive(Clone)]
pub enum SceneAction {
    NoChange,
    Push(Scene),
    Change(Scene),
    Pop(u32), // u32 is number of times to pop
    Quit,
}

pub struct SceneManager<'a> {
    pub scenes: Vec<Box<Sceneable + 'a>>,
    pub should_exit: bool,
    //pub resources: &'a Resources,
}

impl<'a> SceneManager<'a> {
    pub fn new(initial_scene: Box<Sceneable + 'a>) -> SceneManager<'a> {
        SceneManager {
            scenes: vec![initial_scene],
            should_exit: false,
            //resources,
        }
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn handle_scene_action(&mut self, action: SceneAction, resources: &'a Resources) {
        match action {
            SceneAction::NoChange => {},

            SceneAction::Push(scene) => {
                self.scenes.push(scene.to_obj(resources));
            },

            SceneAction::Change(scene) => {
                *self.scenes.last_mut().unwrap() = scene.to_obj(resources);
            },

            SceneAction::Pop(n) => {
                for _i in 0..n {
                    self.scenes.pop();
                }
            },

            SceneAction::Quit => {
                self.should_exit = true;
            },
        };
    }

    pub fn update(&mut self, resources: &'a Resources) {
        let action = self.scenes.last_mut().unwrap().update(resources);
        self.handle_scene_action(action, resources);
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        for scene in self.scenes.iter() {
            scene.draw(window);
        }

        window.display();
    }

    pub fn handle_event(&mut self, event: Event, resources: &'a Resources) {
        let action = self.scenes.last_mut().unwrap().handle_event(event, resources);
        self.handle_scene_action(action, resources);
    }
}
