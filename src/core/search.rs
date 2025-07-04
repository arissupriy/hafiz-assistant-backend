// Search Module untuk Hafiz Assistant Backend
// Menangani berbagai jenis pencarian ayat

use crate::data::{AyahData, SimilarAyahWithText};
use crate::core::processing::get_data_bundle;
use crate::utils::text::normalize_arabic_text;

/// Pencarian ayat berdasarkan teks
pub fn search_ayahs_by_text(query: &str, limit: usize) -> Vec<AyahData> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    let normalized_query = normalize_arabic_text(query);
    let query_lower = normalized_query.to_lowercase();
    
    bundle.search_ayahs_by_text(&query_lower, limit)
        .into_iter()
        .cloned()
        .collect()
}

/// Pencarian ayat serupa
pub fn search_similar_ayahs(verse_key: &str, limit: usize) -> Vec<SimilarAyahWithText> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    bundle.search_similar_ayahs(verse_key, limit)
}

/// Pencarian ayat berdasarkan kata kunci dalam terjemahan
pub fn search_ayahs_by_translation(query: &str, limit: usize) -> Vec<AyahData> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    
    for ayah in bundle.ayah_data.values() {
        if ayah.translation_id.to_lowercase().contains(&query_lower) {
            results.push(ayah.clone());
            if results.len() >= limit {
                break;
            }
        }
    }
    
    results
}

/// Pencarian ayat berdasarkan transliterasi
pub fn search_ayahs_by_transliteration(query: &str, limit: usize) -> Vec<AyahData> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    
    for ayah in bundle.ayah_data.values() {
        if ayah.transliteration.to_lowercase().contains(&query_lower) {
            results.push(ayah.clone());
            if results.len() >= limit {
                break;
            }
        }
    }
    
    results
}

/// Pencarian ayat berdasarkan range
pub fn search_ayahs_by_range(
    start_verse: &str, 
    end_verse: &str
) -> Vec<AyahData> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    let start_parts: Vec<&str> = start_verse.split(':').collect();
    let end_parts: Vec<&str> = end_verse.split(':').collect();
    
    if start_parts.len() != 2 || end_parts.len() != 2 {
        return Vec::new();
    }
    
    let start_surah = start_parts[0].parse::<u16>().unwrap_or(0);
    let start_ayah = start_parts[1].parse::<u16>().unwrap_or(0);
    let end_surah = end_parts[0].parse::<u16>().unwrap_or(0);
    let end_ayah = end_parts[1].parse::<u16>().unwrap_or(0);
    
    let mut results = Vec::new();
    
    for ayah in bundle.ayah_data.values() {
        let in_range = if start_surah == end_surah {
            // Same surah
            ayah.surah_number == start_surah && 
            ayah.ayah_number >= start_ayah && 
            ayah.ayah_number <= end_ayah
        } else {
            // Different surahs
            (ayah.surah_number == start_surah && ayah.ayah_number >= start_ayah) ||
            (ayah.surah_number == end_surah && ayah.ayah_number <= end_ayah) ||
            (ayah.surah_number > start_surah && ayah.surah_number < end_surah)
        };
        
        if in_range {
            results.push(ayah.clone());
        }
    }
    
    // Sort by surah and ayah number
    results.sort_by(|a, b| {
        a.surah_number.cmp(&b.surah_number)
            .then_with(|| a.ayah_number.cmp(&b.ayah_number))
    });
    
    results
}

/// Pencarian ayat berdasarkan tema (menggunakan phrase_verses)
pub fn search_ayahs_by_theme(theme: &str, limit: usize) -> Vec<AyahData> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    let mut results = Vec::new();
    
    if let Some(verse_ids) = bundle.phrase_verses.get(theme) {
        for verse_id in verse_ids.iter().take(limit) {
            for ayah in bundle.ayah_data.values() {
                if ayah.id == *verse_id {
                    results.push(ayah.clone());
                    break;
                }
            }
        }
    }
    
    results
}

/// Pencarian ayat dengan multiple criteria
pub fn advanced_search(
    text_query: Option<&str>,
    translation_query: Option<&str>,
    surah_number: Option<u16>,
    juz_number: Option<u8>,
    limit: usize
) -> Vec<AyahData> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    let mut results = Vec::new();
    
    for ayah in bundle.ayah_data.values() {
        let mut matches = true;
        
        // Check text query
        if let Some(text) = text_query {
            let normalized_text = normalize_arabic_text(text);
            if !ayah.text_arab.contains(&normalized_text) && 
               !ayah.text_uthmani.contains(&normalized_text) {
                matches = false;
            }
        }
        
        // Check translation query
        if let Some(translation) = translation_query {
            if !ayah.translation_id.to_lowercase().contains(&translation.to_lowercase()) {
                matches = false;
            }
        }
        
        // Check surah number
        if let Some(surah) = surah_number {
            if ayah.surah_number != surah {
                matches = false;
            }
        }
        
        // Check juz number
        if let Some(juz) = juz_number {
            if ayah.juz_number != juz {
                matches = false;
            }
        }
        
        if matches {
            results.push(ayah.clone());
            if results.len() >= limit {
                break;
            }
        }
    }
    
    results
}

/// Pencarian fuzzy untuk menangani typo
pub fn fuzzy_search(query: &str, limit: usize) -> Vec<AyahData> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    let normalized_query = normalize_arabic_text(query);
    let query_words: Vec<&str> = normalized_query.split_whitespace().collect();
    
    let mut scored_results = Vec::new();
    
    for ayah in bundle.ayah_data.values() {
        let ayah_words: Vec<&str> = ayah.text_arab.split_whitespace().collect();
        let score = calculate_fuzzy_score(&query_words, &ayah_words);
        
        if score > 0.0 {
            scored_results.push((ayah.clone(), score));
        }
    }
    
    // Sort by score (descending)
    scored_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    scored_results.into_iter()
        .take(limit)
        .map(|(ayah, _)| ayah)
        .collect()
}

/// Menghitung skor fuzzy matching
fn calculate_fuzzy_score(query_words: &[&str], ayah_words: &[&str]) -> f64 {
    let mut total_score = 0.0;
    let mut matches = 0;
    
    for query_word in query_words {
        let mut best_score = 0.0;
        
        for ayah_word in ayah_words {
            let score = string_similarity(query_word, ayah_word);
            if score > best_score {
                best_score = score;
            }
        }
        
        if best_score > 0.6 { // Threshold untuk dianggap match
            total_score += best_score;
            matches += 1;
        }
    }
    
    if matches == 0 {
        0.0
    } else {
        total_score / matches as f64
    }
}

/// Menghitung similarity antara dua string
fn string_similarity(s1: &str, s2: &str) -> f64 {
    let len1 = s1.len();
    let len2 = s2.len();
    
    if len1 == 0 || len2 == 0 {
        return 0.0;
    }
    
    let max_len = len1.max(len2);
    let distance = levenshtein_distance(s1, s2);
    
    1.0 - (distance as f64 / max_len as f64)
}

/// Menghitung Levenshtein distance
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let len1 = chars1.len();
    let len2 = chars2.len();
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    
    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_similarity() {
        assert!(string_similarity("hello", "hello") > 0.9);
        assert!(string_similarity("hello", "helo") > 0.7);
        assert!(string_similarity("hello", "world") < 0.3);
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
        assert_eq!(levenshtein_distance("hello", "helo"), 1);
        assert_eq!(levenshtein_distance("hello", "world"), 4);
    }
}
