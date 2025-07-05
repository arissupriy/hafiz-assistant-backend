// Integration test for metadata extraction
use hafiz_assistant_backend::data::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_extraction() {
        let data_loader = DataLoader::new();
        
        // Test with Al-Fatiha verse 1
        if let Some(ayah_data) = data_loader.parse_ayah_data_from_metadata("1:1") {
            println!("Testing verse 1:1:");
            println!("  Surah: {} ({})", ayah_data.surah_name, ayah_data.surah_name_arabic);
            println!("  Juz: {}", ayah_data.juz_number);
            println!("  Hizb: {}", ayah_data.hizb_number);
            println!("  Rub: {}", ayah_data.rub_number);
            println!("  Ruku: {}", ayah_data.ruku_number);
            println!("  Manzil: {}", ayah_data.manzil_number);
            println!("  Page: {}", ayah_data.page_number);
            
            // Basic assertions
            assert_eq!(ayah_data.surah_number, 1);
            assert_eq!(ayah_data.ayah_number, 1);
            assert_eq!(ayah_data.verse_key, "1:1");
            assert!(ayah_data.juz_number >= 1);
            assert!(ayah_data.page_number >= 1);
        } else {
            panic!("Failed to parse metadata for verse 1:1");
        }
        
        // Test with a verse that has sajda
        if let Some(ayah_data) = data_loader.parse_ayah_data_from_metadata("7:206") {
            println!("\nTesting verse 7:206 (should have sajda):");
            println!("  Sajda: {}", if ayah_data.sajda.is_some() { "Yes" } else { "No" });
            if let Some(sajda) = &ayah_data.sajda {
                println!("    Type: {}, Recommended: {}, Obligatory: {}", 
                         sajda.sajda_type, sajda.recommended, sajda.obligatory);
            }
        }
        
        // Test with Ayat al-Kursi
        if let Some(ayah_data) = data_loader.parse_ayah_data_from_metadata("2:255") {
            println!("\nTesting verse 2:255 (Ayat al-Kursi):");
            println!("  Surah: {} ({})", ayah_data.surah_name, ayah_data.surah_name_arabic);
            println!("  Juz: {}", ayah_data.juz_number);
            println!("  Page: {}", ayah_data.page_number);
            
            // Basic assertions for Al-Baqarah
            assert_eq!(ayah_data.surah_number, 2);
            assert_eq!(ayah_data.ayah_number, 255);
            assert_eq!(ayah_data.verse_key, "2:255");
        }
        
        println!("\nAll metadata extraction tests passed!");
    }
}
