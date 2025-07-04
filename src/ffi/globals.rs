// FFI Globals untuk Hafiz Assistant Backend
// Global variables dan constants untuk FFI

/// Versi library
pub const LIBRARY_VERSION: &str = "1.0.0";

/// Nama library
pub const LIBRARY_NAME: &str = "Hafiz Assistant Backend";

/// Mendapatkan versi library
#[no_mangle]
pub extern "C" fn get_library_version() -> *const std::ffi::c_char {
    LIBRARY_VERSION.as_ptr() as *const std::ffi::c_char
}

/// Mendapatkan nama library
#[no_mangle]
pub extern "C" fn get_library_name() -> *const std::ffi::c_char {
    LIBRARY_NAME.as_ptr() as *const std::ffi::c_char
}

/// Status inisialisasi global
static mut IS_INITIALIZED: bool = false;

/// Cek apakah library sudah diinisialisasi
#[no_mangle]
pub extern "C" fn is_initialized() -> bool {
    unsafe { IS_INITIALIZED }
}

/// Set status inisialisasi
pub fn set_initialized(status: bool) {
    unsafe {
        IS_INITIALIZED = status;
    }
}
