pub const SCREEN_HEIGHT: usize = 30;
pub const SCREEN_WIDTH: usize = 60;

pub struct Screen {
    data: [[i32; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            data: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT]
        }
    }
}