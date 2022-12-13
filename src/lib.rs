#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

use wasm4::*;

const MSG: &str = "Hello World";

static mut X: i16 = 0;
static mut Y: i16 = 0;

static mut XI: i8 = 1;
static mut YI: i8 = 1;

#[no_mangle]
fn update() {
    unsafe {
        if X + (MSG.len() as i16 * FONT_SIZE) == 159 {
            XI = -1;
        } else if X == 0 {
            XI = 1;
        }
        if Y + FONT_SIZE == 159 {
            YI = -1;
        } else if Y == 0 {
            YI = 1;
        }

        X += XI as i16;
        Y += YI as i16;

        *DRAW_COLORS = 3;

        text(MSG, X.into(), Y.into());
    }
}
