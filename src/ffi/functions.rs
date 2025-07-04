// FFI Functions untuk Hafiz Assistant Backend
// Fungsi-fungsi yang dapat dipanggil dari bahasa lain

use crate::ffi::structures::*;
use crate::data::structures::*;
use crate::core::{processing, search};
use std::ffi::{CString, CStr, c_char};
use std::ptr;

/// Inisialisasi data Quran
#[no_mangle]
pub extern "C" fn initialize_data_ffi() -> bool {
    match processing::initialize_data() {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Mendapatkan data ayat berdasarkan verse key
#[no_mangle]
pub extern "C" fn get_ayah_data_ffi(verse_key: *const c_char) -> *mut AyahDataFFI {
    if verse_key.is_null() {
        return ptr::null_mut();
    }
    
    let verse_key_str = unsafe {
        match CStr::from_ptr(verse_key).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };
    
    match processing::get_ayah_data(verse_key_str) {
        Some(ayah) => convert_ayah_to_ffi(ayah),
        None => ptr::null_mut(),
    }
}

/// Membebaskan memori untuk AyahDataFFI
#[no_mangle]
pub extern "C" fn free_ayah_data_ffi(data: *mut AyahDataFFI) {
    if data.is_null() {
        return;
    }
    
    unsafe {
        let ayah_data = Box::from_raw(data);
        free_c_string(ayah_data.verse_key);
        free_c_string(ayah_data.arabic_text);
        free_c_string(ayah_data.translation);
        free_c_string(ayah_data.transliteration);
        free_c_string(ayah_data.surah_name);
        free_c_string(ayah_data.surah_name_arabic);
    }
}

/// Pencarian ayat berdasarkan teks
#[no_mangle]
pub extern "C" fn search_ayahs_by_text_ffi(
    query: *const c_char,
    limit: usize,
    result_count: *mut usize,
) -> *mut AyahDataFFI {
    if query.is_null() || result_count.is_null() {
        return ptr::null_mut();
    }
    
    let query_str = unsafe {
        match CStr::from_ptr(query).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };
    
    let results = search::search_ayahs_by_text(query_str, limit);
    let count = results.len();
    
    unsafe {
        *result_count = count;
    }
    
    if count == 0 {
        return ptr::null_mut();
    }
    
    convert_ayah_array_to_ffi(results)
}

/// Pencarian ayat serupa
#[no_mangle]
pub extern "C" fn search_similar_ayahs_ffi(
    verse_key: *const c_char,
    limit: usize,
    result_count: *mut usize,
) -> *mut SimilarAyahWithTextFFI {
    if verse_key.is_null() || result_count.is_null() {
        return ptr::null_mut();
    }
    
    let verse_key_str = unsafe {
        match CStr::from_ptr(verse_key).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };
    
    let results = search::search_similar_ayahs(verse_key_str, limit);
    let count = results.len();
    
    unsafe {
        *result_count = count;
    }
    
    if count == 0 {
        return ptr::null_mut();
    }
    
    convert_similar_ayah_array_to_ffi(results)
}

/// Membebaskan memori untuk array SimilarAyahWithTextFFI
#[no_mangle]
pub extern "C" fn free_similar_ayahs_ffi(data: *mut SimilarAyahWithTextFFI, count: usize) {
    if data.is_null() {
        return;
    }
    
    unsafe {
        for i in 0..count {
            let item = data.add(i);
            free_c_string((*item).verse_key);
            free_c_string((*item).arabic_text);
            free_c_string((*item).translation);
            free_c_string((*item).transliteration);
        }
        drop(Box::from_raw(std::slice::from_raw_parts_mut(data, count)));
    }
}

/// Mendapatkan informasi surah
#[no_mangle]
pub extern "C" fn get_surah_info_ffi(surah_number: u16) -> *mut SurahInfoFFI {
    match processing::get_surah_info(surah_number) {
        Some(surah) => convert_surah_to_ffi(surah),
        None => ptr::null_mut(),
    }
}

/// Membebaskan memori untuk SurahInfoFFI
#[no_mangle]
pub extern "C" fn free_surah_info_ffi(data: *mut SurahInfoFFI) {
    if data.is_null() {
        return;
    }
    
    unsafe {
        let surah_info = Box::from_raw(data);
        free_c_string(surah_info.name_simple);
        free_c_string(surah_info.name_arabic);
        free_c_string(surah_info.name_english);
        free_c_string(surah_info.revelation_place);
    }
}

/// Mendapatkan ayat-ayat dalam surah
#[no_mangle]
pub extern "C" fn get_ayahs_by_surah_ffi(
    surah_number: u16,
    result_count: *mut usize,
) -> *mut AyahDataFFI {
    if result_count.is_null() {
        return ptr::null_mut();
    }
    
    let results = processing::get_ayahs_by_surah(surah_number);
    let count = results.len();
    
    unsafe {
        *result_count = count;
    }
    
    if count == 0 {
        return ptr::null_mut();
    }
    
    convert_ayah_array_to_ffi(results)
}

/// Mendapatkan ayat-ayat dalam juz
#[no_mangle]
pub extern "C" fn get_ayahs_by_juz_ffi(
    juz_number: u8,
    result_count: *mut usize,
) -> *mut AyahDataFFI {
    if result_count.is_null() {
        return ptr::null_mut();
    }
    
    let results = processing::get_ayahs_by_juz(juz_number);
    let count = results.len();
    
    unsafe {
        *result_count = count;
    }
    
    if count == 0 {
        return ptr::null_mut();
    }
    
    convert_ayah_array_to_ffi(results)
}

/// Mendapatkan ayat-ayat dalam halaman
#[no_mangle]
pub extern "C" fn get_ayahs_by_page_ffi(
    page_number: u16,
    result_count: *mut usize,
) -> *mut AyahDataFFI {
    if result_count.is_null() {
        return ptr::null_mut();
    }
    
    let results = processing::get_ayahs_by_page(page_number);
    let count = results.len();
    
    unsafe {
        *result_count = count;
    }
    
    if count == 0 {
        return ptr::null_mut();
    }
    
    convert_ayah_array_to_ffi(results)
}

/// Pencarian lanjutan dengan multiple criteria
#[no_mangle]
pub extern "C" fn advanced_search_ffi(
    criteria: *const AdvancedSearchCriteriaFFI,
    result_count: *mut usize,
) -> *mut AyahDataFFI {
    if criteria.is_null() || result_count.is_null() {
        return ptr::null_mut();
    }
    
    let criteria_data = unsafe { &*criteria };
    
    let text_query = if criteria_data.text_query.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(criteria_data.text_query).to_str().ok() }
    };
    
    let translation_query = if criteria_data.translation_query.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(criteria_data.translation_query).to_str().ok() }
    };
    
    let surah_number = if criteria_data.surah_number == 0 {
        None
    } else {
        Some(criteria_data.surah_number)
    };
    
    let juz_number = if criteria_data.juz_number == 0 {
        None
    } else {
        Some(criteria_data.juz_number)
    };
    
    let results = search::advanced_search(
        text_query,
        translation_query,
        surah_number,
        juz_number,
        criteria_data.limit,
    );
    
    let count = results.len();
    
    unsafe {
        *result_count = count;
    }
    
    if count == 0 {
        return ptr::null_mut();
    }
    
    convert_ayah_array_to_ffi(results)
}

/// Membebaskan memori untuk array AyahDataFFI
#[no_mangle]
pub extern "C" fn free_ayah_array_ffi(data: *mut AyahDataFFI, count: usize) {
    if data.is_null() {
        return;
    }
    
    unsafe {
        for i in 0..count {
            let item = data.add(i);
            free_c_string((*item).verse_key);
            free_c_string((*item).arabic_text);
            free_c_string((*item).translation);
            free_c_string((*item).transliteration);
            free_c_string((*item).surah_name);
            free_c_string((*item).surah_name_arabic);
        }
        drop(Box::from_raw(std::slice::from_raw_parts_mut(data, count)));
    }
}

/// Mendapatkan statistik Quran
#[no_mangle]
pub extern "C" fn get_quran_statistics_ffi() -> *mut QuranStatisticsFFI {
    match processing::get_quran_statistics() {
        Some(stats) => {
            let ffi_stats = Box::new(QuranStatisticsFFI {
                total_surahs: stats.total_surahs,
                total_ayahs: stats.total_ayahs,
                total_words: stats.total_words,
                total_letters: stats.total_letters,
                total_pages: stats.total_pages,
                total_juz: stats.total_juz,
                total_hizb: stats.total_hizb,
                total_ruku: stats.total_ruku,
                total_manzil: stats.total_manzil,
                total_sajda: stats.total_sajda,
            });
            Box::into_raw(ffi_stats)
        }
        None => ptr::null_mut(),
    }
}

/// Membebaskan memori untuk QuranStatisticsFFI
#[no_mangle]
pub extern "C" fn free_quran_statistics_ffi(data: *mut QuranStatisticsFFI) {
    if data.is_null() {
        return;
    }
    
    unsafe {
        let _ = Box::from_raw(data);
    }
}

/// Validasi verse key
#[no_mangle]
pub extern "C" fn validate_verse_key_ffi(verse_key: *const c_char) -> bool {
    if verse_key.is_null() {
        return false;
    }
    
    let verse_key_str = unsafe {
        match CStr::from_ptr(verse_key).to_str() {
            Ok(s) => s,
            Err(_) => return false,
        }
    };
    
    processing::validate_verse_key(verse_key_str)
}

/// Mendapatkan ayat acak
#[no_mangle]
pub extern "C" fn get_random_ayah_ffi() -> *mut AyahDataFFI {
    match processing::get_random_ayah() {
        Some(ayah) => convert_ayah_to_ffi(ayah),
        None => ptr::null_mut(),
    }
}

// Helper functions

/// Konversi AyahData ke AyahDataFFI
fn convert_ayah_to_ffi(ayah: AyahData) -> *mut AyahDataFFI {
    let ffi_ayah = Box::new(AyahDataFFI {
        verse_key: string_to_c_string(&ayah.verse_key),
        arabic_text: string_to_c_string(&ayah.text_arab),
        translation: string_to_c_string(&ayah.translation_id),
        transliteration: string_to_c_string(&ayah.transliteration),
        surah_name: string_to_c_string(&ayah.surah_name),
        surah_name_arabic: string_to_c_string(&ayah.surah_name_arabic),
        surah_number: ayah.surah_number,
        ayah_number: ayah.ayah_number,
        juz_number: ayah.juz_number,
        hizb_number: ayah.hizb_number,
        rub_number: ayah.rub_number,
        ruku_number: ayah.ruku_number,
        manzil_number: ayah.manzil_number,
        page_number: ayah.page_number,
    });
    
    Box::into_raw(ffi_ayah)
}

/// Konversi SurahInfo ke SurahInfoFFI
fn convert_surah_to_ffi(surah: SurahInfo) -> *mut SurahInfoFFI {
    let ffi_surah = Box::new(SurahInfoFFI {
        id: surah.id,
        name_simple: string_to_c_string(&surah.name_simple),
        name_arabic: string_to_c_string(&surah.name_arabic),
        name_english: string_to_c_string(&surah.name_english),
        revelation_order: surah.revelation_order,
        revelation_place: string_to_c_string(&surah.revelation_place),
        verses_count: surah.verses_count,
        bismillah_pre: surah.bismillah_pre,
    });
    
    Box::into_raw(ffi_surah)
}

/// Konversi array AyahData ke array AyahDataFFI
fn convert_ayah_array_to_ffi(ayahs: Vec<AyahData>) -> *mut AyahDataFFI {
    let mut ffi_ayahs = Vec::with_capacity(ayahs.len());
    
    for ayah in ayahs {
        ffi_ayahs.push(AyahDataFFI {
            verse_key: string_to_c_string(&ayah.verse_key),
            arabic_text: string_to_c_string(&ayah.text_arab),
            translation: string_to_c_string(&ayah.translation_id),
            transliteration: string_to_c_string(&ayah.transliteration),
            surah_name: string_to_c_string(&ayah.surah_name),
            surah_name_arabic: string_to_c_string(&ayah.surah_name_arabic),
            surah_number: ayah.surah_number,
            ayah_number: ayah.ayah_number,
            juz_number: ayah.juz_number,
            hizb_number: ayah.hizb_number,
            rub_number: ayah.rub_number,
            ruku_number: ayah.ruku_number,
            manzil_number: ayah.manzil_number,
            page_number: ayah.page_number,
        });
    }
    
    let boxed_array = ffi_ayahs.into_boxed_slice();
    Box::into_raw(boxed_array) as *mut AyahDataFFI
}

/// Konversi array SimilarAyahWithText ke array SimilarAyahWithTextFFI
fn convert_similar_ayah_array_to_ffi(ayahs: Vec<SimilarAyahWithText>) -> *mut SimilarAyahWithTextFFI {
    let mut ffi_ayahs = Vec::with_capacity(ayahs.len());
    
    for ayah in ayahs {
        ffi_ayahs.push(SimilarAyahWithTextFFI {
            verse_key: string_to_c_string(&ayah.ayah_data.verse_key),
            arabic_text: string_to_c_string(&ayah.ayah_data.text_arab),
            translation: string_to_c_string(&ayah.ayah_data.translation_id),
            transliteration: string_to_c_string(&ayah.ayah_data.transliteration),
            similarity_score: ayah.similarity_score,
            surah_number: ayah.ayah_data.surah_number,
            ayah_number: ayah.ayah_data.ayah_number,
        });
    }
    
    let boxed_array = ffi_ayahs.into_boxed_slice();
    Box::into_raw(boxed_array) as *mut SimilarAyahWithTextFFI
}

/// Konversi String ke C string
fn string_to_c_string(s: &str) -> *mut c_char {
    match CString::new(s) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Membebaskan C string
fn free_c_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
