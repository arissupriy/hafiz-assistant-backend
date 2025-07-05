// Core Processing untuk Hafiz Assistant Backend
// Menangani inisialisasi data dan operasi dasar

use crate::data::{QuranCoreDataBundle, AyahData, SurahInfo, RenderedPage, DataLoader};
use std::sync::{Arc, Mutex, OnceLock};

// Global data bundle
static GLOBAL_DATA: OnceLock<Arc<Mutex<Option<QuranCoreDataBundle>>>> = OnceLock::new();

/// Inisialisasi data Quran
pub fn initialize_data() -> Result<(), Box<dyn std::error::Error>> {
    let data_mutex = GLOBAL_DATA.get_or_init(|| Arc::new(Mutex::new(None)));
    let mut data_guard = data_mutex.lock().unwrap();
    
    if data_guard.is_some() {
        return Ok(()); // Data sudah diinisialisasi
    }

    // Gunakan DataLoader yang sudah dimodifikasi untuk embedded data
    println!("🔄 Loading embedded Quran data from binary...");
    let bundle = DataLoader::load_all_data()?;
    
    // Validasi data
    bundle.validate().map_err(|e| format!("Data validation failed: {}", e))?;
    
    *data_guard = Some(bundle);
    println!("✅ All embedded data loaded and validated successfully!");
    
    Ok(())
}

/// Mendapatkan data bundle global
pub fn get_data_bundle() -> Option<QuranCoreDataBundle> {
    let data_mutex = GLOBAL_DATA.get()?;
    let data_guard = data_mutex.lock().ok()?;
    data_guard.clone()
}

/// Mendapatkan data ayat berdasarkan verse key
pub fn get_ayah_data(verse_key: &str) -> Option<AyahData> {
    let bundle = get_data_bundle()?;
    bundle.get_ayah_data(verse_key).cloned()
}

/// Mendapatkan informasi surah
pub fn get_surah_info(surah_number: u16) -> Option<SurahInfo> {
    let bundle = get_data_bundle()?;
    bundle.get_surah_info(surah_number).cloned()
}

/// Mendapatkan ayat-ayat dalam surah
pub fn get_ayahs_by_surah(surah_number: u16) -> Vec<AyahData> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    bundle.get_ayahs_by_surah(surah_number)
        .into_iter()
        .cloned()
        .collect()
}

/// Mendapatkan ayat-ayat dalam juz
pub fn get_ayahs_by_juz(juz_number: u8) -> Vec<AyahData> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    bundle.get_ayahs_by_juz(juz_number)
        .into_iter()
        .cloned()
        .collect()
}

/// Mendapatkan ayat-ayat dalam halaman
pub fn get_ayahs_by_page(page_number: u16) -> Vec<AyahData> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    bundle.get_ayahs_by_page(page_number)
        .into_iter()
        .cloned()
        .collect()
}

/// Mendapatkan statistik Quran
pub fn get_quran_statistics() -> Option<crate::data::structures::QuranStatistics> {
    let bundle = get_data_bundle()?;
    Some(bundle.statistics.clone())
}

/// Mendapatkan daftar semua surah
pub fn get_all_surahs() -> Vec<SurahInfo> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    bundle.surah_info.values().cloned().collect()
}

/// Validasi verse key
pub fn validate_verse_key(verse_key: &str) -> bool {
    if let Some(parts) = verse_key.split_once(':') {
        if let (Ok(surah), Ok(ayah)) = (parts.0.parse::<u16>(), parts.1.parse::<u16>()) {
            return surah >= 1 && surah <= 114 && ayah >= 1;
        }
    }
    false
}

/// Mendapatkan ayat acak
pub fn get_random_ayah() -> Option<AyahData> {
    use rand::Rng;
    
    let bundle = get_data_bundle()?;
    let ayahs: Vec<&AyahData> = bundle.ayah_data.values().collect();
    
    if ayahs.is_empty() {
        return None;
    }
    
    let mut rng = rand::rng();
    let index = rng.random_range(0..ayahs.len());
    Some(ayahs[index].clone())
}

/// Reset data (untuk testing)
pub fn reset_data() {
    if let Some(data_mutex) = GLOBAL_DATA.get() {
        let mut data_guard = data_mutex.lock().unwrap();
        *data_guard = None;
    }
}

// Page-related functions

/// Mendapatkan data halaman berdasarkan nomor halaman
pub fn get_page_data(page_number: u16) -> Option<RenderedPage> {
    let bundle = get_data_bundle()?;
    
    bundle.rendered_pages
        .iter()
        .find(|page| page.page_number == page_number)
        .cloned()
}

/// Mendapatkan total jumlah halaman
pub fn get_total_pages() -> u16 {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return 0,
    };
    
    // Get from info table or calculate from rendered pages
    if bundle.rendered_pages.is_empty() {
        604 // Default from QCF V2
    } else {
        bundle.rendered_pages
            .iter()
            .map(|page| page.page_number)
            .max()
            .unwrap_or(604)
    }
}

/// Mendapatkan range halaman
pub fn get_pages_range(start_page: u16, end_page: u16) -> Vec<RenderedPage> {
    let bundle = match get_data_bundle() {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    bundle.rendered_pages
        .iter()
        .filter(|page| page.page_number >= start_page && page.page_number <= end_page)
        .cloned()
        .collect()
}

/// Mendapatkan halaman yang berisi ayat tertentu
pub fn get_page_containing_verse(verse_key: &str) -> Option<u16> {
    let bundle = get_data_bundle()?;
    
    for page in &bundle.rendered_pages {
        for line in &page.lines {
            if line.verse_keys.contains(&verse_key.to_string()) {
                return Some(page.page_number);
            }
        }
    }
    
    None
}

/// Mendapatkan ayat-ayat dalam halaman tertentu
pub fn get_verses_in_page(page_number: u16) -> Vec<String> {
    if let Some(page) = get_page_data(page_number) {
        let mut verses = std::collections::HashSet::new();
        
        for line in &page.lines {
            for verse_key in &line.verse_keys {
                verses.insert(verse_key.clone());
            }
        }
        
        let mut result: Vec<String> = verses.into_iter().collect();
        result.sort_by(|a, b| {
            // Sort by surah:ayah number
            let a_parts: Vec<&str> = a.split(':').collect();
            let b_parts: Vec<&str> = b.split(':').collect();
            
            if a_parts.len() == 2 && b_parts.len() == 2 {
                let a_surah = a_parts[0].parse::<u16>().unwrap_or(0);
                let a_ayah = a_parts[1].parse::<u16>().unwrap_or(0);
                let b_surah = b_parts[0].parse::<u16>().unwrap_or(0);
                let b_ayah = b_parts[1].parse::<u16>().unwrap_or(0);
                
                (a_surah, a_ayah).cmp(&(b_surah, b_ayah))
            } else {
                a.cmp(b)
            }
        });
        result
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_verse_key() {
        assert!(validate_verse_key("1:1"));
        assert!(validate_verse_key("114:6"));
        assert!(!validate_verse_key("115:1"));
        assert!(!validate_verse_key("invalid"));
    }

    #[test]
    fn test_initialize_data() {
        reset_data();
        // Test akan berhasil jika direktori data ada
        let result = initialize_data();
        println!("Initialize data result: {:?}", result);
    }
}
