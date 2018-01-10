use std::cmp;
use ::{WINDOW_HEIGHT, WINDOW_WIDTH};

use resources::Resources;

use scene::{SceneAction, Scene};

use sfml::window::{Event, Key};
use sfml::graphics::{Text, Color, RenderWindow, RenderTarget, Transformable};

use level::Level;

// Play
// Credits
// Exit

pub struct MainMenu<'a> {
    title_text: Text<'a>,
    menu_items: [MenuItem<'a>; 3],
    current_item: usize,
}

impl<'a> MainMenu<'a> {
    const BG_COLOR: Color = Color {
        r: 240,
        g: 240,
        b: 240,
        a: 255,
    };

    const TEXT_COLOR: Color = Color {
        r: 25,
        g: 25,
        b: 25,
        a: 255,
    };

    pub fn new(res: &'a Resources) -> Box<MainMenu<'a>> {
        let title_text = Text::new("Ice Game", &res.menu_res.raleway, 36);

        let play_text = MenuItem::new(
            Text::new("Play", &res.menu_res.raleway, 28),
            Box::new(move || {
                SceneAction::Change(Box::new(Level::new(0)))
            //StateAction::Change(Box::new(LevelSelect { font: tmp_font.clone() }))
            })
        );
        let credits_text = MenuItem::new(
            Text::new("Credits", &res.menu_res.raleway, 28),
            Box::new(|| {
                SceneAction::NoChange
            })
        );
        let exit_text = MenuItem::new(
            Text::new("Quit", &res.menu_res.raleway, 28),
            Box::new(|| {
                SceneAction::Quit
            })
        );
        
        let menu: Box<MainMenu + 'a> = Box::new(MainMenu {
            title_text,
            menu_items: [play_text, credits_text, exit_text],
            current_item: 0,
        });

        menu.title_text.set_fill_color(&Self::TEXT_COLOR);
        menu.title_text.set_position((100.0, 100.0));

        for (index, menu_item) in menu.menu_items.iter().enumerate() {
            menu_item.text.set_position((300.0, 300.0 + (index * 100) as f32));
        }

        menu
    }
}

impl<'a> Scene for MainMenu<'a> {
    fn update(&mut self) -> SceneAction {
        SceneAction::NoChange
    }

    fn draw(&self, window: &mut RenderWindow) {
        window.clear(&Self::BG_COLOR);

        window.draw(&self.title_text);

        for (index, menu_item) in self.menu_items.iter().enumerate() {

            if index == self.current_item {
                // TODO: Change to a constants e.g. TEXT_SELECTED_COLOR
                menu_item.text.set_fill_color(&Color::rgba(255, 0, 0, 255));
            } else {
                menu_item.text.set_fill_color(&Self::TEXT_COLOR);
            }

            window.draw(&menu_item.text);

        }

    }

    fn handle_event(&mut self, event: Event, res: &Resources) -> SceneAction {

        if let Event::KeyPressed { code, .. } = event{
            match code {
                Key::Up => {
                    if self.current_item != 0 {
                        self.current_item -= 1;
                    }
                },

                Key::Down => {
                    self.current_item = cmp::min(self.current_item + 1, self.menu_items.len() - 1);
                },

                Key::Return => {
                    return (self.menu_items.get(self.current_item).unwrap().on_click)();
                }

                _ => {},
            }
        }
        SceneAction::NoChange
    }
}


struct MenuItem<'a> {
    text: Text<'a>,
    on_click: Box<Fn() -> SceneAction>,
}

impl<'a> MenuItem<'a> {
    fn new(text: Text, on_click: Box<Fn() -> SceneAction>) -> MenuItem
    {
        MenuItem {
            text,
            on_click,
        }
    }
}

/*struct LevelSelect {
    font: Font,
}

impl LevelSelect {
    const BUTTON_SIZE: f32 = WINDOW_HEIGHT as f32 / 4.0;
    const BUTTON_COLOR: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    const BG_COLOR: Color = Color { r: 0.95, g: 0.95, b: 0.95, a: 0.95 };
}

impl State for LevelSelect {

    fn update(&mut self) -> StateAction {
        StateAction::NoChange
    }

    fn draw(&self, ctx: &mut Context) {
        graphics::set_background_color(ctx, Self::BG_COLOR);
        graphics::clear(ctx);

        use ::COLOR_SCHEME;

        const WIN_HEIGHTF: f32 = WINDOW_HEIGHT as f32;
        const WIN_WIDTHF: f32 = WINDOW_WIDTH as f32;

        for level_id in 0..(::level::TileMap::MAPS.len()) {
            let rect = graphics::Rect::new((Self::BUTTON_SIZE / 2.0) + (Self::BUTTON_SIZE * 1.5 * level_id as f32), (WIN_HEIGHTF - Self::BUTTON_SIZE) / 2.0, Self::BUTTON_SIZE, Self::BUTTON_SIZE);
            graphics::set_color(ctx, COLOR_SCHEME[level_id % COLOR_SCHEME.len()]);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, rect);

            let text = Text::new(ctx, &level_id.to_string(), &self.font).unwrap();
            graphics::set_color(ctx, MainMenu::TEXT_COLOR);
            graphics::draw(ctx, &text, Point2::new(rect.x + (rect.w / 2.0), rect.y + (rect.h / 2.0)), 0.0);
        }
    }

    fn handle_input(&mut self, ctx: &mut Context, input_event: InputEvent) -> StateAction {
        if let InputEvent::KeyDown(key) = input_event {
            match key {
                Keycode::Return => StateAction::Change(Box::new(::level::Level::new(0))),
                _ => StateAction::NoChange,
            }
        } else {
            StateAction::NoChange
        }
    }

}*/