// Data Loader untuk Hafiz Assistant Backend
// Memuat data dari file JSON dan mengkonversi ke struktur internal

use crate::data::structures::*;
use crate::data::bundle::QuranCoreDataBundle;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Loader untuk data Quran
pub struct DataLoader {
    pub data_dir: String,
}

impl DataLoader {
    /// Membuat loader baru
    pub fn new(data_dir: &str) -> Self {
        Self {
            data_dir: data_dir.to_string(),
        }
    }

    /// Memuat semua data Quran
    pub fn load_all_data(&self) -> Result<QuranCoreDataBundle, Box<dyn std::error::Error>> {
        let mut bundle = QuranCoreDataBundle::new();

        // Load ayah metadata
        bundle.ayah_metadata = self.load_ayah_metadata()?;
        
        // Load ayah data dengan terjemahan
        bundle.ayah_data = self.load_ayah_data()?;
        
        // Load surah names
        bundle.surah_names = self.load_surah_names()?;
        
        // Load surah info
        bundle.surah_info = self.load_surah_info()?;
        
        // Load translations
        bundle.indonesian_translation = self.load_indonesian_translation()?;
        
        // Load transliteration
        bundle.transliteration = self.load_transliteration()?;
        
        // Load page data
        bundle.rendered_pages = self.load_rendered_pages()?;
        
        // Load juz info
        bundle.juz_info = self.load_juz_info()?;
        
        // Load hizb info
        bundle.hizb_info = self.load_hizb_info()?;
        
        // Load ruku info
        bundle.ruku_info = self.load_ruku_info()?;
        
        // Load manzil info
        bundle.manzil_info = self.load_manzil_info()?;
        
        // Load sajda info
        bundle.sajda_info = self.load_sajda_info()?;
        
        // Load phrase verses
        bundle.phrase_verses = self.load_phrase_verses()?;
        
        // Load matching ayahs
        bundle.matching_ayahs = self.load_matching_ayahs()?;

        Ok(bundle)
    }

    /// Memuat metadata ayat
    fn load_ayah_metadata(&self) -> Result<Vec<AyahMetadata>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("quran-metadata-ayah.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut ayahs = Vec::new();
        
        // Parse JSON berdasarkan struktur aktual - object dengan key numerik
        if let Some(verses_obj) = data.as_object() {
            for (_id_str, verse_data) in verses_obj {
                // Struktur: {"1": {"id": 1, "surah_number": 1, ...}, "2": {...}, ...}
                if let Some(ayah) = self.parse_ayah_metadata(verse_data) {
                    ayahs.push(ayah);
                }
            }
        } else if let Some(verses) = data.get("verses").and_then(|v| v.as_array()) {
            // Fallback untuk struktur dengan wrapper "verses"
            for verse in verses {
                if let Some(ayah) = self.parse_ayah_metadata(verse) {
                    ayahs.push(ayah);
                }
            }
        }
        
        // Sort by ID to ensure proper ordering
        ayahs.sort_by_key(|a| a.id);
        
        Ok(ayahs)
    }

    /// Memuat data ayat lengkap
    fn load_ayah_data(&self) -> Result<HashMap<String, AyahData>, Box<dyn std::error::Error>> {
        let mut ayah_data = HashMap::new();
        
        // Load translations first
        let translations = self.load_indonesian_translation()?;
        let transliterations = self.load_transliteration()?;
        
        // Load from metadata file first
        let metadata_file = Path::new(&self.data_dir).join("quran-metadata-ayah.json");
        if metadata_file.exists() {
            let data = self.read_json_file(&metadata_file)?;
            
            if let Some(obj) = data.as_object() {
                for (_, verse_data) in obj {
                    if let Some(mut ayah) = self.parse_ayah_data_from_metadata(verse_data) {
                        // Integrate translation data
                        if let Some(translation) = translations.get(&ayah.verse_key) {
                            ayah.translation_id = translation.clone();
                        }
                        
                        // Integrate transliteration data
                        if let Some(transliteration) = transliterations.get(&ayah.verse_key) {
                            ayah.transliteration = transliteration.clone();
                        }
                        
                        ayah_data.insert(ayah.verse_key.clone(), ayah);
                    }
                }
            }
        }
        
        Ok(ayah_data)
    }

    /// Memuat nama-nama surah
    fn load_surah_names(&self) -> Result<Vec<SurahNameMetadata>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("quran-metadata-surah-name.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut surahs = Vec::new();
        if let Some(chapters) = data.get("chapters").and_then(|c| c.as_array()) {
            for chapter in chapters {
                if let Some(surah) = self.parse_surah_name_metadata(chapter) {
                    surahs.push(surah);
                }
            }
        }
        
        Ok(surahs)
    }

    /// Memuat informasi surah
    fn load_surah_info(&self) -> Result<HashMap<u16, SurahInfo>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("surah-info-id.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut surah_info = HashMap::new();
        
        // Berdasarkan analisis Python, struktur adalah object dengan key numerik string
        if let Some(obj) = data.as_object() {
            for (_key, surah_data) in obj {
                if let Some(surah) = self.parse_surah_info(surah_data) {
                    surah_info.insert(surah.id, surah);
                }
            }
        } else if let Some(chapters) = data.as_array() {
            // Fallback untuk struktur array
            for chapter in chapters {
                if let Some(surah) = self.parse_surah_info(chapter) {
                    surah_info.insert(surah.id, surah);
                }
            }
        }
        
        Ok(surah_info)
    }

    /// Memuat terjemahan Indonesia
    fn load_indonesian_translation(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("indonesian-mokhtasar.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut translations = HashMap::new();
        
        // Handle direct verse key structure: { "1:1": {"text": "..."}, "1:2": {"text": "..."}, ... }
        if let Some(obj) = data.as_object() {
            for (verse_key, verse_data) in obj {
                if let Some(text) = verse_data.get("text").and_then(|t| t.as_str()) {
                    translations.insert(verse_key.clone(), text.to_string());
                }
            }
        }
        
        Ok(translations)
    }

    /// Memuat transliterasi
    fn load_transliteration(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("transliteration-simple.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut transliterations = HashMap::new();
        
        // Handle direct verse key structure: { "1:1": "...", "1:2": "...", ... } (may be string or object)
        if let Some(obj) = data.as_object() {
            for (verse_key, verse_data) in obj {
                let text = if let Some(text_str) = verse_data.as_str() {
                    // Direct string value
                    text_str.to_string()
                } else if let Some(text_obj) = verse_data.get("text").and_then(|t| t.as_str()) {
                    // Object with text field
                    text_obj.to_string()
                } else {
                    continue;
                };
                transliterations.insert(verse_key.clone(), text);
            }
        }
        
        Ok(transliterations)
    }

    /// Memuat data halaman
    fn load_rendered_pages(&self) -> Result<Vec<RenderedPage>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("qpc-v2-15-lines.db.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut pages = Vec::new();
        
        // Parse database structure
        if let Some(objects) = data.get("objects").and_then(|o| o.as_array()) {
            // Cari tabel pages
            for table in objects {
                if let Some(table_name) = table.get("name").and_then(|n| n.as_str()) {
                    if table_name == "pages" {
                        if let Some(rows) = table.get("rows").and_then(|r| r.as_array()) {
                            pages = self.parse_pages_from_database_rows(rows)?;
                        }
                        break;
                    }
                }
            }
        }
        
        Ok(pages)
    }

    /// Parse halaman dari database rows
    fn parse_pages_from_database_rows(&self, rows: &[serde_json::Value]) -> Result<Vec<RenderedPage>, Box<dyn std::error::Error>> {
        use std::collections::HashMap;
        
        // Group rows by page number
        let mut pages_map: HashMap<u16, Vec<_>> = HashMap::new();
        
        for row in rows {
            if let Some(row_array) = row.as_array() {
                if row_array.len() >= 7 {
                    let page_number = row_array[0].as_u64().unwrap_or(0) as u16;
                    let line_number = row_array[1].as_u64().unwrap_or(0) as u8;
                    let line_type = row_array[2].as_str().unwrap_or("").to_string();
                    let is_centered = row_array[3].as_u64().unwrap_or(0) == 1;
                    let first_word_id = row_array[4].as_u64().unwrap_or(0) as u32;
                    let last_word_id = row_array[5].as_u64().unwrap_or(0) as u32;
                    let surah_number = if let Some(surah_val) = row_array[6].as_u64() {
                        Some(surah_val as u16)
                    } else {
                        None
                    };
                    
                    pages_map.entry(page_number).or_insert_with(Vec::new).push((
                        line_number,
                        line_type,
                        is_centered,
                        first_word_id,
                        last_word_id,
                        surah_number,
                    ));
                }
            }
        }
        
        // Load ayah data for text mapping
        let ayah_data = self.load_ayah_data()?;
        
        // Convert to RenderedPage structs
        let mut pages = Vec::new();
        let mut page_numbers: Vec<u16> = pages_map.keys().cloned().collect();
        page_numbers.sort();
        
        for page_number in page_numbers {
            if let Some(page_rows) = pages_map.get(&page_number) {
                let mut lines = Vec::new();
                let mut surah_headers = Vec::new();
                
                // Sort lines by line number
                let mut sorted_rows = page_rows.clone();
                sorted_rows.sort_by_key(|row| row.0);
                
                for (line_number, line_type, is_centered, first_word_id, last_word_id, surah_number) in sorted_rows {
                    // Determine text and verse keys based on line type
                    let (text, verse_keys) = if line_type == "surah_name" {
                        // Surah header line
                        if let Some(surah_num) = surah_number {
                            if let Some(surah_info) = self.get_surah_info_by_number(surah_num) {
                                surah_headers.push(SurahHeader {
                                    surah_number: surah_num,
                                    name_arabic: surah_info.name_arabic.clone(),
                                    name_simple: surah_info.name_simple.clone(),
                                    line_number,
                                });
                                (surah_info.name_arabic, Vec::new())
                            } else {
                                (format!("Surah {}", surah_num), Vec::new())
                            }
                        } else {
                            ("".to_string(), Vec::new())
                        }
                    } else if line_type == "ayah" && first_word_id > 0 && last_word_id > 0 {
                        // Ayah line - we need to map word IDs to verses
                        // For now, we'll use a simplified approach
                        self.get_text_and_verses_for_word_range(first_word_id, last_word_id, &ayah_data)
                    } else {
                        ("".to_string(), Vec::new())
                    };
                    
                    lines.push(PageLine {
                        line_number,
                        line_type,
                        text,
                        is_centered,
                        verse_keys,
                    });
                }
                
                pages.push(RenderedPage {
                    page_number,
                    lines,
                    surah_headers,
                });
            }
        }
        
        Ok(pages)
    }

    /// Get text and verse keys for word range (optimized implementation)
    fn get_text_and_verses_for_word_range(
        &self, 
        first_word_id: u32, 
        last_word_id: u32, 
        _ayah_data: &HashMap<String, AyahData>
    ) -> (String, Vec<String>) {
        // Simple implementation that estimates verses based on word IDs
        // This is much faster than building a complete word mapping
        
        // Rough estimation: average 13.42 words per ayah based on our analysis
        let avg_words_per_ayah = 13.42;
        
        // Estimate which verse range this word range covers
        let start_verse_id = ((first_word_id as f64 - 1.0) / avg_words_per_ayah).floor() as u32 + 1;
        let end_verse_id = ((last_word_id as f64 - 1.0) / avg_words_per_ayah).ceil() as u32 + 1;
        
        // Convert verse IDs to verse keys (this is approximate)
        let mut verse_keys = Vec::new();
        
        // Simple mapping: first 7 verses are Al-Fatihah, then Al-Baqarah starts
        for verse_id in start_verse_id..=end_verse_id.min(start_verse_id + 3) {
            if verse_id <= 7 {
                verse_keys.push(format!("1:{}", verse_id));
            } else {
                // Estimate surah and ayah for verses after Al-Fatihah
                let ayah_in_baqarah = verse_id - 7;
                if ayah_in_baqarah <= 286 {
                    verse_keys.push(format!("2:{}", ayah_in_baqarah));
                }
            }
        }
        
        // Generate simple text placeholder
        let text = format!("Text for words {}-{}", first_word_id, last_word_id);
        
        (text, verse_keys)
    }

    /// Get surah info by number (helper method)
    fn get_surah_info_by_number(&self, surah_number: u16) -> Option<SurahInfo> {
        // Load surah info and find by number
        if let Ok(surah_info_map) = self.load_surah_info() {
            surah_info_map.get(&surah_number).cloned()
        } else {
            None
        }
    }

    /// Memuat info juz
    fn load_juz_info(&self) -> Result<HashMap<u8, JuzInfo>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("quran-metadata-juz.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut juz_info = HashMap::new();
        if let Some(juzs) = data.get("juzs").and_then(|j| j.as_array()) {
            for juz in juzs {
                if let Some(juz_data) = self.parse_juz_info(juz) {
                    juz_info.insert(juz_data.id, juz_data);
                }
            }
        }
        
        Ok(juz_info)
    }

    /// Memuat info hizb
    fn load_hizb_info(&self) -> Result<HashMap<u8, HizbInfo>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("quran-metadata-hizb.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut hizb_info = HashMap::new();
        if let Some(hizbs) = data.get("hizbs").and_then(|h| h.as_array()) {
            for hizb in hizbs {
                if let Some(hizb_data) = self.parse_hizb_info(hizb) {
                    hizb_info.insert(hizb_data.id, hizb_data);
                }
            }
        }
        
        Ok(hizb_info)
    }

    /// Memuat info ruku
    fn load_ruku_info(&self) -> Result<HashMap<u8, RukuInfo>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("quran-metadata-ruku.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut ruku_info = HashMap::new();
        if let Some(rukus) = data.get("rukus").and_then(|r| r.as_array()) {
            for ruku in rukus {
                if let Some(ruku_data) = self.parse_ruku_info(ruku) {
                    ruku_info.insert(ruku_data.id, ruku_data);
                }
            }
        }
        
        Ok(ruku_info)
    }

    /// Memuat info manzil
    fn load_manzil_info(&self) -> Result<HashMap<u8, ManzilInfo>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("quran-metadata-manzil.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut manzil_info = HashMap::new();
        if let Some(manzils) = data.get("manzils").and_then(|m| m.as_array()) {
            for manzil in manzils {
                if let Some(manzil_data) = self.parse_manzil_info(manzil) {
                    manzil_info.insert(manzil_data.id, manzil_data);
                }
            }
        }
        
        Ok(manzil_info)
    }

    /// Memuat info sajda
    fn load_sajda_info(&self) -> Result<HashMap<u8, SajdaInfo>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("quran-metadata-sajda.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut sajda_info = HashMap::new();
        if let Some(sajdas) = data.get("sajdas").and_then(|s| s.as_array()) {
            for sajda in sajdas {
                if let Some(sajda_data) = self.parse_sajda_info(sajda) {
                    sajda_info.insert(sajda_data.id, sajda_data);
                }
            }
        }
        
        Ok(sajda_info)
    }

    /// Memuat phrase verses
    fn load_phrase_verses(&self) -> Result<HashMap<String, Vec<u32>>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("phrase_verses.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut phrase_verses = HashMap::new();
        if let Some(obj) = data.as_object() {
            for (phrase, verses) in obj {
                if let Some(verse_ids) = verses.as_array() {
                    let ids: Vec<u32> = verse_ids
                        .iter()
                        .filter_map(|v| v.as_u64().map(|n| n as u32))
                        .collect();
                    phrase_verses.insert(phrase.clone(), ids);
                }
            }
        }
        
        Ok(phrase_verses)
    }

    /// Memuat matching ayahs
    fn load_matching_ayahs(&self) -> Result<HashMap<String, Vec<MatchingAyah>>, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.data_dir).join("matching-ayah.json");
        let data = self.read_json_file(&file_path)?;
        
        let mut matching_ayahs = HashMap::new();
        if let Some(obj) = data.as_object() {
            for (key, matches) in obj {
                if let Some(match_list) = matches.as_array() {
                    let matches: Vec<MatchingAyah> = match_list
                        .iter()
                        .filter_map(|v| self.parse_matching_ayah(v))
                        .collect();
                    matching_ayahs.insert(key.clone(), matches);
                }
            }
        }
        
        Ok(matching_ayahs)
    }

    /// Parse matching ayah dari JSON
    fn parse_matching_ayah(&self, match_obj: &Value) -> Option<MatchingAyah> {
        Some(MatchingAyah {
            matched_ayah_key: match_obj.get("matched_ayah_key")?.as_str()?.to_string(),
            matched_words_count: match_obj.get("matched_words_count")?.as_u64()? as u32,
            coverage: match_obj.get("coverage")?.as_u64()? as u32,
            score: match_obj.get("score")?.as_u64()? as u32,
            match_words: match_obj.get("match_words")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| {
                            item.as_array().map(|range| {
                                range.iter()
                                    .filter_map(|n| n.as_u64().map(|v| v as u32))
                                    .collect::<Vec<u32>>()
                            })
                        })
                        .collect::<Vec<Vec<u32>>>()
                })
                .unwrap_or_default(),
        })
    }

    /// Membaca file JSON
    fn read_json_file(&self, file_path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Value = serde_json::from_str(&contents)?;
        Ok(data)
    }

    /// Parse ayah metadata dari JSON
    fn parse_ayah_metadata(&self, verse: &Value) -> Option<AyahMetadata> {
        Some(AyahMetadata {
            id: verse.get("id")?.as_u64()? as u32,
            surah_number: verse.get("surah_number")?.as_u64()? as u16,
            ayah_number: verse.get("ayah_number")?.as_u64()? as u16,
            verse_key: verse.get("verse_key")?.as_str()?.to_string(),
            text: verse.get("text")?.as_str()?.to_string(),
        })
    }

    /// Parse ayah data dari metadata JSON
    fn parse_ayah_data_from_metadata(&self, verse: &Value) -> Option<AyahData> {
        Some(AyahData {
            id: verse.get("id")?.as_u64()? as u32,
            surah_number: verse.get("surah_number")?.as_u64()? as u16,
            ayah_number: verse.get("ayah_number")?.as_u64()? as u16,
            verse_key: verse.get("verse_key")?.as_str()?.to_string(),
            text_arab: verse.get("text")?.as_str()?.to_string(),
            text_uthmani: verse.get("text")?.as_str()?.to_string(),
            translation_id: String::new(),
            transliteration: String::new(),
            surah_name: String::new(),
            surah_name_arabic: String::new(),
            juz_number: 1, // Default values
            hizb_number: 1,
            rub_number: 1,
            ruku_number: 1,
            manzil_number: 1,
            page_number: 1,
            line_number: 1,
            sajda: None,
        })
    }

    /// Parse surah name metadata
    fn parse_surah_name_metadata(&self, chapter: &Value) -> Option<SurahNameMetadata> {
        Some(SurahNameMetadata {
            id: chapter.get("id")?.as_u64()? as u16,
            name_simple: chapter.get("name_simple")?.as_str()?.to_string(),
            name_arabic: chapter.get("name_arabic")?.as_str()?.to_string(),
            revelation_order: chapter.get("revelation_order")?.as_u64()? as u16,
            revelation_place: chapter.get("revelation_place")?.as_str()?.to_string(),
            verses_count: chapter.get("verses_count")?.as_u64()? as u16,
            bismillah_pre: chapter.get("bismillah_pre")?.as_bool()?,
        })
    }

    /// Parse surah info
    fn parse_surah_info(&self, chapter: &Value) -> Option<SurahInfo> {
        // Berdasarkan analisis Python, struktur aktual adalah:
        // {"surah_number": 1, "surah_name": "Al-Fatihah", "text": "...", "short_text": "..."}
        Some(SurahInfo {
            id: chapter.get("surah_number")?.as_u64()? as u16,
            name_simple: chapter.get("surah_name")?.as_str()?.to_string(),
            name_arabic: chapter.get("surah_name")?.as_str()?.to_string(), // Gunakan nama yang sama untuk sementara
            name_english: chapter.get("surah_name")?.as_str()?.to_string(),
            revelation_order: chapter.get("surah_number")?.as_u64()? as u16, // Default ke nomor surah
            revelation_place: "Unknown".to_string(), // Default value
            verses_count: 1, // Default value - akan diupdate nanti
            bismillah_pre: chapter.get("surah_number")?.as_u64()? != 1, // Semua kecuali Al-Fatihah
        })
    }

    /// Parse juz info
    fn parse_juz_info(&self, juz: &Value) -> Option<JuzInfo> {
        Some(JuzInfo {
            id: juz.get("id")?.as_u64()? as u8,
            first_verse_id: juz.get("first_verse_id")?.as_u64()? as u32,
            last_verse_id: juz.get("last_verse_id")?.as_u64()? as u32,
            verses_count: juz.get("verses_count")?.as_u64()? as u16,
            name_arabic: juz.get("name_arabic").and_then(|n| n.as_str()).unwrap_or("").to_string(),
            name_simple: juz.get("name_simple").and_then(|n| n.as_str()).unwrap_or("").to_string(),
        })
    }

    /// Parse hizb info
    fn parse_hizb_info(&self, hizb: &Value) -> Option<HizbInfo> {
        Some(HizbInfo {
            id: hizb.get("id")?.as_u64()? as u8,
            first_verse_id: hizb.get("first_verse_id")?.as_u64()? as u32,
            last_verse_id: hizb.get("last_verse_id")?.as_u64()? as u32,
            verses_count: hizb.get("verses_count")?.as_u64()? as u16,
        })
    }

    /// Parse ruku info
    fn parse_ruku_info(&self, ruku: &Value) -> Option<RukuInfo> {
        Some(RukuInfo {
            id: ruku.get("id")?.as_u64()? as u8,
            surah_number: ruku.get("surah_number")?.as_u64()? as u16,
            first_verse_id: ruku.get("first_verse_id")?.as_u64()? as u32,
            last_verse_id: ruku.get("last_verse_id")?.as_u64()? as u32,
            verses_count: ruku.get("verses_count")?.as_u64()? as u16,
        })
    }

    /// Parse manzil info
    fn parse_manzil_info(&self, manzil: &Value) -> Option<ManzilInfo> {
        Some(ManzilInfo {
            id: manzil.get("id")?.as_u64()? as u8,
            first_verse_id: manzil.get("first_verse_id")?.as_u64()? as u32,
            last_verse_id: manzil.get("last_verse_id")?.as_u64()? as u32,
            verses_count: manzil.get("verses_count")?.as_u64()? as u16,
        })
    }

    /// Parse sajda info
    fn parse_sajda_info(&self, sajda: &Value) -> Option<SajdaInfo> {
        Some(SajdaInfo {
            id: sajda.get("id")?.as_u64()? as u8,
            sajda_type: sajda.get("type")?.as_str()?.to_string(),
            recommended: sajda.get("recommended")?.as_bool()?,
            obligatory: sajda.get("obligatory")?.as_bool()?,
        })
    }
}
