// Materi Match
// Match itu seperti if/else if, tapi lebih rapi dan powerful.
// Rust akan mencocokkan nilai ke setiap "arm" (cabang) dari atas ke bawah.
// Setiap arm dipisahkan dengan koma, dan wajib ada "_" sebagai default (pengganti else).

// Contoh:
#[test]
fn materi_match() {
    let nilai = 85;

    match nilai {
        90..=100 => println!("Grade A"),
        80..=89 => println!("Grade B"),
        70..=79 => println!("Grade C"),
        _ => println!("Grade D"), // _ artinya "selain semua di atas"
    }
}

// Perilaku match:
// - Cocokkan nilai dari atas ke bawah
// - Begitu ketemu yang cocok, langsung eksekusi dan berhenti
// - "_" wajib ada sebagai penangkap semua kondisi yang tidak cocok
// - Mirip if/else if tapi jauh lebih rapi untuk banyak kondisi

// SOAL 1

// Gunakan match untuk tampilkan nama harinya.
// 1 = "Senin", 2 = "Selasa", 3 = "Rabu", 4 = "Kamis",
// 5 = "Jumat", 6 = "Sabtu", 7 = "Minggu",
// selain itu tampilkan "Hari tidak valid".
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]
#[test]
fn soal_1() {
    let hari = 3;
    match hari {
        1 => println!("Senin"),
        2 => println!("Selasa"),
        3 => println!("Rabu"),
        4 => println!("Kamis"),
        5 => println!("Jumat"),
        6 => println!("Sabtu"),
        7 => println!("Minggu"),
        _ => println!("Hari tidak valid"),
    }
}

// SOAL 1
// Buat variable "bulan": u8 = 8.
// Gunakan match untuk tampilkan nama bulannya.
// 1 = "Januari" ... 12 = "Desember", selain itu = "Bulan tidak valid".
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]

// SOAL 2
// Buat variable "kode_status": u16 = 404.
// Gunakan match untuk tampilkan artinya.
// 200 = "OK", 201 = "Created", 400 = "Bad Request",
// 404 = "Not Found", 500 = "Internal Server Error", selain itu = "Unknown".
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]

// SOAL 3
// Buat variable "nilai": u8 = 78.
// Gunakan match dengan range untuk tampilkan grade.
// 90..=100 = "A", 80..=89 = "B", 70..=79 = "C",
// 60..=69 = "D", selain itu = "F".
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]

// SOAL 4
// Buat variable "role": &str = "admin".
// Gunakan match untuk tampilkan akses yang dimiliki.
// "admin"     = "Full Access"
// "moderator" = "Read & Write Access"
// "user"      = "Read Only Access"
// selain itu  = "No Access"
// Clue: nama function -> fn soal_4(), wajib pakai attribute #[test]

// SOAL 5
// Buat variable "harga": i64 = 3_500_000.
// Gunakan match dengan range untuk tampilkan kategori produk.
// 0..=500_000          = "Budget"
// 500_001..=2_000_000  = "Mid Range"
// 2_000_001..=5_000_000 = "Premium"
// selain itu           = "Luxury"
// Clue: nama function -> fn soal_5(), wajib pakai attribute #[test]
