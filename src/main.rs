// Impor modul yang diperlukan
use std::{
    collections::HashMap,
    fs::{File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};
use serde::{Deserialize, Serialize};
use serde_json::Value; // Untuk parsing JSON fleksibel
use bincode;
use bincode::config::standard; // Impor konfigurasi standar untuk bincode 2.x.x

// --- Struktur Data Input (Mirroring JSON Anda) ---

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct AyahMetadata {
    pub id: u32,
    pub surah_number: u16,
    pub ayah_number: u16,
    pub verse_key: String,
    pub text: String, // Teks Arab asli ayat
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct QpcLineData {
    pub page_number: u16,
    pub line_number: u16,
    pub line_type: String,
    pub is_centered: u8,
    pub first_word_id: Option<u32>,
    pub last_word_id: Option<u32>,
    pub surah_number: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct LemmaWord {
    pub lemma_id: u32,
    pub word_location: String, // Contoh: "54:26:4" (surah:ayah:word_index_in_ayah)
    pub text: String, // Teks Arab kata
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct SurahNameMetadata {
    pub id: u16,
    pub name_simple: String,
    pub name_arabic: String,
    pub revelation_order: u16,
    pub revelation_place: String,
    pub verses_count: u16,
    pub bismillah_pre: bool,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct JuzMetadata {
    pub juz_number: u16,
    pub verses_count: u32,
    pub first_verse_key: String,
    pub last_verse_key: String,
    pub verse_mapping: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct HizbMetadataEntry {
    pub hizb_number: u16,
    pub verses_count: u32,
    pub first_verse_key: String,
    pub last_verse_key: String,
    pub verse_mapping: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct ManzilMetadataEntry {
    pub manzil_number: u16,
    pub verses_count: u32,
    pub first_verse_key: String,
    pub last_verse_key: String,
    pub verse_mapping: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct RubMetadataEntry {
    pub rub_number: u16,
    pub verses_count: u32,
    pub first_verse_key: String,
    pub last_verse_key: String,
    pub verse_mapping: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct RukuMetadataEntry {
    pub ruku_number: u16,
    pub verses_count: u32,
    pub first_verse_key: String,
    pub last_verse_key: String,
    pub verse_mapping: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct SajdaMetadataEntry {
    pub sajdah_number: u16,
    pub verse_key: String,
    pub sajdah_type: String,
}


#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct Phrase {
    pub surahs: u16,
    pub ayahs: u16,
    pub count: u16,
    pub source: PhraseSource,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct PhraseSource {
    pub key: String,
    #[serde(rename = "from")]
    pub from_idx: u16,
    #[serde(rename = "to")]
    pub to_idx: u16,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct MatchingAyahEntry {
    pub matched_ayah_key: String,
    pub matched_words_count: u16,
    pub coverage: u16,
    pub score: u16,
    pub match_words: Vec<Vec<u16>>,
}

// --- Struktur Data Tambahan ---

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct IndonesianTranslation {
    pub t: String,
}

// PERBAIKAN: Struktur ini tidak lagi digunakan untuk indonesian_mokhtasar_translations
// karena kita akan mem-parsingnya secara manual dari Value ke String
// #[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
// pub struct IndonesianMokhtasarEntry {
//     pub text: String,
// }

// PERBAIKAN: Hapus struct TafsirJalalaynEntry karena tidak dibundel
// #[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
// pub struct TafsirJalalaynEntry {
//     pub text: String,
// }

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct SurahInfoIdEntry {
    pub surah_number: u16,
    pub surah_name: String,
    pub text: String,
    pub short_text: String,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct TopicEntry {
    pub topic_id: u32,
    pub name: String,
    pub arabic_name: String,
    pub parent_id: Option<u32>,
    pub description: String,
    pub ayahs: String,
    pub keywords: String,
    pub total_ayahs: u32,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct WordRoot {
    pub root_id: u32,
    pub word_location: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct WordStem {
    pub stem_id: u32,
    pub word_location: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct TransliterationEntry {
    pub t: String,
}

// --- Struktur Data Output (Bundel Rust) ---

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct RenderedLine {
    pub text: String,
    pub line_type: String,
    pub is_centered: bool,
}

#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct RenderedPage {
    pub page_number: u16,
    pub lines: Vec<RenderedLine>,
}

// Struktur utama untuk bundel data core Al-Qur'an yang akan diserialisasikan
#[derive(Debug, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct QuranCoreDataBundle {
    pub pages_content: HashMap<u16, RenderedPage>,
    pub surah_metadata: HashMap<u16, SurahNameMetadata>,
    pub juz_metadata: HashMap<u16, JuzMetadata>,

    pub hizb_metadata: HashMap<u16, HizbMetadataEntry>,
    pub manzil_metadata: HashMap<u16, ManzilMetadataEntry>,
    pub rub_metadata: HashMap<u16, RubMetadataEntry>,
    pub ruku_metadata: HashMap<u16, RukuMetadataEntry>,
    pub sajda_metadata: HashMap<u16, SajdaMetadataEntry>,

    pub phrases_data: HashMap<String, Phrase>,
    pub phrase_verses_map: HashMap<String, Vec<u32>>,
    pub matching_ayahs_map: HashMap<String, Vec<MatchingAyahEntry>>,

    pub quran_id_simple_translations: HashMap<String, IndonesianTranslation>,
    // PERBAIKAN: Tipe value untuk terjemahan Mokhtasar adalah String
    pub indonesian_mokhtasar_translations: HashMap<String, String>,
    // PERBAIKAN: Hapus in_tafsir_jalalayn dari struct bundle
    // pub in_tafsir_jalalayn: HashMap<String, String>, // Baris ini dihapus
    pub surah_info_id: HashMap<u16, SurahInfoIdEntry>,
    pub topics_data: HashMap<u32, TopicEntry>,
    pub word_roots: HashMap<String, WordRoot>,
    pub word_stems: HashMap<String, WordStem>,
    pub transliterations: HashMap<String, TransliterationEntry>,
}

fn main() -> io::Result<()> {
    println!("Memulai proses bundling data Al-Qur'an...");

    let data_dir = PathBuf::from("data");
    let output_bundle_path = PathBuf::from("quran_core_data.bin");

    // --- Langkah 1: Muat Semua Data Mentah ---
    println!("Memuat data mentah...");

    // Muat quran-metadata-ayah.json
    let ayah_metadata_raw: HashMap<String, AyahMetadata> =
        load_json_file(&data_dir.join("quran-metadata-ayah.json"))?;
    let _ayah_metadata: HashMap<u32, AyahMetadata> = ayah_metadata_raw.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect();


    // Muat qpc-v2-15-lines.db.json (parsing manual karena struktur "database" JSON)
    let qpc_lines_db_json: Value = load_json_file(&data_dir.join("qpc-v2-15-lines.db.json"))?;
    let qpc_lines_raw: Vec<QpcLineData> = qpc_lines_db_json["objects"][1]["rows"]
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "qpc-v2-15-lines.db.json 'rows' not an array"))?
        .iter()
        .map(|row| {
            QpcLineData {
                page_number: row[0].as_u64().unwrap() as u16,
                line_number: row[1].as_u64().unwrap() as u16,
                line_type: row[2].as_str().unwrap().to_string(),
                is_centered: row[3].as_u64().unwrap() as u8,
                first_word_id: row[4].as_u64().map(|id| id as u32),
                last_word_id: row[5].as_u64().map(|id| id as u32),
                surah_number: row[6].as_u64().map(|id| id as u16),
            }
        })
        .collect();

    // Muat word-lemma.db.json (parsing manual)
    let lemma_words_db_json: Value = load_json_file(&data_dir.join("word-lemma.db.json"))?;
    let _lemma_words_raw: Vec<LemmaWord> = lemma_words_db_json["objects"][0]["rows"] // Tandai sebagai unused
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "word-lemma.db.json 'rows' not an array"))?
        .iter()
        .map(|row| {
            LemmaWord {
                lemma_id: row[0].as_u64().unwrap() as u32,
                word_location: row[1].as_str().unwrap().to_string(),
                text: row[2].as_str().unwrap().to_string(),
            }
        })
        .collect();

    // Muat quran-metadata-surah-name.json
    let surah_name_metadata_raw: HashMap<String, SurahNameMetadata> =
        load_json_file(&data_dir.join("quran-metadata-surah-name.json"))?;
    let surah_metadata: HashMap<u16, SurahNameMetadata> = surah_name_metadata_raw.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect();

    // Muat quran-metadata-juz.json
    let juz_metadata_raw: HashMap<String, JuzMetadata> =
        load_json_file(&data_dir.join("quran-metadata-juz.json"))?;
    let juz_metadata: HashMap<u16, JuzMetadata> = juz_metadata_raw.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect();

    // Muat metadata struktural lainnya (Hizb, Manzil, Rub, Ruku, Sajda)
    let hizb_metadata_raw: HashMap<String, HizbMetadataEntry> = load_json_file(&data_dir.join("quran-metadata-hizb.json"))?;
    let hizb_metadata: HashMap<u16, HizbMetadataEntry> = hizb_metadata_raw.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect();

    let manzil_metadata_raw: HashMap<String, ManzilMetadataEntry> = load_json_file(&data_dir.join("quran-metadata-manzil.json"))?;
    let manzil_metadata: HashMap<u16, ManzilMetadataEntry> = manzil_metadata_raw.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect();

    let rub_metadata_raw: HashMap<String, RubMetadataEntry> = load_json_file(&data_dir.join("quran-metadata-rub.json"))?;
    let rub_metadata: HashMap<u16, RubMetadataEntry> = rub_metadata_raw.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect();

    let ruku_metadata_raw: HashMap<String, RukuMetadataEntry> = load_json_file(&data_dir.join("quran-metadata-ruku.json"))?;
    let ruku_metadata: HashMap<u16, RukuMetadataEntry> = ruku_metadata_raw.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect();

    let sajda_metadata_raw: HashMap<String, SajdaMetadataEntry> = load_json_file(&data_dir.join("quran-metadata-sajda.json"))?;
    let sajda_metadata: HashMap<u16, SajdaMetadataEntry> = sajda_metadata_raw.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect();


    // Muat phrases.json
    let phrases_data: HashMap<String, Phrase> =
        load_json_file(&data_dir.join("phrases.json"))?;

    // Muat phrase_verses.json
    let phrase_verses_map: HashMap<String, Vec<u32>> =
        load_json_file(&data_dir.join("phrase_verses.json"))?;

    // Muat matching-ayah.json
    let matching_ayahs_map: HashMap<String, Vec<MatchingAyahEntry>> =
        load_json_file(&data_dir.join("matching-ayah.json"))?;

    // --- Muat Data Tambahan ---

    // quran-id-simple.json
    let quran_id_simple_translations: HashMap<String, IndonesianTranslation> =
        load_json_file(&data_dir.join("quran-id-simple.json"))?;

    // indonesian-mokhtasar.json
    let indonesian_mokhtasar_translations_raw_value: HashMap<String, Value> =
        load_json_file(&data_dir.join("indonesian-mokhtasar.json"))?;
    
    let mut indonesian_mokhtasar_translations: HashMap<String, String> = HashMap::new();
    for (key, value) in indonesian_mokhtasar_translations_raw_value {
        if value.is_object() {
            if let Some(text_val) = value.get("text") {
                if let Some(text_str) = text_val.as_str() {
                    indonesian_mokhtasar_translations.insert(key, text_str.to_string());
                } else {
                    eprintln!("Peringatan: 'text' field di indonesian-mokhtasar.json untuk kunci {} bukan string. Menggunakan string kosong.", key);
                }
            } else {
                eprintln!("Peringatan: Objek di indonesian-mokhtasar.json untuk kunci {} tidak memiliki field 'text'. Menggunakan string kosong.", key);
            }
        } else if value.is_string() {
            indonesian_mokhtasar_translations.insert(key, value.as_str().unwrap().to_string());
        } else {
            eprintln!("Peringatan: Tipe nilai tidak terduga untuk kunci {} di indonesian-mokhtasar.json: {:?}", key, value);
        }
    }


    // in-tafsir-jalalayn.json - DIHAPUS DARI BUNDEL
    // Inisialisasi sebagai HashMap kosong karena tidak akan dibundel.
    let _in_tafsir_jalalayn_map: HashMap<String, String> = HashMap::new();


    // surah-info-id.json
    let surah_info_id_raw: HashMap<String, SurahInfoIdEntry> =
        load_json_file(&data_dir.join("surah-info-id.json"))?;
    let surah_info_id: HashMap<u16, SurahInfoIdEntry> = surah_info_id_raw.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect();

    // topics.db.json (parsing manual)
    let topics_db_json: Value = load_json_file(&data_dir.join("topics.db.json"))?;
    let topics_data_raw: Vec<TopicEntry> = topics_db_json["objects"][0]["rows"]
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "topics.db.json 'rows' not an array"))?
        .iter()
        .map(|row| {
            TopicEntry {
                topic_id: row[0].as_u64().unwrap() as u32,
                name: row[1].as_str().unwrap().to_string(),
                arabic_name: row[2].as_str().unwrap_or("").to_string(),
                parent_id: row[3].as_u64().map(|id| id as u32),
                description: row[4].as_str().unwrap_or("").to_string(),
                ayahs: row[5].as_str().unwrap_or("").to_string(),
                keywords: row[6].as_str().unwrap_or("").to_string(),
                total_ayahs: row[7].as_u64().unwrap_or(0) as u32,
            }
        })
        .collect();
    let topics_data: HashMap<u32, TopicEntry> = topics_data_raw.into_iter().map(|t| (t.topic_id, t)).collect();


    // word-root.db.json (parsing manual)
    let word_root_db_json: Value = load_json_file(&data_dir.join("word-root.db.json"))?;
    let word_roots_raw: Vec<WordRoot> = word_root_db_json["objects"][0]["rows"]
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "word-root.db.json 'rows' not an array"))?
        .iter()
        .map(|row| {
            WordRoot {
                root_id: row[0].as_u64().unwrap() as u32,
                word_location: row[1].as_str().unwrap().to_string(),
                text: row[2].as_str().unwrap().to_string(),
            }
        })
        .collect();
    let word_roots: HashMap<String, WordRoot> = word_roots_raw.into_iter().map(|w| (w.word_location.clone(), w)).collect();


    // word-stem.db.json (parsing manual)
    let word_stem_db_json: Value = load_json_file(&data_dir.join("word-stem.db.json"))?;
    let word_stems_raw: Vec<WordStem> = word_stem_db_json["objects"][0]["rows"]
        .as_array()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "word-stem.db.json 'rows' not an array"))?
        .iter()
        .map(|row| {
            WordStem {
                stem_id: row[0].as_u64().unwrap() as u32,
                word_location: row[1].as_str().unwrap().to_string(),
                text: row[2].as_str().unwrap().to_string(),
            }
        })
        .collect();
    let word_stems: HashMap<String, WordStem> = word_stems_raw.into_iter().map(|w| (w.word_location.clone(), w)).collect();


    // transliteration-simple.json
    let transliterations: HashMap<String, TransliterationEntry> =
        load_json_file(&data_dir.join("transliteration-simple.json"))?;


    println!("Data mentah berhasil dimuat.");

    // --- Langkah 2: Bangun "Bank Kata" Global yang Lebih Robust ---
    println!("Membangun bank kata global...");

    let mut word_location_to_text: HashMap<String, String> = HashMap::new();
    // PERBAIKAN: Menggunakan word_roots untuk mengisi word_location_to_text secara awal
    for word_root_entry in word_roots.values() {
        word_location_to_text.insert(word_root_entry.word_location.clone(), word_root_entry.text.clone());
    }
    // PERBAIKAN: Menambahkan kata-kata dari word_stems jika mereka punya word_location yang unik
    for word_stem_entry in word_stems.values() {
        word_location_to_text.insert(word_stem_entry.word_location.clone(), word_stem_entry.text.clone());
    }
    // PERBAIKAN: Menambahkan kata-kata dari lemma_words_raw untuk memastikan semua kata dari lemma ada
    // Ini mengasumsikan lemma_words_raw adalah sumber paling lengkap untuk pemetaan word_location ke teks kata
    // dan kita akan menimpa jika word_roots/word_stems memiliki versi yang lebih spesifik/diinginkan.
    // Jika lemma_words_raw adalah Vec<LemmaWord> seperti yang di-parse, kita perlu memprosesnya.
    // Dulu _lemma_words_raw tidak digunakan. Sekarang kita perlu membuatnya digunakan.
    // Asumsi: word_location di LemmaWord sama dengan di WordRoot/WordStem
    // Anda mungkin perlu memilah prioritas teks kata dari lemma, root, atau stem.
    // Untuk saat ini, kita akan menggunakan word_location_to_text yang sudah diisi dari word_roots/stems
    // dan _lemma_words_raw tidak digunakan langsung di sini untuk mengisi bank kata.

    let mut global_quran_words: Vec<String> = Vec::new();
    let mut global_word_id_map: HashMap<(u16, u16, u16), u32> = HashMap::new();
    let mut current_global_word_id: u32 = 1;

    trait ArabicCharExt {
        fn is_alphanumeric_arabic(&self) -> bool;
    }

    impl ArabicCharExt for char {
        fn is_alphanumeric_arabic(&self) -> bool {
            (*self >= '\u{0621}' && *self <= '\u{064A}') || // Arabic letters
            (*self >= '\u{0660}' && *self <= '\u{0669}') || // Arabic-Indic digits
            (*self >= '\u{0670}' && *self <= '\u{06D3}') || // Extended Arabic letters
            (*self >= '\u{06D5}' && *self <= '\u{06ED}') || // Extended Arabic letters (more)
            (*self >= '\u{06F0}' && *self <= '\u{06F9}')    // Eastern Arabic-Indic digits
        }
    }

    // Menggunakan _ayah_metadata yang sudah dimuat dari quran-metadata-ayah.json
    let mut sorted_ayah_metadata: Vec<&AyahMetadata> = _ayah_metadata.values().collect();
    sorted_ayah_metadata.sort_by(|a, b| {
        a.surah_number.cmp(&b.surah_number)
            .then_with(|| a.ayah_number.cmp(&b.ayah_number))
    });

    for ayah_meta in sorted_ayah_metadata {
        let words_in_ayah_text: Vec<String> = ayah_meta.text
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric_arabic() && c != 'ـ').to_string())
            .filter(|s| !s.is_empty())
            .collect();

        for (word_idx_in_ayah_zero_based, word_text) in words_in_ayah_text.into_iter().enumerate() {
            let word_idx_in_ayah_one_based = (word_idx_in_ayah_zero_based + 1) as u16;
            let word_location_key = (ayah_meta.surah_number, ayah_meta.ayah_number, word_idx_in_ayah_one_based);

            // Cek apakah word_location ini ada di word_location_to_text (dari word-lemma/root/stem)
            // Jika ada, gunakan teks yang lebih presisi dari word-lemma/root/stem
            let final_word_text = if let Some(precise_text) = word_location_to_text.get(&format!("{}:{}:{}", ayah_meta.surah_number, ayah_meta.ayah_number, word_idx_in_ayah_one_based)) {
                precise_text.clone()
            } else {
                word_text // Gunakan teks dari quran-metadata-ayah jika tidak ada di word-lemma/root/stem
            };

            global_quran_words.push(final_word_text);
            global_word_id_map.insert(word_location_key, current_global_word_id);
            current_global_word_id += 1;
        }
    }
    
    println!("Bank kata global berhasil dibangun ({} kata).", global_quran_words.len());

    // --- Langkah 3: Bangun Data Tampilan Halaman (Teks Per Baris) ---
    println!("Membangun konten halaman Mushaf...");

    let mut pages_content: HashMap<u16, RenderedPage> = HashMap::new();

    let mut qpc_lines_by_page: HashMap<u16, Vec<&QpcLineData>> = HashMap::new();
    for line_data in &qpc_lines_raw {
        qpc_lines_by_page.entry(line_data.page_number).or_insert_with(Vec::new).push(line_data);
    }

    for page_num in 1..=604 {
        let mut lines_for_page: Vec<RenderedLine> = Vec::with_capacity(15);
        
        if let Some(lines_raw) = qpc_lines_by_page.get(&page_num) {
            let mut sorted_lines_raw = lines_raw.clone();
            sorted_lines_raw.sort_by_key(|l| l.line_number);

            for line_data in sorted_lines_raw {
                let mut line_text = String::new();
                let is_centered_bool = line_data.is_centered == 1;

                match line_data.line_type.as_str() {
                    "ayah" => {
                        if let (Some(first_id), Some(last_id)) = (line_data.first_word_id, line_data.last_word_id) {
                            if first_id >= 1 && (last_id as usize) <= global_quran_words.len() {
                                for i in (first_id - 1)..last_id {
                                    if let Some(word) = global_quran_words.get(i as usize) {
                                        line_text.push_str(word);
                                        line_text.push(' ');
                                    }
                                }
                                line_text = line_text.trim_end().to_string();
                            } else {
                                eprintln!("Peringatan: ID kata tidak valid untuk halaman {} baris {}. First: {}, Last: {}. Total kata: {}", page_num, line_data.line_number, first_id, last_id, global_quran_words.len());
                                line_text = format!("ERROR: Invalid word IDs for page {} line {}", page_num, line_data.line_number);
                            }
                        } else {
                            eprintln!("Peringatan: Ayah tanpa first_word_id/last_word_id di halaman {} baris {}", page_num, line_data.line_number);
                            line_text = format!("ERROR: Missing word IDs for page {} line {}", page_num, line_data.line_number);
                        }
                    }
                    "surah_name" => {
                        if let Some(surah_num) = line_data.surah_number {
                            if let Some(meta) = surah_metadata.get(&surah_num) {
                                line_text = meta.name_arabic.clone();
                            } else {
                                eprintln!("Peringatan: Metadata surah {} tidak ditemukan untuk halaman {} baris {}", surah_num, page_num, line_data.line_number);
                                line_text = format!("Surah {}", surah_num);
                            }
                        } else {
                            eprintln!("Peringatan: Nama surah tanpa surah_number di halaman {} baris {}", page_num, line_data.line_number);
                            line_text = "Nama Surah".to_string();
                        }
                    }
                    "basmallah" => {
                        line_text = "بِسْمِ ٱللَّهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ".to_string();
                    }
                    _ => {
                        eprintln!("Peringatan: Tipe baris tidak dikenal: {} di halaman {} baris {}", line_data.line_type, page_num, line_data.line_number);
                        line_text = format!("UNKNOWN LINE TYPE: {}", line_data.line_type);
                    }
                }
                lines_for_page.push(RenderedLine {
                    text: line_text,
                    line_type: line_data.line_type.clone(),
                    is_centered: is_centered_bool,
                });
            }
        } else {
            eprintln!("Peringatan: Tidak ada data baris di qpc-v2-15-lines.db.json untuk halaman {}", page_num);
            for i in 1..=15 {
                lines_for_page.push(RenderedLine {
                    text: format!("Halaman {} - Baris {} (Data Tidak Tersedia)", page_num, i),
                    line_type: "placeholder".to_string(),
                    is_centered: false,
                });
            }
        }
        pages_content.insert(page_num, RenderedPage { page_number: page_num, lines: lines_for_page });
    }
    println!("Konten halaman Mushaf berhasil dibangun.");

    // --- Langkah 4: Bangun Bundel Data Akhir ---
    println!("Membangun bundel data akhir...");

    let final_bundle = QuranCoreDataBundle {
        pages_content,
        surah_metadata,
        juz_metadata,
        hizb_metadata,
        manzil_metadata,
        rub_metadata,
        ruku_metadata,
        sajda_metadata,
        phrases_data,
        phrase_verses_map,
        matching_ayahs_map,
        quran_id_simple_translations,
        indonesian_mokhtasar_translations,
        // in_tafsir_jalalayn, // Baris ini dihapus
        surah_info_id,
        topics_data,
        word_roots,
        word_stems,
        transliterations,
    };

    // --- Langkah 5: Serialisasikan Bundel ke File Biner ---
    println!("Menserialisasikan bundel ke file biner...");
    
    let encoded: Vec<u8> = bincode::encode_to_vec(&final_bundle, standard())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Gagal serialisasi: {}", e)))?;

    let mut file = File::create(&output_bundle_path)?;
    file.write_all(&encoded)?;

    println!(
        "Bundel data berhasil disimpan ke {:?} dengan ukuran {} byte.",
        output_bundle_path,
        encoded.len()
    );

    // --------------------------------------------------------------------
    // Bagian 6: Pengujian Bundel (Simulasi Penggunaan Runtime)
    // Ini menunjukkan bagaimana aplikasi Flutter Anda akan meminta Rust
    // untuk data tertentu saat runtime.
    // --------------------------------------------------------------------

    println!("\nMemulai pengujian bundel data...");

    // Deserialisasikan bundel dari file biner yang baru saja dibuat
    let loaded_bundle = match deserialize_bundle(&output_bundle_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Gagal memuat bundel untuk pengujian: {}", e);
            return Err(e);
        }
    };
    println!("Bundel data berhasil dimuat kembali untuk pengujian.");

    // --- Pengujian Fungsionalitas ---

    // 1. Uji Konten Halaman Mushaf (Misal Halaman 1 dan Halaman 604)
    println!("\n--- Uji Konten Halaman Mushaf ---");
    let test_pages = vec![1, 604]; // Halaman yang ingin diuji
    for page_num in test_pages {
        if let Some(page_content) = loaded_bundle.pages_content.get(&page_num) {
            println!("Konten Halaman {}:", page_num);
            for line in &page_content.lines {
                // Batasi panjang teks untuk tampilan konsol dengan aman untuk UTF-8
                let display_text = if line.text.chars().count() > 70 {
                    let truncated: String = line.text.chars().take(70).collect();
                    format!("{}...", truncated)
                } else {
                    line.text.clone()
                };
                println!("  Baris {}: [{} {}] {}", line.line_type, line.is_centered, line.text.len(), display_text);
            }
        } else {
            println!("Halaman {} tidak ditemukan dalam bundel.", page_num);
        }
    }

    // 2. Uji Metadata Surah (Misal Surah Al-Fatihah (1) dan An-Nas (114))
    println!("\n--- Uji Metadata Surah ---");
    let test_surahs = vec![1, 114];
    for surah_num in test_surahs {
        if let Some(surah_meta) = loaded_bundle.surah_metadata.get(&surah_num) {
            println!(
                "Metadata Surah {}: {} ({}) - {} ayat, Wahyu di {}",
                surah_num, surah_meta.name_arabic, surah_meta.name_simple,
                surah_meta.verses_count, surah_meta.revelation_place
            );
        } else {
            println!("Metadata surah {} tidak ditemukan.", surah_num);
        }
    }

    // 3. Uji Terjemahan (Misal 1:1 dan 2:255)
    println!("\n--- Uji Terjemahan ---");
    let test_verse_keys = vec!["1:1".to_string(), "2:255".to_string()];
    for verse_key in test_verse_keys {
        if let Some(translation_entry) = loaded_bundle.quran_id_simple_translations.get(&verse_key) {
            println!("Terjemahan QID untuk {}: {}", verse_key, translation_entry.t);
        }
        // Perbaikan: Akses teks dari indonesian_mokhtasar_translations yang kini adalah HashMap<String, String>
        if let Some(mokhtasar_text) = loaded_bundle.indonesian_mokhtasar_translations.get(&verse_key) {
            println!("Terjemahan Mokhtasar untuk {}: {}", verse_key, mokhtasar_text);
        } else {
             println!("Terjemahan Mokhtasar untuk {} tidak ditemukan.", verse_key);
        }
    }
    
    // 4. Uji Pencarian Frasa (Contoh: "الحمد لله")
    println!("\n--- Uji Pencarian Frasa ---");
    // Perhatian: Key di phrases_data adalah String, pastikan ini adalah key yang valid di phrases.json Anda
    let test_phrase_query = "الحمد لله".to_string(); 
    if let Some(phrase_info) = loaded_bundle.phrases_data.get(&test_phrase_query) {
        println!("Frasa ditemukan: '{}' (Muncul {} kali)", test_phrase_query, phrase_info.count);
        // phrase_verses_map memiliki key berupa verse_key, bukan phrase_text
        // Anda perlu memetakan phrase_id dari phrases_data ke phrase_verses_map
        // Untuk pengujian sederhana, kita bisa coba key yang ada di phrase_verses.json
        // Ganti "2:23" dengan verse_key valid dari phrase_verses.json yang terkait dengan frasa.
        // Jika phrase_verses_map diindeks oleh phrase_id (Integer) dari phrases.json,
        // maka Anda perlu mendapatkan ID frasa dari `phrase_info.source.key`
        // dan menggunakannya untuk mencari di phrase_verses_map
        // Untuk saat ini, saya akan tetap menggunakan contoh "2:23" yang mungkin ada.
        if let Some(verse_ids) = loaded_bundle.phrase_verses_map.get("2:23") { // Contoh: gunakan verse_key yang ada di phrase_verses.json
            println!("  Muncul di ID frasa (contoh 2:23): {:?}", verse_ids); 
        } else {
            println!("  Tidak ada pemetaan verse_ids untuk frasa ini di phrase_verses_map (contoh: 2:23).");
        }
    } else {
        println!("Frasa '{}' tidak ditemukan dalam data phrases_data. Pastikan key frasa sesuai.", test_phrase_query);
    }
    
    // 5. Uji Ayat Mirip (Misal 2:255)
    println!("\n--- Uji Ayat Mirip ---");
    let test_matching_ayah_key = "2:255".to_string();
    if let Some(similar_ayahs) = loaded_bundle.matching_ayahs_map.get(&test_matching_ayah_key) {
        println!("Ayat mirip untuk {}:", test_matching_ayah_key);
        for entry in similar_ayahs {
            println!(
                "  - {} (Cocok kata: {}, Cakupan: {}, Skor: {})",
                entry.matched_ayah_key, entry.matched_words_count, entry.coverage, entry.score
            );
        }
    } else {
        println!("Tidak ada ayat mirip ditemukan untuk {}.", test_matching_ayah_key);
    }

    // 6. Uji Topik (Misal topik ID 1)
    println!("\n--- Uji Topik ---");
    let test_topic_id = 1; // Ganti dengan ID topik yang valid dari topics.db.json Anda
    if let Some(topic_entry) = loaded_bundle.topics_data.get(&test_topic_id) {
        println!("Topik ID {}: '{}' (Ayahs: {})", test_topic_id, topic_entry.name, topic_entry.ayahs);
    } else {
        println!("Topik ID {} tidak ditemukan.", test_topic_id);
    }

    println!("\nPengujian bundel selesai.");

    Ok(())
}

// --- Fungsi Pembantu (untuk deserialisasi bundel) ---

fn deserialize_bundle(input_path: &Path) -> io::Result<QuranCoreDataBundle> {
    let mut file = File::open(input_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    bincode::decode_from_slice(&buffer, standard())
        .map(|(decoded, _)| decoded)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Gagal deserialisasi bundel: {}", e)))
}

// Fungsi generik untuk memuat file JSON
fn load_json_file<T: serde::de::DeserializeOwned>(path: &Path) -> io::Result<T> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    serde_json::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Gagal parsing JSON dari {:?}: {}", path, e)))
}

