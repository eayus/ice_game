extern crate sfml;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, Event, Key, Style};
use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::graphics::Font;

mod scene;
use scene::{SceneManager, Scene, SceneStack, SceneAction};

mod resources;
use resources::Resources;

mod menu;
use menu::MainMenu;

mod level;

const WINDOW_WIDTH: u32 = 960;
const WINDOW_HEIGHT: u32 = 640;

/*const COLOR_SCHEME: [Color; 4] = [
    Color { r: 0.33, g: 0.38, b: 0.4375, a: 1.0 },
    Color { r: 0.777, g: 0.953, b: 0.391, a: 1.0 },
    Color { r: 1.0, g: 0.418, b: 0.418, a: 1.0 },
    Color { r: 0.766, g: 0.301, b: 0.344, a: 1.0 },
];*/

fn main() {

    /*let mut cb = ContextBuilder::new("ice-game", "Eayus")
        .window_setup(conf::WindowSetup::default().title("Ice Puzzle Game"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));


    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        cb = cb.add_resource_path(path);
    }

    let ctx = &mut cb.build().unwrap();

    load_resources(ctx);

    graphics::set_background_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0));


    let state = &mut StateManager::new(Box::new(menu::MainMenu::new(ctx)));
    event::run(ctx, state).unwrap();*/


    let mut window = RenderWindow::new(VideoMode::new(WINDOW_WIDTH, WINDOW_HEIGHT, 32), "Ice Puzzle Game", Style::CLOSE, &ContextSettings::default());
    window.set_vertical_sync_enabled(true);

    let mut res = Resources::new();

    let mut scene_manager = SceneManager::new(SplashScene::new(), res);

    while !scene_manager.should_exit() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                _ => scene_manager.handle_event(event),
            };
        }

        scene_manager.update();
        scene_manager.draw(&mut window);
    }

}


/*struct LoadingScene {
    rx: mpsc::Receiver<Resources>,
    loading_thread: thread::JoinHandle<()>,
}

impl LoadingScene {
    fn new() -> Box<LoadingScene> {
        let (tx, rx) = mpsc::channel();

        let loading_thread = thread::spawn(move || {
            let r = Resources::new();
            r.fonts.insert("raleway", Font::from_file("res/Raleway-Regular.ttf").unwrap());
            tx.send(r).unwrap();
        });

        Box::new(LoadingScene {
            rx,
            loading_thread
        })
    }
}

impl Scene for LoadingScene {
    fn update(&mut self) -> SceneAction {
        if let Ok(res) = self.rx.try_recv() {
            println!("{:#?}", res);
            self.loading_thread.join();
            SceneAction::Quit
        } else {
            SceneAction::NoChange
        }
    }

    fn draw(&self, window: &mut RenderWindow) {
        window.clear(&Color::rgb(0, 200, 200));
    }

    fn handle_event(&mut self, event: Event, resources: &Resources) -> SceneAction {
        if let Event::KeyPressed { code: Key::F, .. } = event {
            SceneAction::Push(Box::new(AnotherScene))
        } else {
            SceneAction::NoChange
        }
    }
}
*/
struct SplashScene;

impl SplashScene {
    fn new() -> Box<SplashScene> {
        Box::new(SplashScene)
    }
}

impl Scene for SplashScene {
    fn update(&mut self) -> SceneAction {
        SceneAction::NoChange
    }

    fn draw(&self, window: &mut RenderWindow) {
        window.clear(&Color::rgb(200, 0, 200));
    }

    fn handle_event<'a>(&mut self, event: Event, resources: &'a Resources) -> SceneAction {
        if let Event::KeyPressed { code: Key::Escape, .. } = event {
            SceneAction::Change(MainMenu::new(resources))
        } else {
            SceneAction::NoChange
        }
    }
}
