use std::mem;
use std::slice;
use std::os::raw::c_void;

use font::FONT8X8;

mod font;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const IMAGE_LENGTH: usize = WIDTH as usize * HEIGHT as usize * 4;
static mut COLOR: u16 = 0;
static mut DIRECTION: bool = true;

static PALETTE: &[u8] = include_bytes!("gameland.pal");

// MCGA 320x200 image, where each color byte is an index into the RGB palette.
static PALETTE_BASED_IMAGE: &[u8] = include_bytes!("gameland.raw");
static mut IMAGE: [u8; IMAGE_LENGTH] = [0; IMAGE_LENGTH];

#[no_mangle]
pub unsafe fn prepare() {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let source_index = y * WIDTH + x;
            let target_index = (y * WIDTH + x) * 4;
            let palette_index = PALETTE_BASED_IMAGE[source_index] as usize * 3;

            IMAGE[target_index + 3] = 255;

            // The palette in the file is 0-255, but the upper two bits (which the VGA hardware ignores) contain garbage.
            // The algorithm works around that, and converts the 18-bit VGA palette entry to a 24-bit color.
            IMAGE[target_index + 0] = (PALETTE[palette_index + 0] & 63) * 4;
            IMAGE[target_index + 1] = (PALETTE[palette_index + 1] & 63) * 4;
            IMAGE[target_index + 2] = (PALETTE[palette_index + 2] & 63) * 4;
        }
    }
}

#[no_mangle]
pub fn clear(pointer: *mut u8, width: usize, height: usize) {
    let byte_size = width * height * 4;
    let buf = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    for i in buf.iter_mut() {
        *i = 0;
    }
}

#[no_mangle]
pub fn fill(pointer: *mut u8, width: usize, height: usize, mut frame: u32) -> u32 {
    // pixels are stored in RGBA, so each pixel is 4 bytes
    let byte_size = width * height * 4;
    let screen_buffer = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    let width = width as u16;

    unsafe {
        if DIRECTION {
            COLOR += 4;
        } else {
            COLOR -= 4;
        }

        if COLOR == 0 {
            DIRECTION = !DIRECTION;
        }
        else if COLOR == 128 {
            DIRECTION = !DIRECTION;
            COLOR -= 4; // avoid overflow
        }

        frame += 1;

        screen_buffer.copy_from_slice(&IMAGE);

        render_scrolltext(screen_buffer);

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let color = COLOR as u8;
                set_pixel(screen_buffer, width, x, y, color, color, color, 255);
            }
        }
    }

    frame
}

unsafe fn set_pixel(screen_buffer: &mut [u8], width: u16, x: usize, y: usize,
                    r: u8, g: u8, b: u8, alpha: u8) {
    let width = width as usize;
    let offset = x * 4 + y * 4 * width;

    // We only overwrite the background pixels, ie #000000 and also other very dark colors (since the image data
    // contains some dark/almost black pixels at the edges of the image)
    if screen_buffer[offset + 0] > 60 ||
       screen_buffer[offset + 1] > 60 ||
       screen_buffer[offset + 2] > 60 {
        return;
    }

    screen_buffer[(offset + 0)] = r;
    screen_buffer[(offset + 1)] = g;
    screen_buffer[(offset + 2)] = b;
    screen_buffer[(offset + 3)] = alpha;
}

static mut TEXT_POSITION: isize = WIDTH as isize;
static TEXT_BASE_ROW: usize = HEIGHT - 8 * 2;
static SCROLL_TEXT: &str = "\
this is a webassembly/rust retrofit of an age-old intro made in a \
completely different age of computing. hardware capacity was limited, but \
a lot of great people were doing their best to squeeze the most out of the \
machines. greetings goes to jojo, bagder, hal and javax. obsolete text \
from the original intro will follow.   gameland! our favourite party is \
alive and kicking butt. please don't be afraid to join the \
community.                      \
this intro was coded by plundis. the music was made by daddy freddy, big \
hugs and smiles to you!           still not convinced? visit our \
homepage: www.gameland.eu.org";

// Insanely poor algorithm; we essentially redraw the whole visible scroll text on each frame. Some obvious ways to
// make it better:
//
// - Copy column 1-END of each affected line to 0-(END - 1), and only render the right-most column.
// - Perhaps even more efficient: pre-render the whole scroll text into a separate buffer, and then just copy the relevant part on each frame.
//
// Sadly, I feel more comfortable in C (memcpy & friends) on these kind of things than in Rust, but I will definitely be
// incredibly happy to receive PRs that improves this!
unsafe fn render_scrolltext(screen_buffer: &mut [u8])
{
    let scroll_text = SCROLL_TEXT.as_bytes();

    TEXT_POSITION -= 1;
    if TEXT_POSITION < -((scroll_text.len() * 16) as isize) {
        TEXT_POSITION = WIDTH as isize;
    }

    // The 8x8 font being used is extrapolated to 16x16 by adding black pixels between each row and column (the * 2) in
    // the algorithm below.
    for i in 0..scroll_text.len() {
        let character_bitmap = FONT8X8[scroll_text[i] as usize];
        for bitmap_x in 0..8 {
            let x = TEXT_POSITION + (16 * i as isize) + (bitmap_x * 2);
            if x < 0 || x >= WIDTH as isize {
                // This column is outside the visible area, so nothing more needs to be done - can move to the next column
                // right away.
                continue;
            }

            for bitmap_y in 0..8 {
                let pixel_set = (character_bitmap[bitmap_y] & (1 << bitmap_x)) != 0;
                let color = if pixel_set {
                    192
                } else {
                    0
                } as u8;
                set_pixel(screen_buffer, WIDTH as u16,
                          x as usize, TEXT_BASE_ROW + (bitmap_y * 2),
                          color, color, color, 255);
            }
        }
    }
}
