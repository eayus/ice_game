mod player;
use self::player::Player;

use resources::Resources;

use scene::{Scene, SceneAction, Sceneable};

use sfml::window::{Event, Key};
use sfml::system::{Vector2i, Vector2f, Vector2};
use sfml::graphics::{Shape, RectangleShape, RenderWindow, ConvexShape, Color, Transformable, RenderTarget};

#[derive(Copy,Clone,PartialEq)]
pub enum Tile {
    Wall,
    Teleporter(u32), // Index of the teleporter. Teleporters with same index will be linked.
    Empty,
    Start,
    OneWay(Direction),
    Target,
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone)]
pub struct TileMap {
    tiles: [[Tile; 8]; 5]
}

// Change this to a const-generic when it becomes a feature.
pub struct Level {
    player: Player,
    map_id: usize,
}

impl Tile {

    const EMPTY_COLOR: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    const WALL_COLOR: Color = Color { r: 84, g: 97, b: 112, a: 255 };
    const TARGET_COLOR: Color = Color { r: 198, g: 243, b: 100, a: 255 };
    const TELEPORTER_COLOR: Color = Color { r: 255, g: 107, b: 107, a: 255 };
    const ONEWAY_COLOR: Color = Color { r: 195, g: 77, b: 88, a: 255 };

    fn draw(&self, window: &mut RenderWindow, position: Vector2f) {

        let mut rect = RectangleShape::with_size(Vector2::new(64.0, 64.0));
        rect.set_position(position);

        let color = match *self {
            Tile::Wall => Self::WALL_COLOR,
            Tile::Target => Self::TARGET_COLOR,
            Tile::Teleporter(_) => {
                // TODO: Change to loop
                rect.set_fill_color(&Self::TELEPORTER_COLOR);
                window.draw(&rect);

                rect.set_fill_color(&Self::EMPTY_COLOR);
                rect.set_size(Vector2::new(52.0, 52.0));
                rect.move_((6.0, 6.0));
                window.draw(&rect);
                
                rect.set_fill_color(&Self::TELEPORTER_COLOR);
                rect.set_size(Vector2::new(40.0, 40.0));
                rect.move_((6.0, 6.0));
                window.draw(&rect);
                
                rect.set_fill_color(&Self::EMPTY_COLOR);
                rect.set_size(Vector2::new(28.0, 28.0));
                rect.move_((6.0, 6.0));
                window.draw(&rect);
                
                rect.set_fill_color(&Self::TELEPORTER_COLOR);
                rect.set_size(Vector2::new(16.0, 16.0));
                rect.move_((6.0, 6.0));
                window.draw(&rect);

                rect.set_fill_color(&Self::EMPTY_COLOR);
                rect.set_size(Vector2::new(4.0, 4.0));
                rect.move_((6.0, 6.0));
                window.draw(&rect);
                
                return;
            },
            Tile::OneWay(dir) => {
                let rot = dir.get_rot();

                let mut triangle = ConvexShape::new(3);

                triangle.set_point(0, (32.0, 8.0));
                triangle.set_point(1, (8.0, 56.0));
                triangle.set_point(2, (56.0, 56.0));

                triangle.set_origin((32.0, 32.0));
                triangle.rotate(rot as f32);

                triangle.set_fill_color(&Self::ONEWAY_COLOR);

                triangle.move_((position.x + 32.0, position.y + 32.0));

                window.draw(&triangle);

                return;
            },
            _ => return,
        };

        rect.set_fill_color(&color);
        window.draw(&rect);

    }

}

impl Direction {

    fn get_unit_vec(&self) -> Vector2i {
        match *self {
            Direction::Left  => Vector2::new(-1, 0),
            Direction::Right => Vector2::new( 1, 0),
            Direction::Up    => Vector2::new( 0,-1),
            Direction::Down  => Vector2::new( 0, 1),
        }
    }

    fn get_rot(&self) -> f64 {
        match *self {
            Direction::Left  => 270.0,
            Direction::Right => 90.0,
            Direction::Up    => 0.0,
            Direction::Down  => 180.0,
        }
    }

}

impl TileMap {

    // Maps
    pub const MAPS: [TileMap; 3] = [
        TileMap {
            tiles: [
                [Tile::Start, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty],
                [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
                [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
                [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Target, Tile::Empty, Tile::Empty, Tile::Empty],
                [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            ],
        },

        TileMap {
            tiles: [
                [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Start, Tile::Empty, Tile::OneWay(Direction::Right), Tile::Empty, Tile::Empty],
                [Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
                [Tile::Empty, Tile::Empty, Tile::Target, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall, Tile::Empty],
                [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
                [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            ],
        },

        TileMap {
            tiles: [
                [Tile::Empty, Tile::Empty, Tile::Wall, Tile::Start, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
                [Tile::Teleporter(0), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
                [Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
                [Tile::Empty, Tile::Empty, Tile::Target, Tile::Teleporter(0), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
                [Tile::Empty, Tile::Empty, Tile::OneWay(Direction::Right), Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
            ],
        },

    ];

    const GRID_LINE_COLOR: Color = Color {
        r: 128,
        g: 128,
        b: 128,
        a: 64,
    };

    pub fn new(map_id: usize) -> TileMap {
        Self::MAPS[map_id].clone()
    }

    fn draw(&self, window: &mut RenderWindow, position: Vector2f) {

        for (y, &row) in self.tiles.iter().enumerate() {

            for (x, &tile) in row.iter().enumerate() {

                tile.draw(window, Vector2::new( (x * 65) as f32 + position.x, (y * 65) as f32 + position.y))

            }

        }

        let mut rect = RectangleShape::new();
        rect.set_fill_color(&Self::GRID_LINE_COLOR);

        for y in 1..self.tiles.len() {
            rect.set_size((self.tiles[0].len() as f32 * 65.0, 1.0));
            rect.set_position((position.x, (y * 65) as f32 - 1.0 + position.y));

            window.draw(&rect);
        }

        for x in 1..self.tiles[0].len() {
            rect.set_size((1.0, self.tiles.len() as f32 * 65.0));
            rect.set_position(((x * 65) as f32 - 1.0 + position.x, position.y));

            window.draw(&rect);
        }

    }

    fn get_tile<F>(&self, is_tile: F) -> Option<Vector2i>
        where F: Fn(&Tile, Vector2i) -> bool {

        for (y, &row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {

                let tile_pos = Vector2::new(x as i32, y as i32);

                if is_tile(&tile, tile_pos) {
                    return Some(tile_pos);
                }

            }
        }

        None

    }

    // TODO: Add bounds checking
    fn get_tile_at(&self, pos: Vector2i) -> Tile {
        let x = pos.x as usize;
        let y = pos.y as usize;

        if let Some(row) = self.tiles.get(y) {
            if let Some(tile) = row.get(x) {
                return *tile;
            }
        }

        Tile::Wall
    }

}

// TODO: Unhardocde 32
impl Level {

    const ROUNDED_BG_COLOR: Color = Color::WHITE;

    const BG_COLOR: Color = Color {
        r: 242,
        g: 242,
        b: 242,
        a: 255,
    };

    pub fn new(level_id: usize) -> Box<Level> {

        let tile_map = &TileMap::MAPS[level_id];

        let player_pos = tile_map.get_tile(|tile: &Tile, _pos: Vector2i| -> bool {
            *tile == Tile::Start
        }).expect("There is no start tile!");

        Box::new(Level {
            player: Player::new(player_pos),
            map_id: level_id,
        })

    }

    fn next_map(&mut self) {
        self.map_id += 1;

        let player_pos = TileMap::MAPS[self.map_id].get_tile(|tile: &Tile, _pos: Vector2i| -> bool {
            *tile == Tile::Start
        }).expect("There is no start tile!");

        self.player = Player::new(player_pos);
    }

    pub fn move_player(&mut self, dir: Direction) {

        self.player.set_direction(dir, /*&self.tile_map*/ &TileMap::MAPS[self.map_id]);

    }


}

impl Sceneable for Level {

    fn update(&mut self, _resources: &Resources) -> SceneAction {
        self.player.update();

        if self.player.reached_target() {
            self.next_map();
        }

        SceneAction::NoChange
    }

    fn draw(&self, window: &mut RenderWindow) {
        window.clear(&Self::BG_COLOR);

        let map_width = TileMap::MAPS[self.map_id].tiles[0].len() as f32 * 65.0;
        let map_height = TileMap::MAPS[self.map_id].tiles.len() as f32 * 65.0;

        // TODO: Remove hard-coded screen resolution.
        let map_x = (960.0 - map_width) / 2.0;
        let map_y = (640.0 - map_height) / 2.0;

        // TODO: Use a Transform rather than passing a poition, and use trans.tranform_point on the points.

        let mut rect = RectangleShape::with_size(Vector2::new(map_width + 65.0, map_height + 65.0));
        rect.set_position(Vector2::new(map_x - 32.0, map_y - 32.0));
        rect.set_fill_color(&Self::ROUNDED_BG_COLOR);

        window.draw(&rect);

        TileMap::MAPS[self.map_id].draw(window, Vector2::new(map_x, map_y));

        //self.tile_map.draw(window, Vector2::new(map_x, map_y));
        self.player.draw(window, Vector2::new(map_x, map_y));

    }

    fn handle_event(&mut self, event: Event, resources: &Resources) -> SceneAction {
        match event {
            Event::KeyPressed { code, .. } => match code {
                Key::A => self.move_player(Direction::Left),
                Key::D => self.move_player(Direction::Right),
                Key::W => self.move_player(Direction::Up),
                Key::S => self.move_player(Direction::Down),

                Key::F => return SceneAction::Change(Scene::MainMenu),
                _ => {},
            },
            _ => {},
        };
        SceneAction::NoChange
    }
}
