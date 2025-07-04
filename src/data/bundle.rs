// Data Bundle untuk Hafiz Assistant Backend
// Menggabungkan semua data Quran dalam satu struktur

use crate::data::structures::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Bundle data utama Quran
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuranCoreDataBundle {
    pub ayah_metadata: Vec<AyahMetadata>,
    pub ayah_data: HashMap<String, AyahData>,
    pub surah_names: Vec<SurahNameMetadata>,
    pub surah_info: HashMap<u16, SurahInfo>,
    pub phrase_verses: HashMap<String, Vec<u32>>,
    pub matching_ayahs: HashMap<String, Vec<MatchingAyah>>,
    pub indonesian_translation: HashMap<String, String>,
    pub transliteration: HashMap<String, String>,
    pub rendered_pages: Vec<RenderedPage>,
    pub juz_info: HashMap<u8, JuzInfo>,
    pub hizb_info: HashMap<u8, HizbInfo>,
    pub ruku_info: HashMap<u8, RukuInfo>,
    pub manzil_info: HashMap<u8, ManzilInfo>,
    pub sajda_info: HashMap<u8, SajdaInfo>,
    pub statistics: QuranStatistics,
}

impl QuranCoreDataBundle {
    /// Membuat bundle kosong
    pub fn new() -> Self {
        Self {
            ayah_metadata: Vec::new(),
            ayah_data: HashMap::new(),
            surah_names: Vec::new(),
            surah_info: HashMap::new(),
            phrase_verses: HashMap::new(),
            matching_ayahs: HashMap::new(),
            indonesian_translation: HashMap::new(),
            transliteration: HashMap::new(),
            rendered_pages: Vec::new(),
            juz_info: HashMap::new(),
            hizb_info: HashMap::new(),
            ruku_info: HashMap::new(),
            manzil_info: HashMap::new(),
            sajda_info: HashMap::new(),
            statistics: QuranStatistics::default(),
        }
    }

    /// Mendapatkan data ayat berdasarkan verse key
    pub fn get_ayah_data(&self, verse_key: &str) -> Option<&AyahData> {
        self.ayah_data.get(verse_key)
    }

    /// Mendapatkan informasi surah
    pub fn get_surah_info(&self, surah_number: u16) -> Option<&SurahInfo> {
        self.surah_info.get(&surah_number)
    }

    /// Mendapatkan ayat-ayat dalam surah
    pub fn get_ayahs_by_surah(&self, surah_number: u16) -> Vec<&AyahData> {
        self.ayah_data
            .values()
            .filter(|ayah| ayah.surah_number == surah_number)
            .collect()
    }

    /// Mendapatkan ayat-ayat dalam juz
    pub fn get_ayahs_by_juz(&self, juz_number: u8) -> Vec<&AyahData> {
        self.ayah_data
            .values()
            .filter(|ayah| ayah.juz_number == juz_number)
            .collect()
    }

    /// Mendapatkan ayat-ayat dalam halaman
    pub fn get_ayahs_by_page(&self, page_number: u16) -> Vec<&AyahData> {
        self.ayah_data
            .values()
            .filter(|ayah| ayah.page_number == page_number)
            .collect()
    }

    /// Pencarian ayat berdasarkan teks Arab
    pub fn search_ayahs_by_text(&self, query: &str, limit: usize) -> Vec<&AyahData> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for ayah in self.ayah_data.values() {
            if ayah.text_arab.contains(&query_lower) ||
               ayah.text_uthmani.contains(&query_lower) ||
               ayah.translation_id.to_lowercase().contains(&query_lower) ||
               ayah.transliteration.to_lowercase().contains(&query_lower) {
                results.push(ayah);
                if results.len() >= limit {
                    break;
                }
            }
        }

        results
    }

    /// Pencarian ayat serupa menggunakan matching-ayah.json (bidirectional)
    pub fn search_similar_ayahs(&self, verse_key: &str, limit: usize) -> Vec<SimilarAyahWithText> {
        let mut results = Vec::new();

        // Direct matches where verse_key is the source
        if let Some(matches) = self.matching_ayahs.get(verse_key) {
            for match_info in matches {
                if let Some(ayah_data) = self.get_ayah_data(&match_info.matched_ayah_key) {
                    results.push(SimilarAyahWithText {
                        ayah_data: ayah_data.clone(),
                        similarity_score: match_info.score as f64 / 100.0,
                        matching_words: Vec::new(), // We could extract this from match_words if needed
                        matching_type: "direct".to_string(),
                        matched_words_count: match_info.matched_words_count,
                        coverage: match_info.coverage,
                    });
                }
            }
        }

        // Reverse matches where verse_key is the target
        for (source_verse, matches) in &self.matching_ayahs {
            if source_verse == verse_key {
                continue; // Skip self
            }
            
            for match_info in matches {
                if match_info.matched_ayah_key == verse_key {
                    if let Some(ayah_data) = self.get_ayah_data(source_verse) {
                        results.push(SimilarAyahWithText {
                            ayah_data: ayah_data.clone(),
                            similarity_score: match_info.score as f64 / 100.0,
                            matching_words: Vec::new(),
                            matching_type: "reverse".to_string(),
                            matched_words_count: match_info.matched_words_count,
                            coverage: match_info.coverage,
                        });
                    }
                }
            }
        }

        // Sort by similarity score (descending)
        results.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        results.truncate(limit);

        results
    }

    /// Validasi data bundle
    pub fn validate(&self) -> Result<(), String> {
        if self.ayah_data.is_empty() {
            return Err("No ayah data found".to_string());
        }

        if self.surah_info.is_empty() {
            return Err("No surah info found".to_string());
        }

        // Check if all ayahs have valid surah numbers
        for ayah in self.ayah_data.values() {
            if !self.surah_info.contains_key(&ayah.surah_number) {
                return Err(format!("Invalid surah number {} in ayah {}", ayah.surah_number, ayah.verse_key));
            }
        }

        Ok(())
    }
}

impl Default for QuranCoreDataBundle {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for QuranStatistics {
    fn default() -> Self {
        Self {
            total_surahs: 114,
            total_ayahs: 6236,
            total_words: 77430,
            total_letters: 323670,
            total_pages: 604,
            total_juz: 30,
            total_hizb: 60,
            total_ruku: 556,
            total_manzil: 7,
            total_sajda: 15,
        }
    }
}
