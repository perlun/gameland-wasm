use std::mem;
use std::slice;
use std::os::raw::c_void;

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
    let buf = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

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
        // if frame % 10 != 0 {
        //     return frame;
        // }

        buf.copy_from_slice(&IMAGE);

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                set_pixel(buf, width, x, y);
            }
        }
    }

    frame
}

unsafe fn set_pixel(pixels: &mut [u8], width: u16, x: usize, y: usize) {
    let width = width as usize;
    let offset = x * 4 + y * 4 * width;

    // We only overwrite the background pixels, ie #000000 and also other very dark colors (since the image data
    // contains some dark/almost black pixels at the edges of the image)
    if pixels[offset + 0] > 60 ||
       pixels[offset + 1] > 60 ||
       pixels[offset + 2] > 60 {
        return;
    }

    pixels[(offset + 0)] = COLOR as u8;
    pixels[(offset + 1)] = COLOR as u8;
    pixels[(offset + 2)] = COLOR as u8;
    pixels[(offset + 3)] = 255;
}
