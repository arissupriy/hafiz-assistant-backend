// Text Utilities untuk Hafiz Assistant Backend
// Menangani normalisasi dan pemrosesan teks Arab

/// Normalize Arabic text untuk pencarian
pub fn normalize_arabic_text(text: &str) -> String {
    let mut normalized = text.to_string();
    
    // Normalize different forms of Alef
    normalized = normalize_alef_variations(&normalized);
    
    // Remove diacritics (tashkeel)
    normalized = remove_diacritics(&normalized);
    
    // Normalize whitespace
    normalized = normalized.split_whitespace().collect::<Vec<&str>>().join(" ");
    
    normalized
}

/// Normalize variations of Alef
pub fn normalize_alef_variations(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'آ' | 'أ' | 'إ' | 'ا' => 'ا', // Normalize to plain Alef
            _ => c,
        })
        .collect()
}

/// Remove Arabic diacritics (tashkeel)
pub fn remove_diacritics(text: &str) -> String {
    text.chars()
        .filter(|&c| !is_arabic_diacritic(c))
        .collect()
}

/// Check if character is Arabic diacritic
fn is_arabic_diacritic(c: char) -> bool {
    matches!(c,
        '\u{064B}' | // Fathatan
        '\u{064C}' | // Dammatan
        '\u{064D}' | // Kasratan
        '\u{064E}' | // Fatha
        '\u{064F}' | // Damma
        '\u{0650}' | // Kasra
        '\u{0651}' | // Shadda
        '\u{0652}' | // Sukun
        '\u{0653}' | // Maddah
        '\u{0654}' | // Hamza above
        '\u{0655}' | // Hamza below
        '\u{0656}' | // Subscript alef
        '\u{0657}' | // Inverted damma
        '\u{0658}' | // Mark noon ghunna
        '\u{0659}' | // Zwarakay
        '\u{065A}' | // Vowel sign small v above
        '\u{065B}' | // Vowel sign inverted small v above
        '\u{065C}' | // Vowel sign dot below
        '\u{065D}' | // Reversed damma
        '\u{065E}' | // Fatha with two dots
        '\u{065F}' | // Wavy hamza below
        '\u{0670}'   // Superscript alef
    )
}

/// Extract root words from Arabic text (simple implementation)
pub fn extract_root_words(text: &str) -> Vec<String> {
    let normalized = normalize_arabic_text(text);
    normalized
        .split_whitespace()
        .map(|word| word.to_string())
        .collect()
}

/// Check if text contains Arabic characters
pub fn contains_arabic(text: &str) -> bool {
    text.chars().any(|c| is_arabic_letter(c))
}

/// Check if character is Arabic letter
fn is_arabic_letter(c: char) -> bool {
    matches!(c, '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{FB50}'..='\u{FDFF}' | '\u{FE70}'..='\u{FEFF}')
}

/// Convert Arabic-Indic digits to Latin digits
pub fn arabic_to_latin_digits(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '٠' => '0',
            '١' => '1',
            '٢' => '2',
            '٣' => '3',
            '٤' => '4',
            '٥' => '5',
            '٦' => '6',
            '٧' => '7',
            '٨' => '8',
            '٩' => '9',
            _ => c,
        })
        .collect()
}

/// Convert Latin digits to Arabic-Indic digits
pub fn latin_to_arabic_digits(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '0' => '٠',
            '1' => '١',
            '2' => '٢',
            '3' => '٣',
            '4' => '٤',
            '5' => '٥',
            '6' => '٦',
            '7' => '٧',
            '8' => '٨',
            '9' => '٩',
            _ => c,
        })
        .collect()
}

/// Clean text for search (remove extra whitespace, normalize)
pub fn clean_text_for_search(text: &str) -> String {
    let normalized = normalize_arabic_text(text);
    normalized.trim().to_lowercase()
}

/// Extract verse key from text (e.g., "1:1", "Al-Fatihah 1")
pub fn extract_verse_key(text: &str) -> Option<String> {
    // Simple regex-like pattern matching for verse keys
    let trimmed = text.trim();
    
    // Pattern: number:number
    if let Some(colon_pos) = trimmed.find(':') {
        let before_colon = &trimmed[..colon_pos];
        let after_colon = &trimmed[colon_pos + 1..];
        
        if let (Ok(surah), Ok(ayah)) = (before_colon.parse::<u16>(), after_colon.parse::<u16>()) {
            if surah >= 1 && surah <= 114 && ayah >= 1 {
                return Some(format!("{}:{}", surah, ayah));
            }
        }
    }
    
    None
}

/// Format verse key for display
pub fn format_verse_key(surah: u16, ayah: u16) -> String {
    format!("{}:{}", surah, ayah)
}

/// Parse verse key into components
pub fn parse_verse_key(verse_key: &str) -> Option<(u16, u16)> {
    if let Some((surah_str, ayah_str)) = verse_key.split_once(':') {
        if let (Ok(surah), Ok(ayah)) = (surah_str.parse::<u16>(), ayah_str.parse::<u16>()) {
            return Some((surah, ayah));
        }
    }
    None
}

/// Word count for Arabic text
pub fn count_words(text: &str) -> usize {
    normalize_arabic_text(text)
        .split_whitespace()
        .count()
}

/// Character count for Arabic text (excluding diacritics)
pub fn count_characters(text: &str) -> usize {
    remove_diacritics(text)
        .chars()
        .filter(|c| !c.is_whitespace())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_alef_variations() {
        let text = "آمن أحد إله الله";
        let normalized = normalize_alef_variations(text);
        assert_eq!(normalized, "امن احد اله الله");
    }

    #[test]
    fn test_remove_diacritics() {
        let text = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let clean = remove_diacritics(text);
        assert_eq!(clean, "بسم الله الرحمن الرحيم");
    }

    #[test]
    fn test_normalize_arabic_text() {
        let text = "  بِسْمِ   اللَّهِ  ";
        let normalized = normalize_arabic_text(text);
        assert_eq!(normalized, "بسم الله");
    }

    #[test]
    fn test_arabic_to_latin_digits() {
        let text = "سورة ١ آية ٢";
        let converted = arabic_to_latin_digits(text);
        assert_eq!(converted, "سورة 1 آية 2");
    }

    #[test]
    fn test_parse_verse_key() {
        assert_eq!(parse_verse_key("1:1"), Some((1, 1)));
        assert_eq!(parse_verse_key("114:6"), Some((114, 6)));
        assert_eq!(parse_verse_key("invalid"), None);
    }

    #[test]
    fn test_extract_verse_key() {
        assert_eq!(extract_verse_key("1:1"), Some("1:1".to_string()));
        assert_eq!(extract_verse_key("  2:255  "), Some("2:255".to_string()));
        assert_eq!(extract_verse_key("invalid"), None);
    }

    #[test]
    fn test_contains_arabic() {
        assert!(contains_arabic("الله"));
        assert!(contains_arabic("Hello الله"));
        assert!(!contains_arabic("Hello World"));
    }

    #[test]
    fn test_count_words() {
        let text = "بسم الله الرحمن الرحيم";
        assert_eq!(count_words(text), 4); // Correct count is 4 words
    }
}
