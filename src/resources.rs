use std::collections::HashMap;
use sfml::graphics::Font;

// Maybe hold a struct for all the resources that need to be loaded at different times. In an
// Option<T>? so that they can be initialsed at a later date. Perhaps this is what futures are for?
pub struct Resources {
    pub menu_res: MenuResources,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            menu_res: MenuResources::new(),
        }
    }
}

struct MenuResources {
    pub raleway: Font,
}

impl MenuResources {
    fn new() -> MenuResources {
        MenuResources {
            raleway: Font::from_file("res/Raleway-Regular.ttf").unwrap(),
        }
    }
}
