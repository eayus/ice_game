extern crate num;
use self::num::{FromPrimitive, ToPrimitive, clamp};

pub enum Delay {
    Post(u32),
    Pre(u32),
    None,
}

impl Delay {
    fn num_frames(&self) -> u32 {
        match *self {
            Delay::Post(frames) => frames,
            Delay::Pre(frames) => frames,
            Delay::None => 0,
        }
    }
}

pub struct Transition<T>
    where T: FromPrimitive + ToPrimitive
{
    from: T,
    to: T,
    total_frames: u32,
    elapsed_frames: u32,
    running: bool,
    delay: Delay,
}

impl<T> Transition<T>
    where T: FromPrimitive + ToPrimitive + Clone
{
    pub fn new(from: T, to: T, duration_in_frames: u32, delay: Delay) -> Transition<T> {
        Transition {
            from,
            to,
            total_frames: duration_in_frames + delay.num_frames(),
            elapsed_frames: 0,
            running: false,
            delay,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn update(&mut self) -> bool {
        if self.running {
            self.elapsed_frames += 1;
        }
        self.elapsed_frames > self.total_frames
    }

    pub fn get_val(&self) -> T {
        let from = self.from.to_f64().unwrap();
        let to = self.to.to_f64().unwrap();
        let elapsed = self.elapsed_frames as f64;
        let total = self.total_frames as f64;

        let val = match self.delay {
            Delay::Post(frames) => lerp_post_delay(from, to, elapsed / total, frames as f64 / total),
            Delay::Pre(frames) => lerp_pre_delay(from, to, elapsed / total, frames as f64 / total),
            Delay::None => lerp_no_delay(from, to, elapsed / total),
        };

        T::from_f64(val).unwrap_or(self.from.clone())
    }
}

// Const generic?
fn lerp_no_delay(from: f64, to: f64, perc: f64) -> f64 {
    from + ((to - from) * perc)
}

fn lerp_post_delay(from: f64, to: f64, perc: f64, delay_perc: f64) -> f64 {
    let new_perc = clamp(perc / (1.0 - delay_perc), 0.0, 1.0);
    from + ((to - from) * new_perc)
}

fn lerp_pre_delay(from: f64, to: f64, perc: f64, delay_perc: f64) -> f64 {
    let new_perc = clamp(delay_perc + (perc / (1.0 - delay_perc)), 0.0, 1.0);
    from + ((to - from) * new_perc)
}



