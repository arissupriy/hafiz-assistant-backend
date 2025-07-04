// FFI Module untuk Hafiz Assistant Backend
// Menyediakan interface untuk integrasi dengan bahasa lain

pub mod structures;
pub mod functions;
pub mod globals;

// Re-export public types and functions
pub use structures::*;
pub use functions::*;
pub use globals::*;
