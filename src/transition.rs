pub enum Easing {
    Linear,
}

pub enum Type {
    In,
    Out,
}

pub struct Transition {
    total_frames: u32,
    elapsed_frames: u32,
    running: bool,
    // Store function which maps the percent to opacity?
}

impl Transition {
    pub fn new(duration_in_frames: u32, easing: Easing, trans_type: Type) -> Transition {
        Transition {
            total_frames: duration_in_frames,
            elapsed_frames: 0,
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn update(&mut self) -> bool {
        if self.running {
            self.elapsed_frames += 1;
        }
        self.elapsed_frames >= self.total_frames
    }

    pub fn get_opacity(&self) -> u8 {
        use std::cmp;
        let mut percent = self.elapsed_frames as f64 / self.total_frames as f64;
        if percent > 1.0 {
            percent = 1.0;
        }
        ( 255.0 * (1.0 - percent) ) as u8
    }
}

fn in_linear(percent: f64) -> f64 {
    percent
}

fn out_linear(percent: f64) -> f64 {
    1.0 - percent
}
