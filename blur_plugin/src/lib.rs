use std::ffi::c_char;

#[no_mangle]
pub extern "C" fn process_image(
    _width: u32,
    _height: u32,
    _rgba_data: *mut u8,
    _params: *const c_char,
) {
    // No-op stub
}
