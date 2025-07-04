// Data Structures untuk Hafiz Assistant Backend
// Struktur data lengkap untuk aplikasi Quran

use serde::{Deserialize, Serialize};

/// Metadata lengkap untuk setiap ayat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AyahData {
    pub id: u32,
    pub surah_number: u16,
    pub ayah_number: u16,
    pub verse_key: String,
    pub text_arab: String,
    pub text_uthmani: String,
    pub translation_id: String,
    pub transliteration: String,
    pub surah_name: String,
    pub surah_name_arabic: String,
    pub juz_number: u8,
    pub hizb_number: u8,
    pub rub_number: u8,
    pub ruku_number: u8,
    pub manzil_number: u8,
    pub page_number: u16,
    pub line_number: u8,
    pub sajda: Option<SajdaInfo>,
}

/// Informasi sajda dalam ayat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SajdaInfo {
    pub id: u8,
    pub sajda_type: String,
    pub recommended: bool,
    pub obligatory: bool,
}

/// Informasi surah
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurahInfo {
    pub id: u16,
    pub name_simple: String,
    pub name_arabic: String,
    pub name_english: String,
    pub revelation_order: u16,
    pub revelation_place: String,
    pub verses_count: u16,
    pub bismillah_pre: bool,
}

/// Struktur untuk data ayat yang serupa dari matching-ayah.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchingAyah {
    pub matched_ayah_key: String,
    pub matched_words_count: u32,
    pub coverage: u32,
    pub score: u32,
    pub match_words: Vec<Vec<u32>>, // Word position ranges
}

/// Ayat serupa dengan skor kemiripan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarAyahWithText {
    pub ayah_data: AyahData,
    pub similarity_score: f64,
    pub matching_words: Vec<String>,
    pub matching_type: String, // "exact", "partial", "semantic", "direct", "reverse"
    pub matched_words_count: u32,
    pub coverage: u32,
}

/// Metadata ayat untuk pencarian
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AyahMetadata {
    pub id: u32,
    pub surah_number: u16,
    pub ayah_number: u16,
    pub verse_key: String,
    pub text: String,
}

/// Nama surah dengan metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurahNameMetadata {
    pub id: u16,
    pub name_simple: String,
    pub name_arabic: String,
    pub revelation_order: u16,
    pub revelation_place: String,
    pub verses_count: u16,
    pub bismillah_pre: bool,
}

/// Halaman Quran dengan layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedPage {
    pub page_number: u16,
    pub lines: Vec<PageLine>,
    pub surah_headers: Vec<SurahHeader>,
}

/// Baris dalam halaman
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageLine {
    pub line_number: u8,
    pub line_type: String,
    pub text: String,
    pub is_centered: bool,
    pub verse_keys: Vec<String>,
}

/// Header surah dalam halaman
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurahHeader {
    pub surah_number: u16,
    pub name_arabic: String,
    pub name_simple: String,
    pub line_number: u8,
}

/// Informasi juz
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JuzInfo {
    pub id: u8,
    pub first_verse_id: u32,
    pub last_verse_id: u32,
    pub verses_count: u16,
    pub name_arabic: String,
    pub name_simple: String,
}

/// Informasi hizb
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HizbInfo {
    pub id: u8,
    pub first_verse_id: u32,
    pub last_verse_id: u32,
    pub verses_count: u16,
}

/// Informasi ruku
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RukuInfo {
    pub id: u8,
    pub surah_number: u16,
    pub first_verse_id: u32,
    pub last_verse_id: u32,
    pub verses_count: u16,
}

/// Informasi manzil
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManzilInfo {
    pub id: u8,
    pub first_verse_id: u32,
    pub last_verse_id: u32,
    pub verses_count: u16,
}

/// Hasil pencarian ayat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub ayahs: Vec<AyahData>,
    pub total_results: usize,
    pub search_time_ms: u64,
    pub query: String,
    pub search_type: String,
}

/// Statistik Quran
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuranStatistics {
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

/// Konfigurasi aplikasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub language: String,
    pub translation_source: String,
    pub font_size: u8,
    pub theme: String,
    pub page_layout: String,
}

/// Error types untuk aplikasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HafizError {
    DataNotFound(String),
    InvalidVerseKey(String),
    InvalidSurahNumber(u16),
    InvalidJuzNumber(u8),
    InvalidPageNumber(u16),
    DatabaseError(String),
    ParseError(String),
    IoError(String),
}

impl std::fmt::Display for HafizError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HafizError::DataNotFound(msg) => write!(f, "Data not found: {}", msg),
            HafizError::InvalidVerseKey(key) => write!(f, "Invalid verse key: {}", key),
            HafizError::InvalidSurahNumber(num) => write!(f, "Invalid surah number: {}", num),
            HafizError::InvalidJuzNumber(num) => write!(f, "Invalid juz number: {}", num),
            HafizError::InvalidPageNumber(num) => write!(f, "Invalid page number: {}", num),
            HafizError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            HafizError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            HafizError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for HafizError {}

/// Type alias untuk Result dengan HafizError
pub type HafizResult<T> = Result<T, HafizError>;
