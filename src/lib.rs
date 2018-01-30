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

static mut COLOR: u16 = 0;
static mut DIRECTION: bool = true;

unsafe fn set_pixel(pixels: &mut [u8], width: u16, x: u16, y: u16) {
    let x = x as usize;
    let y = y as usize;
    let width = width as usize;
    let offset = x * 4 + y * 4 * width;

    pixels[(offset + 0)] = COLOR as u8;
    pixels[(offset + 1)] = COLOR as u8;
    pixels[(offset + 2)] = COLOR as u8;
    pixels[(offset + 3)] = 255;
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
            COLOR += 32;
        } else {
            COLOR -= 32;
        }

        if COLOR == 0 {
            DIRECTION = !DIRECTION;
        }
        else if COLOR == 256 {
            DIRECTION = !DIRECTION;
            COLOR -= 32; // avoid overflow
        }

        frame += 1;
        // if frame % 10 != 0 {
        //     return frame;
        // }

        for x in 0..320 {
            for y in 0..200 {
                set_pixel(buf, width, x, y);
            }
        }
    }

    frame
}
