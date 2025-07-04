// FFI Structures untuk Hafiz Assistant Backend
// Struktur data yang kompatibel dengan C untuk FFI

use std::ffi::c_char;

/// Struktur FFI untuk data ayat
#[repr(C)]
pub struct AyahDataFFI {
    pub verse_key: *mut c_char,
    pub arabic_text: *mut c_char,
    pub translation: *mut c_char,
    pub transliteration: *mut c_char,
    pub surah_name: *mut c_char,
    pub surah_name_arabic: *mut c_char,
    pub surah_number: u16,
    pub ayah_number: u16,
    pub juz_number: u8,
    pub hizb_number: u8,
    pub rub_number: u8,
    pub ruku_number: u8,
    pub manzil_number: u8,
    pub page_number: u16,
}

/// Struktur FFI untuk ayat serupa dengan teks
#[repr(C)]
pub struct SimilarAyahWithTextFFI {
    pub verse_key: *mut c_char,
    pub arabic_text: *mut c_char,
    pub translation: *mut c_char,
    pub transliteration: *mut c_char,
    pub similarity_score: f64,
    pub surah_number: u16,
    pub ayah_number: u16,
}

/// Struktur FFI untuk informasi surah
#[repr(C)]
pub struct SurahInfoFFI {
    pub id: u16,
    pub name_simple: *mut c_char,
    pub name_arabic: *mut c_char,
    pub name_english: *mut c_char,
    pub revelation_order: u16,
    pub revelation_place: *mut c_char,
    pub verses_count: u16,
    pub bismillah_pre: bool,
}

/// Struktur FFI untuk hasil pencarian
#[repr(C)]
pub struct SearchResultFFI {
    pub ayahs: *mut AyahDataFFI,
    pub count: usize,
    pub total_results: usize,
    pub search_time_ms: u64,
    pub query: *mut c_char,
    pub search_type: *mut c_char,
}

/// Struktur FFI untuk statistik Quran
#[repr(C)]
pub struct QuranStatisticsFFI {
    pub total_surahs: u16,
    pub total_ayahs: u32,
    pub total_words: u32,
    pub total_letters: u32,
    pub total_pages: u16,
    pub total_juz: u8,
    pub total_hizb: u8,
    pub total_ruku: u16,
    pub total_manzil: u8,
    pub total_sajda: u8,
}

/// Struktur FFI untuk range ayat
#[repr(C)]
pub struct AyahRangeFFI {
    pub start_surah: u16,
    pub start_ayah: u16,
    pub end_surah: u16,
    pub end_ayah: u16,
}

/// Struktur FFI untuk kriteria pencarian lanjutan
#[repr(C)]
pub struct AdvancedSearchCriteriaFFI {
    pub text_query: *const c_char,
    pub translation_query: *const c_char,
    pub transliteration_query: *const c_char,
    pub surah_number: u16, // 0 means no filter
    pub juz_number: u8,    // 0 means no filter
    pub page_number: u16,  // 0 means no filter
    pub limit: usize,
}

/// Enum untuk jenis pencarian
#[repr(C)]
pub enum SearchTypeFFI {
    TextSearch = 0,
    TranslationSearch = 1,
    TransliterationSearch = 2,
    SimilaritySearch = 3,
    ThemeSearch = 4,
    RangeSearch = 5,
    AdvancedSearch = 6,
    FuzzySearch = 7,
}

/// Struktur FFI untuk error handling
#[repr(C)]
pub struct ErrorFFI {
    pub error_code: i32,
    pub error_message: *mut c_char,
}

// Error codes
pub const ERROR_NONE: i32 = 0;
pub const ERROR_DATA_NOT_INITIALIZED: i32 = 1;
pub const ERROR_INVALID_VERSE_KEY: i32 = 2;
pub const ERROR_INVALID_SURAH_NUMBER: i32 = 3;
pub const ERROR_INVALID_PARAMETERS: i32 = 4;
pub const ERROR_MEMORY_ALLOCATION: i32 = 5;
pub const ERROR_DATA_NOT_FOUND: i32 = 6;
pub const ERROR_UNKNOWN: i32 = 99;
