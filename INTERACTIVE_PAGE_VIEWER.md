# Interactive Page Viewer - Fitur Baru

## 📖 Interactive Page Viewer

Fitur **Interactive Page Viewer** adalah penambahan terbaru yang memungkinkan pengguna untuk menjelajahi Al-Quran halaman per halaman dengan cara yang interaktif dan user-friendly.

### 🚀 Cara Menggunakan

1. **Memulai Interactive Page Viewer**
   - Jalankan CLI: `.\target\release\hafiz_assistant_cli.exe`
   - Pilih opsi **9. Interactive Page Viewer**

2. **Navigasi Halaman**
   - `n` atau `next` - Halaman selanjutnya
   - `p` atau `prev` - Halaman sebelumnya
   - `g` atau `goto` - Pergi ke halaman tertentu
   - `[nomor]` - Langsung ke halaman (contoh: ketik `50` untuk halaman 50)

3. **Pencarian dan Fitur Lanjutan**
   - `f` atau `find` - Cari ayat dan pergi ke halamannya
   - `r` atau `random` - Halaman acak
   - `s` atau `search` - Cari teks dalam halaman saat ini
   - `h` atau `help` - Bantuan lengkap
   - `q` atau `quit` - Kembali ke menu utama

### 🎯 Fitur Utama

#### 1. **Tampilan Halaman Detail**
Setiap halaman menampilkan:
- **Nomor halaman** dan posisi (contoh: "Page 1 / 604")
- **Jumlah baris** dalam halaman
- **Jumlah header surah** dalam halaman
- **Daftar surah** yang ada di halaman
- **Daftar ayat** yang ada di halaman (dengan batas 10 ayat untuk readability)
- **Breakdown baris per baris** dengan tipe dan alignment

#### 2. **Navigasi Cepat**
- **Navigation by number**: Langsung ketik nomor halaman
- **Sequential navigation**: Gunakan n/p untuk maju/mundur
- **Jump to specific page**: Gunakan command `g` untuk pergi ke halaman tertentu
- **Random page**: Gunakan `r` untuk halaman acak

#### 3. **Pencarian Terintegrasi**
- **Find verse**: Cari ayat tertentu (contoh: "2:255") dan langsung pergi ke halamannya
- **Search within page**: Cari teks dalam halaman saat ini
- **Support multiple search targets**: Arabic text, translation, dan transliteration

#### 4. **Tampilan yang User-Friendly**
- **Clean formatting**: Tampilan yang rapi dan mudah dibaca
- **Emoji indicators**: Menggunakan emoji untuk kategori yang berbeda
- **Truncated text**: Teks panjang dipotong untuk readability
- **Grouped display**: Ayat dikelompokkan berdasarkan surah

### 🔍 Contoh Penggunaan

```
📖 INTERACTIVE PAGE VIEWER (MUSHAF)
====================================
📊 Total Pages: 604

============================================================
📄 Current Page: 1 / 604
============================================================

📄 Page 1 Details:
📊 Lines: 8
🎯 Surah headers: 1

📚 Surahs on this page:
   • Al-Fatihah (الفاتحة)

📖 Verses on this page (4 verses):

   📚 Surah 1: Al-Fatihah
   1:1 - بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ
   1:2 - الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ
   1:3 - الرَّحْمَٰنِ الرَّحِيمِ
   1:4 - مَالِكِ يَوْمِ الدِّينِ

📝 Line-by-line breakdown:
   Line 1: 📚 Surah Header (centered)
      Al-Fatihah
   Line 2: 📖 Ayah (centered)
      Text for words 1-5
   ...

🎮 Navigation Options:
  [n] Next page
  [p] Previous page
  [g] Go to specific page
  [f] Find verse (go to page containing verse)
  [r] Random page
  [s] Search within current page
  [h] Help
  [q] Quit to main menu

Enter command: 
```

### 📊 Search Within Page

Fitur pencarian dalam halaman memungkinkan mencari:
- **Teks Arab**: Mencari dalam teks asli Al-Quran
- **Terjemahan**: Mencari dalam terjemahan Indonesia
- **Transliterasi**: Mencari dalam transliterasi

Contoh output pencarian:
```
🔍 Search within Page 1
========================
Enter search text: الله

✅ Found 2 results for 'الله' in page 1:
==================================================

1. 📖 1:1 (found in Arabic)
   🔤 Arabic: بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ
   🌍 Translation: Dengan menyebut nama Allah...

2. 📖 1:2 (found in Arabic)
   🔤 Arabic: الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ
   🌍 Translation: Segala puji bagi Allah...
```

### 🎯 Keunggulan Fitur

1. **Intuitive Navigation**: Navigasi yang mudah dipahami dengan berbagai opsi
2. **Comprehensive Display**: Informasi lengkap tentang setiap halaman
3. **Real-time Search**: Pencarian instan dalam halaman
4. **Verse Location**: Cari ayat dan langsung pergi ke halamannya
5. **Random Exploration**: Fitur random untuk eksplorasi Al-Quran
6. **Help System**: Sistem bantuan yang komprehensif

### 🛠️ Technical Details

- **Total Pages**: 604 halaman (sesuai standar mushaf)
- **Page Data Source**: `qpc-v2-15-lines.db.json`
- **Search Algorithm**: Full-text search dalam Arabic, translation, dan transliteration
- **Memory Efficient**: Data dimuat sesuai kebutuhan
- **Cross-platform**: Bekerja di Windows, Linux, dan macOS

### 📚 Commands Reference

| Command | Alias | Description |
|---------|-------|-------------|
| `n` | `next` | Halaman selanjutnya |
| `p` | `prev`, `previous` | Halaman sebelumnya |
| `g` | `goto` | Pergi ke halaman tertentu |
| `f` | `find` | Cari ayat dan pergi ke halamannya |
| `r` | `random` | Halaman acak |
| `s` | `search` | Cari dalam halaman saat ini |
| `h` | `help` | Tampilkan bantuan |
| `q` | `quit`, `exit` | Kembali ke menu utama |
| `[number]` | - | Langsung ke halaman (contoh: `50`) |

### 🎉 Manfaat

1. **Untuk Pengguna Biasa**: Mudah menjelajahi Al-Quran halaman per halaman
2. **Untuk Developer**: Testing fitur page data dan navigation
3. **Untuk Integrasi**: Demonstrasi kemampuan page rendering untuk Flutter app
4. **Untuk Penelitian**: Tool untuk menganalisis struktur halaman Al-Quran

Fitur ini menjadikan Hafiz Assistant Backend lebih lengkap dan user-friendly, memberikan pengalaman interaktif yang optimal untuk eksplorasi Al-Quran! 🚀📖
