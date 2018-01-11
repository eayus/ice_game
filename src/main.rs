extern crate sfml;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, Event, Key, Style};
use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::graphics::{Font, Text};

mod scene;
use scene::{SceneManager, Sceneable, SceneAction, Scene};

mod resources;
use resources::Resources;

mod menu;
use menu::MainMenu;

mod level;

// TODO: Change to float
const WINDOW_WIDTH: u32 = 960;
const WINDOW_HEIGHT: u32 = 640;

const BG_COLOR: Color = Color { r: 240, g: 240, b: 240, a: 255 };
const TEXT_COLOR: Color = Color { r: 50, g: 50, b: 50, a: 255 };

fn main() {

    let mut window = RenderWindow::new(VideoMode::new(WINDOW_WIDTH, WINDOW_HEIGHT, 32), "Ice Puzzle Game", Style::CLOSE, &ContextSettings::default());
    window.set_vertical_sync_enabled(true);

    let mut resources = Resources::new();

    let mut scene_manager = SceneManager::new(SplashScene::new(&resources));

    while !scene_manager.should_exit() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                _ => scene_manager.handle_event(event, &resources),
            };
        }

        scene_manager.update(&resources);
        scene_manager.draw(&mut window);
    }

}

struct SplashScene<'a> {
    text: Text<'a>,
    trans: Transition,
}

impl<'a> SplashScene<'a> {
    fn new(resources: &Resources) -> Box<SplashScene> {
        Box::new(SplashScene {
            text: {
                let mut t = Text::new("Ice Puzzle", &resources.menu_res.raleway, 50);
                let text_bounds = t.local_bounds();
                t.set_position(( (WINDOW_WIDTH as f32 - text_bounds.width) / 2.0, (WINDOW_HEIGHT as f32 - text_bounds.height - 50.0) / 2.0));
                t.set_fill_color(&TEXT_COLOR);
                t
            },
            trans: Transition::new(60, Easing::Linear, Type::Out),
        })
    }
}

impl<'a> Sceneable for SplashScene<'a> {
    fn update(&mut self, _res: &Resources) -> SceneAction {
        let mut color = TEXT_COLOR;
        color.a = self.trans.get_opacity();
        self.text.set_fill_color(&color);

        if self.trans.update() {
            SceneAction::Change(Scene::MainMenu)
        } else {
            SceneAction::NoChange
        }
    }

    fn draw(&self, window: &mut RenderWindow) {
        window.clear(&BG_COLOR);
        window.draw(&self.text);
    }

    fn handle_event(&mut self, event: Event, resources: &Resources) -> SceneAction {
        if let Event::KeyPressed { code: Key::Return, .. } = event {
            //SceneAction::Change(Scene::MainMenu);
            //SceneAction::NoChange
            self.trans.start();
        }
        SceneAction::NoChange
    }
}

enum Easing {
    Linear,
}

enum Type {
    In,
    Out,
}

struct Transition {
    total_frames: u32,
    elapsed_frames: u32,
    running: bool,
    // Store function which maps the percent to opacity?
}

impl Transition {
    fn new(duration_in_frames: u32, easing: Easing, trans_type: Type) -> Transition {
        Transition {
            total_frames: duration_in_frames,
            elapsed_frames: 0,
            running: false,
        }
    }

    fn start(&mut self) {
        self.running = true;
    }

    fn update(&mut self) -> bool {
        if self.running {
            self.elapsed_frames += 1;
        }
        self.elapsed_frames >= self.total_frames
    }

    fn get_opacity(&self) -> u8 {
        use std::cmp;
        let mut percent = self.elapsed_frames as f64 / self.total_frames as f64;
        if percent > 1.0 {
            percent = 1.0;
        }
        ( 255.0 * (1.0 - percent) ) as u8
    }
}