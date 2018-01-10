use std::collections::VecDeque;

use level::{Direction, Tile, TileMap};

use sfml::system::{Vector2, Vector2f, Vector2i};
use sfml::graphics::{Color, RenderWindow, RectangleShape, Shape, Transformable};

#[derive(Debug)]
pub enum Action {
    Move { dir: Direction, steps: u32 },
    Teleport(Vector2i),
}

pub struct Player {
    action_queue: VecDeque<Action>,
    action_counter: u32,
    window_pos: Vector2f,
    map_pos: Vector2i,
}

impl Player {

    const SPEED: f32 = 4.0;

    const COLOR: Color = Color {
        r: 78,
        g: 204,
        b: 195,
        a: 255,
    };

    pub fn new(starting_position: Vector2i) -> Player {
        Player {
            action_queue: VecDeque::new(),
            window_pos: Vector2::new(starting_position.x as f32 * 65.0, starting_position.y as f32 * 65.0),
            action_counter: 0,
            map_pos: starting_position,
        }
    }

    pub fn update(&mut self) {

        if let Some(action) = self.action_queue.front() {

            match *action {
                Action::Move { dir, steps } => {

                    if self.action_counter == 0 {
                        self.action_counter = (steps as f32 * 65.0 / Self::SPEED) as u32;
                    }

                    let unit_vec = dir.get_unit_vec();

                    self.window_pos.x += unit_vec.x as f32 * Self::SPEED;
                    self.window_pos.y += unit_vec.y as f32 * Self::SPEED;

                    self.action_counter -= 1;
                },
                Action::Teleport(pos) => {
                    self.window_pos.x = pos.x as f32 * 65.0;
                    self.window_pos.y = pos.y as f32 * 65.0;
                },
            }

        }

        if !self.action_queue.is_empty() && self.action_counter == 0 {
            self.action_queue.pop_front();
        }

        if self.action_queue.is_empty() {
            self.window_pos.x = self.map_pos.x as f32 * 65.0;
            self.window_pos.y = self.map_pos.y as f32 * 65.0;
        }

    }

    pub fn draw(&self, window: &RenderWindow) {

        // TODO: Store the rect instead of position?
        let rect = RectangleShape::with_size(Vector2::new(64.0, 64.0));
        rect.set_position((self.window_pos.x, self.window_pos.y));
        rect.set_fill_color(&Self::COLOR);

    }

    fn add_action(&mut self, action: Action) {
        self.action_queue.push_back(action);
    }

    fn is_ready(&self) -> bool {
        self.action_queue.is_empty()
    }

    pub fn set_direction(&mut self, dir: Direction, tile_map: &TileMap) {

        if !self.is_ready() {
            return;
        }

        let unit_vec = dir.get_unit_vec();

        let mut count: u32 = 0;

        loop {
            self.map_pos += unit_vec;

            match tile_map.get_tile_at(self.map_pos) {

                Tile::Wall => {
                    if count != 0 {
                        self.add_action(Action::Move{ dir, steps: count });
                    }

                    self.map_pos -= unit_vec;

                    break;

                },

                Tile::OneWay(allowed_dir) => {
                    if dir == allowed_dir {
                        count += 1;
                    } else {
                        if count != 0 {
                            self.add_action(Action::Move{ dir, steps: count });
                        }

                        self.map_pos -= unit_vec;

                        break;
                    }
                },

                Tile::Teleporter(index) => {
                    // If two teleporters are placed in line of sight, they keep passing you between them (and hence freeze the game). Perhaps make it so you can pass through a teleporter once on each move only.

                    let map_pos = self.map_pos;
                    let is_matching_teleporter = move |tile: &Tile, pos: Vector2i| -> bool {
                        if let Tile::Teleporter(other_index) = *tile {
                            return other_index == index && pos != map_pos;
                        }
                        false
                    };

                    self.map_pos = tile_map.get_tile(is_matching_teleporter).expect("No matching teleporter!");

                    self.add_action(Action::Move{ dir, steps: count + 1} );
                    count = 0;

                    let map_pos = self.map_pos;
                    self.add_action(Action::Teleport(map_pos));

                },

                Tile::Target => {
                    self.add_action(Action::Move{ dir, steps: count + 1 });
                    break;
                },

                _=> {
                    count += 1;
                },

            }

        }

    }

}
