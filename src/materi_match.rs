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
