use crate::cpu::SCREEN_HEIGHT;
use crate::cpu::SCREEN_WIDTH;
use crate::Error;

pub struct Display {
    pub vram: [u8; SCREEN_HEIGHT * SCREEN_WIDTH * 4],
}

impl Display {
    pub fn clear(&mut self) {
        self.vram = [0x00; SCREEN_HEIGHT * SCREEN_WIDTH * 4];
    }
    // fixme: support alpha channel
    pub unsafe fn set_pixel_greyscale(&mut self, pos: usize, val: u8) -> bool {
        // set to unsafe as it does not do any bounds checking
        let mut toggled = false;
        self.vram
            .iter_mut()
            .skip(pos * 4)
            .take(3)
            .for_each(|pixel| {
                let prev_val = *pixel;
                *pixel ^= val;
                if *pixel != prev_val {
                    toggled = true
                }
            });
        self.vram
            .iter_mut()
            .skip(pos * 4 + 3)
            .take(1)
            .for_each(|pixel| *pixel = 0xFF);

        toggled
    }
    pub fn set_pixel(&mut self, x: usize, y: usize) -> Result<bool, Error> {
        let toggled;

        if x < 64 && y < 32 {
            toggled = unsafe { self.set_pixel_greyscale(x + (SCREEN_WIDTH * y), 0xFF) };
        } else if x >= 64 {
            return Err(Error::InvalidValue(format!(
                "Value x is too large ({} is > 63)",
                x
            )));
        } else if y >= 32 {
            return Err(Error::InvalidValue(format!(
                "Value y is too large ({} is > 31)",
                y
            )));
        } else {
            return Err(Error::InvalidValue(
                "An unknown error has occurred!".to_string(),
            ));
        }
        Ok(toggled)
    }
}
