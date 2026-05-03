// Materi For
// For digunakan untuk mengulang sejumlah data atau range yang sudah ditentukan.
// Berbeda dengan loop dan while, for berhenti otomatis tanpa perlu break.

// Contoh for dengan range:
#[test]
fn materi_for() {
    for i in 1..=5 {
        println!("Angka: {}", i);
    }
}
// Output: 1, 2, 3, 4, 5

// Contoh for dengan range tanpa =:
#[test]
fn materi_for_2() {
    for i in 1..5 {
        println!("Angka: {}", i);
    }
}
// Output: 1, 2, 3, 4 (5 tidak termasuk!)

// Perilaku for:
// - 1..5   = dari 1 sampai 4 (tidak termasuk 5)
// - 1..=5  = dari 1 sampai 5 (termasuk 5)
// - berhenti otomatis, tidak perlu break
// - "i" adalah variable sementara yang berubah setiap putaran

// SOAL 1
// Gunakan for untuk tampilkan angka 1 sampai 10.
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]
#[test]
fn soal_1() {
    for i in 1..=10 {
        println!("{}", i);
    }
}

// SOAL 2
// Gunakan for untuk menghitung total penjumlahan dari 1 sampai 10.
// Simpan ke variable mutable "total", tampilkan hasilnya.
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]
#[test]
fn soal_2() {
    let mut total = 0;
    for i in 1..=10 {
        total += i;
    }
    println!("Total: {}", total);
}
