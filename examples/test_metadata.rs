// Test program to verify metadata extraction implementation
use hafiz_assistant_backend::data::*;

fn main() {
    println!("Testing Hafiz Assistant Metadata Extraction...\n");
    
    // Create a data loader instance
    let data_loader = DataLoader::new();
    
    // Test with a few representative verse keys
    let test_verses = vec![
        "1:1",    // Al-Fatiha, first verse
        "2:1",    // Al-Baqarah, first verse
        "2:255",  // Ayat al-Kursi
        "18:1",   // Al-Kahf, first verse
        "114:6",  // An-Nas, last verse of Quran
    ];
    
    for verse_key in test_verses {
        println!("Testing verse: {}", verse_key);
        
        match data_loader.parse_ayah_data_from_metadata(verse_key) {
            Some(ayah_data) => {
                println!("  ✓ Successfully parsed metadata:");
                println!("    Surah: {} ({})", ayah_data.surah_name, ayah_data.surah_name_arabic);
                println!("    Juz: {}", ayah_data.juz_number);
                println!("    Hizb: {}", ayah_data.hizb_number);
                println!("    Rub: {}", ayah_data.rub_number);
                println!("    Ruku: {}", ayah_data.ruku_number);
                println!("    Manzil: {}", ayah_data.manzil_number);
                println!("    Page: {}", ayah_data.page_number);
                println!("    Sajda: {}", if ayah_data.sajda.is_some() { "Yes" } else { "No" });
                if let Some(sajda) = &ayah_data.sajda {
                    println!("      Type: {}, Recommended: {}, Obligatory: {}", 
                             sajda.sajda_type, sajda.recommended, sajda.obligatory);
                }
            }
            None => {
                println!("  ✗ Failed to parse metadata for verse {}", verse_key);
            }
        }
        println!();
    }
    
    println!("Metadata extraction test completed!");
}
