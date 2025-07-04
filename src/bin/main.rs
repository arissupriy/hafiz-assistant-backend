// CLI untuk Hafiz Assistant Backend
// Aplikasi command line untuk testing dan demonstrasi fitur

use hafiz_assistant_core::*;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🕌 Hafiz Assistant Backend CLI v1.0.0");
    println!("=====================================");
    
    // Initialize data
    print!("⏳ Initializing Quran data...");
    io::stdout().flush()?;
    
    match initialize_data() {
        Ok(_) => println!(" ✅ Success!"),
        Err(e) => {
            println!(" ❌ Failed!");
            println!("Error: {}", e);
            println!("\n💡 Make sure the 'data' directory exists and contains the required JSON files.");
            return Ok(());
        }
    }
    
    // Show main menu
    loop {
        show_menu();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let choice = input.trim();
                
                if choice.is_empty() {
                    println!("❌ Please enter a valid choice (1-10).");
                    continue;
                }
                
                match choice {
                    "1" => test_ayah_data(),
                    "2" => test_search_features(),
                    "3" => test_surah_operations(),
                    "4" => test_navigation_features(),
                    "5" => test_page_functions(),
                    "6" => test_ffi_functions(),
                    "7" => show_statistics(),
                    "8" => interactive_search(),
                    "9" => interactive_page_viewer(),
                    "10" => {
                        println!("👋 Goodbye!");
                        break;
                    }
                    _ => println!("❌ Invalid choice '{}'. Please enter 1-10.", choice),
                }
            }
            Err(e) => {
                println!("❌ Error reading input: {}", e);
                continue;
            }
        }
        
        println!("\nPress Enter to continue...");
        let mut dummy = String::new();
        let _ = io::stdin().read_line(&mut dummy);
    }
    
    Ok(())
}

fn show_menu() {
    println!("\n📋 MAIN MENU");
    println!("1. Test Ayah Data Retrieval");
    println!("2. Test Search Features");
    println!("3. Test Surah Operations");
    println!("4. Test Navigation Features");
    println!("5. Test Page (Mushaf) Functions");
    println!("6. Test FFI Functions");
    println!("7. Show Quran Statistics");
    println!("8. Interactive Search");
    println!("9. Interactive Page Viewer");
    println!("10. Exit");
    print!("\nChoose an option (1-10): ");
    io::stdout().flush().unwrap();
}

fn test_ayah_data() {
    println!("\n🔍 TESTING AYAH DATA RETRIEVAL");
    println!("================================");
    
    let test_verses = vec!["1:1", "2:255", "112:1", "114:6"];
    
    for verse_key in test_verses {
        println!("\n📖 Testing verse: {}", verse_key);
        
        match get_ayah_data(verse_key) {
            Some(ayah) => {
                println!("  ✅ Success!");
                println!("  🔤 Arabic: {}", ayah.text_arab);
                println!("  🌍 Translation: {}", ayah.translation_id);
                println!("  📝 Transliteration: {}", ayah.transliteration);
                println!("  📚 Surah: {} ({})", ayah.surah_name, ayah.surah_name_arabic);
                println!("  📄 Location: Juz {}, Page {}", ayah.juz_number, ayah.page_number);
            }
            None => println!("  ❌ Failed to retrieve ayah data"),
        }
    }
}

fn test_search_features() {
    println!("\n🔍 TESTING SEARCH FEATURES");
    println!("===========================");
    
    // Test text search
    println!("\n1. Text Search for 'الله':");
    let text_results = search_ayahs_by_text("الله", 3);
    println!("   Found {} results", text_results.len());
    for (i, ayah) in text_results.iter().enumerate() {
        println!("   {}. {} - {}", i + 1, ayah.verse_key, ayah.text_arab);
    }
    
    // Test similarity search
    println!("\n2. Similar Ayahs to '1:1':");
    let similar_results = search_similar_ayahs("1:1", 3);
    println!("   Found {} similar ayahs", similar_results.len());
    for (i, similar) in similar_results.iter().enumerate() {
        println!("   {}. {} - Similarity: {:.2}%", 
                i + 1, 
                similar.ayah_data.verse_key, 
                similar.similarity_score * 100.0);
    }
    
    // Test advanced search
    println!("\n3. Advanced Search (Surah 1):");
    let advanced_results = core::search::advanced_search(
        Some("بسم"),
        None,
        Some(1),
        None,
        5
    );
    println!("   Found {} results", advanced_results.len());
    for (i, ayah) in advanced_results.iter().enumerate() {
        println!("   {}. {} - {}", i + 1, ayah.verse_key, ayah.text_arab);
    }
}

fn test_surah_operations() {
    println!("\n📚 TESTING SURAH OPERATIONS");
    println!("============================");
    
    // Test surah info
    println!("\n1. Surah Info for Al-Fatihah (1):");
    match get_surah_info(1) {
        Some(surah) => {
            println!("   ✅ Success!");
            println!("   📚 Name: {} ({})", surah.name_simple, surah.name_arabic);
            println!("   📍 Place: {}", surah.revelation_place);
            println!("   🔢 Verses: {}", surah.verses_count);
            println!("   📜 Revelation Order: {}", surah.revelation_order);
        }
        None => println!("   ❌ Failed to get surah info"),
    }
    
    // Test get ayahs by surah
    println!("\n2. Ayahs in Surah 112 (Al-Ikhlas):");
    let surah_ayahs = get_ayahs_by_surah(112);
    println!("   Found {} ayahs", surah_ayahs.len());
    for ayah in surah_ayahs.iter().take(3) {
        println!("   {} - {}", ayah.verse_key, ayah.text_arab);
    }
}

fn test_navigation_features() {
    println!("\n🧭 TESTING NAVIGATION FEATURES");
    println!("===============================");
    
    // Test by juz
    println!("\n1. Ayahs in Juz 30:");
    let juz_ayahs = get_ayahs_by_juz(30);
    println!("   Found {} ayahs", juz_ayahs.len());
    for ayah in juz_ayahs.iter().take(3) {
        println!("   {} - {}", ayah.verse_key, ayah.text_arab);
    }
    
    // Test by page
    println!("\n2. Ayahs in Page 1:");
    let page_ayahs = get_ayahs_by_page(1);
    println!("   Found {} ayahs", page_ayahs.len());
    for ayah in page_ayahs.iter().take(3) {
        println!("   {} - {}", ayah.verse_key, ayah.text_arab);
    }
    
    // Test random ayah
    println!("\n3. Random Ayah:");
    match get_random_ayah() {
        Some(ayah) => {
            println!("   📖 {}: {}", ayah.verse_key, ayah.text_arab);
            println!("   📚 From: {}", ayah.surah_name);
        }
        None => println!("   ❌ Failed to get random ayah"),
    }
}

fn test_ffi_functions() {
    println!("\n🔗 TESTING FFI FUNCTIONS");
    println!("=========================");
    
    use std::ffi::CString;
    
    // Test FFI initialization
    println!("\n1. FFI Initialization:");
    let init_result = ffi::functions::initialize_data_ffi();
    println!("   Result: {}", if init_result { "✅ Success" } else { "❌ Failed" });
    
    // Test FFI get ayah data
    println!("\n2. FFI Get Ayah Data:");
    let verse_key = CString::new("1:1").unwrap();
    let ayah_ptr = ffi::functions::get_ayah_data_ffi(verse_key.as_ptr());
    if !ayah_ptr.is_null() {
        println!("   ✅ FFI get_ayah_data returned valid pointer");
        ffi::functions::free_ayah_data_ffi(ayah_ptr);
        println!("   ✅ Memory freed successfully");
    } else {
        println!("   ❌ FFI get_ayah_data returned null pointer");
    }
    
    // Test FFI search
    println!("\n3. FFI Search:");
    let query = CString::new("الله").unwrap();
    let mut count: usize = 0;
    let results_ptr = ffi::functions::search_ayahs_by_text_ffi(query.as_ptr(), 3, &mut count as *mut usize);
    if !results_ptr.is_null() {
        println!("   ✅ FFI search returned valid pointer");
        println!("   📊 Found {} results", count);
        ffi::functions::free_ayah_array_ffi(results_ptr, count);
        println!("   ✅ Search results freed successfully");
    } else {
        println!("   ❌ FFI search returned null pointer");
    }
}

fn test_page_functions() {
    println!("\n📄 TESTING PAGE (MUSHAF) FUNCTIONS");
    println!("====================================");
    
    // Test total pages
    println!("\n1. Total Pages:");
    let total_pages = get_total_pages();
    println!("   Total pages: {}", total_pages);
    
    // Test specific pages
    println!("\n2. Testing Specific Pages:");
    let test_pages = vec![1, 2, 100, 604];
    
    for page_number in test_pages {
        println!("\n📄 Testing page: {}", page_number);
        
        match get_page_data(page_number) {
            Some(page) => {
                println!("   ✅ Page data found");
                println!("   📊 Lines: {}", page.lines.len());
                println!("   🎯 Surah headers: {}", page.surah_headers.len());
                
                // Show first few lines
                for (_i, line) in page.lines.iter().take(3).enumerate() {
                    println!("   Line {}: {} ({})", 
                        line.line_number, 
                        line.line_type,
                        if line.is_centered { "centered" } else { "aligned" }
                    );
                    if !line.text.is_empty() {
                        println!("     Text: {}", &line.text[..line.text.len().min(50)]);
                    }
                    if !line.verse_keys.is_empty() {
                        println!("     Verses: {:?}", line.verse_keys);
                    }
                }
                
                if page.lines.len() > 3 {
                    println!("     ... and {} more lines", page.lines.len() - 3);
                }
                
                // Show surah headers
                for header in &page.surah_headers {
                    println!("   📚 Surah {}: {} ({})", 
                        header.surah_number, 
                        header.name_simple, 
                        header.name_arabic
                    );
                }
            }
            None => println!("   ❌ Page data not found"),
        }
    }
    
    // Test page range
    println!("\n3. Testing Page Range (1-3):");
    let pages = get_pages_range(1, 3);
    println!("   Found {} pages in range", pages.len());
    
    for page in pages {
        println!("   Page {}: {} lines, {} surah headers", 
            page.page_number, 
            page.lines.len(), 
            page.surah_headers.len()
        );
    }
    
    // Test verse to page mapping
    println!("\n4. Testing Verse to Page Mapping:");
    let test_verses = vec!["1:1", "2:255", "18:10", "114:6"];
    
    for verse_key in test_verses {
        match get_page_containing_verse(verse_key) {
            Some(page_number) => {
                println!("   {} is on page {}", verse_key, page_number);
                
                // Show verses in that page
                let verses_in_page = get_verses_in_page(page_number);
                println!("     Page {} contains {} verses", page_number, verses_in_page.len());
                if !verses_in_page.is_empty() {
                    println!("     First few: {:?}", &verses_in_page[..verses_in_page.len().min(5)]);
                }
            }
            None => println!("   {} not found in any page", verse_key),
        }
    }
}

fn show_statistics() {
    println!("\n📊 QURAN STATISTICS");
    println!("====================");
    
    match get_quran_statistics() {
        Some(stats) => {
            println!("📚 Total Surahs: {}", stats.total_surahs);
            println!("📖 Total Ayahs: {}", stats.total_ayahs);
            println!("💬 Total Words: {}", stats.total_words);
            println!("🔤 Total Letters: {}", stats.total_letters);
            println!("📄 Total Pages: {}", stats.total_pages);
            println!("📗 Total Juz: {}", stats.total_juz);
            println!("📘 Total Hizb: {}", stats.total_hizb);
            println!("📙 Total Ruku: {}", stats.total_ruku);
            println!("📔 Total Manzil: {}", stats.total_manzil);
            println!("🙏 Total Sajda: {}", stats.total_sajda);
        }
        None => println!("❌ Failed to get statistics"),
    }
}

fn interactive_search() {
    println!("\n🔍 INTERACTIVE SEARCH");
    println!("======================");
    
    loop {
        println!("\n1. Search by Arabic text");
        println!("2. Search by translation");
        println!("3. Search similar ayahs");
        println!("4. Advanced search");
        println!("5. Return to main menu");
        print!("\nChoose search type (1-5): ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => {
                print!("Enter Arabic text to search: ");
                io::stdout().flush().unwrap();
                let mut query = String::new();
                io::stdin().read_line(&mut query).unwrap();
                
                let results = search_ayahs_by_text(query.trim(), 10);
                display_search_results(&results);
            }
            "2" => {
                print!("Enter translation text to search: ");
                io::stdout().flush().unwrap();
                let mut query = String::new();
                io::stdin().read_line(&mut query).unwrap();
                
                let results = core::search::search_ayahs_by_translation(query.trim(), 10);
                display_search_results(&results);
            }
            "3" => {
                print!("Enter verse key (e.g., 1:1): ");
                io::stdout().flush().unwrap();
                let mut verse_key = String::new();
                io::stdin().read_line(&mut verse_key).unwrap();
                
                let results = search_similar_ayahs(verse_key.trim(), 10);
                display_similar_results(&results);
            }
            "4" => {
                println!("Advanced Search:");
                print!("Arabic text (optional): ");
                io::stdout().flush().unwrap();
                let mut text_query = String::new();
                io::stdin().read_line(&mut text_query).unwrap();
                
                print!("Translation text (optional): ");
                io::stdout().flush().unwrap();
                let mut trans_query = String::new();
                io::stdin().read_line(&mut trans_query).unwrap();
                
                print!("Surah number (optional): ");
                io::stdout().flush().unwrap();
                let mut surah_input = String::new();
                io::stdin().read_line(&mut surah_input).unwrap();
                
                let text_q = if text_query.trim().is_empty() { None } else { Some(text_query.trim()) };
                let trans_q = if trans_query.trim().is_empty() { None } else { Some(trans_query.trim()) };
                let surah_num = surah_input.trim().parse::<u16>().ok();
                
                let results = core::search::advanced_search(text_q, trans_q, surah_num, None, 10);
                display_search_results(&results);
            }
            "5" => break,
            _ => println!("❌ Invalid choice"),
        }
    }
}

fn display_search_results(results: &[AyahData]) {
    if results.is_empty() {
        println!("❌ No results found");
        return;
    }
    
    println!("\n📋 Search Results ({} found):", results.len());
    println!("{}", "=".repeat(50));
    
    for (i, ayah) in results.iter().enumerate() {
        println!("\n{}. 📖 {}", i + 1, ayah.verse_key);
        println!("   🔤 Arabic: {}", ayah.text_arab);
        println!("   🌍 Translation: {}", ayah.translation_id);
        println!("   📚 Surah: {}", ayah.surah_name);
    }
}

fn display_similar_results(results: &[SimilarAyahWithText]) {
    if results.is_empty() {
        println!("❌ No similar ayahs found");
        return;
    }
    
    println!("\n📋 Similar Ayahs ({} found):", results.len());
    println!("{}", "=".repeat(50));
    
    for (i, similar) in results.iter().enumerate() {
        println!("\n{}. 📖 {} (Similarity: {:.1}%)", 
                i + 1, 
                similar.ayah_data.verse_key, 
                similar.similarity_score * 100.0);
        println!("   🔤 Arabic: {}", similar.ayah_data.text_arab);
        println!("   🌍 Translation: {}", similar.ayah_data.translation_id);
        println!("   🎯 Match Type: {}", similar.matching_type);
    }
}

fn interactive_page_viewer() {
    println!("\n📖 INTERACTIVE PAGE VIEWER (MUSHAF)");
    println!("====================================");
    
    let total_pages = get_total_pages();
    println!("📊 Total Pages: {}", total_pages);
    
    let mut current_page = 1u16;
    
    loop {
        println!("\n{}", "=".repeat(60));
        println!("📄 Current Page: {} / {}", current_page, total_pages);
        println!("{}", "=".repeat(60));
        
        // Display current page
        display_page_detailed(current_page);
        
        println!("\n🎮 Navigation Options:");
        println!("  [n] Next page");
        println!("  [p] Previous page");
        println!("  [g] Go to specific page");
        println!("  [f] Find verse (go to page containing verse)");
        println!("  [r] Random page");
        println!("  [s] Search within current page");
        println!("  [h] Help");
        println!("  [q] Quit to main menu");
        
        print!("\nEnter command: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let command = input.trim().to_lowercase();
            
            match command.as_str() {
                "n" | "next" => {
                    if current_page < total_pages {
                        current_page += 1;
                    } else {
                        println!("❌ Already at last page!");
                    }
                }
                "p" | "prev" | "previous" => {
                    if current_page > 1 {
                        current_page -= 1;
                    } else {
                        println!("❌ Already at first page!");
                    }
                }
                "g" | "goto" => {
                    print!("Enter page number (1-{}): ", total_pages);
                    io::stdout().flush().unwrap();
                    let mut page_input = String::new();
                    if io::stdin().read_line(&mut page_input).is_ok() {
                        if let Ok(page_num) = page_input.trim().parse::<u16>() {
                            if page_num >= 1 && page_num <= total_pages {
                                current_page = page_num;
                            } else {
                                println!("❌ Invalid page number! Must be between 1 and {}", total_pages);
                            }
                        } else {
                            println!("❌ Invalid input! Please enter a number.");
                        }
                    }
                }
                "f" | "find" => {
                    print!("Enter verse (format: surah:ayah, e.g., 2:255): ");
                    io::stdout().flush().unwrap();
                    let mut verse_input = String::new();
                    if io::stdin().read_line(&mut verse_input).is_ok() {
                        let verse_key = verse_input.trim();
                        if let Some(page) = get_page_containing_verse(verse_key) {
                            current_page = page;
                            println!("✅ Found! Verse {} is on page {}", verse_key, page);
                        } else {
                            println!("❌ Verse {} not found!", verse_key);
                        }
                    }
                }
                "r" | "random" => {
                    current_page = (rand::random::<u16>() % total_pages) + 1;
                    println!("🎲 Random page: {}", current_page);
                }
                "s" | "search" => {
                    search_within_page(current_page);
                }
                "h" | "help" => {
                    show_page_viewer_help();
                }
                "q" | "quit" | "exit" => {
                    println!("👋 Returning to main menu...");
                    break;
                }
                // Quick navigation with numbers
                num_str if num_str.parse::<u16>().is_ok() => {
                    let page_num = num_str.parse::<u16>().unwrap();
                    if page_num >= 1 && page_num <= total_pages {
                        current_page = page_num;
                    } else {
                        println!("❌ Invalid page number! Must be between 1 and {}", total_pages);
                    }
                }
                _ => {
                    println!("❌ Unknown command '{}'. Type 'h' for help.", command);
                }
            }
        }
    }
}

fn display_page_detailed(page_num: u16) {
    if let Some(page_data) = get_page_data(page_num) {
        println!("\n📄 Page {} Details:", page_num);
        println!("📊 Lines: {}", page_data.lines.len());
        println!("🎯 Surah headers: {}", page_data.surah_headers.len());
        
        // Show surah information if any
        if !page_data.surah_headers.is_empty() {
            println!("\n📚 Surahs on this page:");
            for surah in &page_data.surah_headers {
                println!("   • {} ({})", surah.name_simple, surah.name_arabic);
            }
        }
        
        // Show verses on this page
        let verses = get_verses_in_page(page_num);
        if !verses.is_empty() {
            println!("\n📖 Verses on this page ({} verses):", verses.len());
            
            // Group verses by surah for better display
            let mut current_surah = 0u16;
            let mut displayed_count = 0;
            
            for verse_key in &verses {
                if displayed_count >= 10 {
                    println!("   ... and {} more verses", verses.len() - displayed_count);
                    break;
                }
                
                if let Some(ayah) = get_ayah_data(verse_key) {
                    // Show surah header when we encounter a new surah
                    if ayah.surah_number != current_surah {
                        current_surah = ayah.surah_number;
                        println!("\n   📚 Surah {}: {}", current_surah, ayah.surah_name);
                    }
                    
                    // Display ayah with truncated text for readability
                    let text = if ayah.text_arab.chars().count() > 50 {
                        let truncated: String = ayah.text_arab.chars().take(50).collect();
                        format!("{}...", truncated)
                    } else {
                        ayah.text_arab.clone()
                    };
                    
                    println!("   {}:{} - {}", ayah.surah_number, ayah.ayah_number, text);
                    displayed_count += 1;
                }
            }
        }
        
        // Show line-by-line breakdown
        println!("\n📝 Line-by-line breakdown:");
        for (i, line) in page_data.lines.iter().enumerate().take(5) {
            let line_type = match line.line_type.as_str() {
                "surah_name" => "📚 Surah Header",
                "basmallah" => "🌟 Basmallah",
                "ayah" => "📖 Ayah",
                _ => "📄 Line",
            };
            
            println!("   Line {}: {} ({})", i + 1, line_type, 
                    if line.is_centered { "centered" } else { "aligned" });
            if line.text.chars().count() > 40 {
                let truncated: String = line.text.chars().take(40).collect();
                println!("      {}", truncated);
                println!("      ...");
            } else {
                println!("      {}", line.text);
            }
        }
        
        if page_data.lines.len() > 5 {
            println!("   ... and {} more lines", page_data.lines.len() - 5);
        }
        
    } else {
        println!("❌ Page {} not found!", page_num);
    }
}

fn search_within_page(page_num: u16) {
    println!("\n🔍 Search within Page {}", page_num);
    println!("========================");
    
    print!("Enter search text: ");
    io::stdout().flush().unwrap();
    
    let mut search_input = String::new();
    if io::stdin().read_line(&mut search_input).is_ok() {
        let search_text = search_input.trim();
        
        if search_text.is_empty() {
            println!("❌ Please enter search text.");
            return;
        }
        
        // Get all verses in the page
        let verses = get_verses_in_page(page_num);
        let mut found_results = Vec::new();
        
        for verse_key in &verses {
            if let Some(ayah) = get_ayah_data(verse_key) {
                // Search in Arabic text
                if ayah.text_arab.contains(search_text) {
                    found_results.push((ayah, "Arabic"));
                }
                // Search in translation
                else if ayah.translation_id.contains(search_text) {
                    found_results.push((ayah, "Translation"));
                }
                // Search in transliteration
                else if ayah.transliteration.contains(search_text) {
                    found_results.push((ayah, "Transliteration"));
                }
            }
        }
        
        if found_results.is_empty() {
            println!("❌ No results found for '{}' in page {}", search_text, page_num);
        } else {
            println!("\n✅ Found {} results for '{}' in page {}:", 
                    found_results.len(), search_text, page_num);
            println!("{}", "=".repeat(50));
            
            for (i, (ayah, found_in)) in found_results.iter().enumerate() {
                println!("\n{}. 📖 {}:{} (found in {})", 
                        i + 1, ayah.surah_number, ayah.ayah_number, found_in);
                println!("   🔤 Arabic: {}", ayah.text_arab);
                println!("   🌍 Translation: {}", ayah.translation_id);
            }
        }
    }
}

fn show_page_viewer_help() {
    println!("\n📚 INTERACTIVE PAGE VIEWER - HELP");
    println!("==================================");
    println!("Commands:");
    println!("  n, next     - Go to next page");
    println!("  p, prev     - Go to previous page");
    println!("  g, goto     - Go to specific page number");
    println!("  f, find     - Find verse and go to its page");
    println!("  r, random   - Go to random page");
    println!("  s, search   - Search within current page");
    println!("  h, help     - Show this help");
    println!("  q, quit     - Return to main menu");
    println!("  [number]    - Quick jump to page number");
    println!("\nTips:");
    println!("  • Use arrow keys or n/p for quick navigation");
    println!("  • Type page numbers directly for quick jumps");
    println!("  • Use 'f' to find specific verses like 2:255");
    println!("  • Use 's' to search for text within current page");
}
