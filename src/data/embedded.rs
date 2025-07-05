// Embedded Quran data files as constants
// Data lengkap Quran dalam format JSON yang di-embed ke binary

/// Quran text in simple Indonesian format
pub const QURAN_ID_SIMPLE: &str = include_str!("../../data/quran-id-simple.json");

/// Indonesian transliteration data
pub const TRANSLITERATION_SIMPLE: &str = include_str!("../../data/transliteration-simple.json");

/// Indonesian translation (Mokhtasar)
pub const INDONESIAN_MOKHTASAR: &str = include_str!("../../data/indonesian-mokhtasar.json");

/// Tafsir Jalalayn in Indonesian
pub const IN_TAFSIR_JALALAYN: &str = include_str!("../../data/in-tafsir-jalalayn.json");

/// Surah information in Indonesian
pub const SURAH_INFO_ID: &str = include_str!("../../data/surah-info-id.json");

/// Matching ayah data for similarity search
pub const MATCHING_AYAH: &str = include_str!("../../data/matching-ayah.json");

/// Ayah themes database
pub const AYAH_THEMES_DB: &str = include_str!("../../data/ayah-themes.db.json");

/// Topics database
pub const TOPICS_DB: &str = include_str!("../../data/topics.db.json");

/// Word lemma database
pub const WORD_LEMMA_DB: &str = include_str!("../../data/word-lemma.db.json");

/// Word root database
pub const WORD_ROOT_DB: &str = include_str!("../../data/word-root.db.json");

/// Word stem database
pub const WORD_STEM_DB: &str = include_str!("../../data/word-stem.db.json");

/// Phrase verses mapping
pub const PHRASE_VERSES: &str = include_str!("../../data/phrase_verses.json");

/// Phrases database
pub const PHRASES: &str = include_str!("../../data/phrases.json");

/// Quran page layout database (QPC v2 15 lines)
pub const QPC_V2_15_LINES_DB: &str = include_str!("../../data/qpc-v2-15-lines.db.json");

// Quran Metadata Files
/// Quran metadata for ayahs
pub const QURAN_METADATA_AYAH: &str = include_str!("../../data/quran-metadata-ayah.json");

/// Quran metadata for juz (parts)
pub const QURAN_METADATA_JUZ: &str = include_str!("../../data/quran-metadata-juz.json");

/// Quran metadata for hizb (half-parts)
pub const QURAN_METADATA_HIZB: &str = include_str!("../../data/quran-metadata-hizb.json");

/// Quran metadata for manzil (sections)
pub const QURAN_METADATA_MANZIL: &str = include_str!("../../data/quran-metadata-manzil.json");

/// Quran metadata for rub (quarters)
pub const QURAN_METADATA_RUB: &str = include_str!("../../data/quran-metadata-rub.json");

/// Quran metadata for ruku (passages)
pub const QURAN_METADATA_RUKU: &str = include_str!("../../data/quran-metadata-ruku.json");

/// Quran metadata for sajda (prostrations)
pub const QURAN_METADATA_SAJDA: &str = include_str!("../../data/quran-metadata-sajda.json");

/// Quran metadata for surah names
pub const QURAN_METADATA_SURAH_NAME: &str = include_str!("../../data/quran-metadata-surah-name.json");