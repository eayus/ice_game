use sfml::graphics::RenderWindow;
use sfml::window::Event;

use resources::Resources;

pub type SceneStack = Vec<Box<Scene>>;

pub trait Scene {
    fn update(&mut self) -> SceneAction;
    fn draw(&self, window: &mut RenderWindow);
    fn handle_event(&mut self, event: Event, resources: &Resources) -> SceneAction;
}

pub enum SceneAction {
    NoChange,
    Push(Box<Scene>),
    Change(Box<Scene>),
    Pop(u32), // u32 is number of times to pop
    Quit,
}

pub struct SceneManager {
    scenes: SceneStack,
    should_exit: bool,
    resources: Resources,
}

impl SceneManager {
    pub fn new(initial_scene: Box<Scene>, resources: Resources) -> SceneManager {
        SceneManager {
            scenes: vec![initial_scene],
            should_exit: false,
            resources,
        }
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn handle_scene_action(&mut self, action: SceneAction) {
        match action {
            SceneAction::NoChange => {},

            SceneAction::Push(new_state) => {
                self.scenes.push(new_state);
            },

            SceneAction::Change(new_state) => {
                *self.scenes.last_mut().unwrap() = new_state;
            },

            SceneAction::Pop(n) => {
                for i in 0..n {
                    self.scenes.pop();
                }
            },

            SceneAction::Quit => {
                self.should_exit = true;
            },
        };
    }

    pub fn update(&mut self) {
        let action = self.scenes.last_mut().unwrap().update();
        self.handle_scene_action(action);
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        for scene in self.scenes.iter() {
            scene.draw(window);
        }

        window.display();
    }

    pub fn handle_event(&mut self, event: Event) {
        let action = self.scenes.last_mut().unwrap().handle_event(event, &self.resources);
        self.handle_scene_action(action);
    }
}
