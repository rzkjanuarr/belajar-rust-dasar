// SOAL 1
// Buat variable "nilai": i32 = 75.
// Jika nilai lebih dari atau sama dengan 60, tampilkan "Lulus".
// Jika tidak, tampilkan "Tidak Lulus".
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]]
#[test]
fn soal_1() {
    let nilai = 75;

    if nilai >= 60 {
        println!("LULUS!");
    } else {
        println!("TIDAK LULUS!");
    }
}

// SOAL 2
// Buat variable "saldo": i32 = 500_000.
// Buat variable "harga_barang": i32 = 350_000.
// Jika saldo cukup untuk membeli barang, tampilkan "Pembelian Berhasil".
// Jika tidak, tampilkan "Saldo Tidak Cukup".
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]
#[test]
fn soal_2() {
    let saldo = 500_000;
    let harga_barang = 350_000;

    if saldo > harga_barang {
        println!("SALDO CUKUP");
    } else {
        println!("SALDO TIDAK CUKUP");
    }
}

// SOAL 3
// Buat variable "umur": u8 = 17.
// Jika umur lebih dari atau sama dengan 18, tampilkan "Boleh Masuk".
// Jika tidak, tampilkan "Dibawah Umur, Tidak Boleh Masuk".
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]
#[test]
fn soal_3() {
    let umur = 17;

    if umur >= 18 {
        println!("BOLEH MASUK");
    } else {
        println!("dibawah umur!")
    }
}

// SOAL 1
// Buat variable "suhu": i32 = 38.
// Jika suhu di bawah 20, tampilkan "Dingin".
// Jika suhu antara 20-35, tampilkan "Normal".
// Jika suhu di atas 35, tampilkan "Panas".
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]

// SOAL 2
// Buat variable "saldo": i64 = 2_500_000.
// Jika saldo di atas 5_000_000, tampilkan "Platinum Member".
// Jika saldo di atas 2_000_000, tampilkan "Gold Member".
// Jika saldo di atas 500_000, tampilkan "Silver Member".
// Jika tidak, tampilkan "Regular Member".
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]

// SOAL 3
// Buat variable "umur": u8 = 25.
// Jika umur di bawah 13, tampilkan "Anak-anak".
// Jika umur di bawah 18, tampilkan "Remaja".
// Jika umur di bawah 60, tampilkan "Dewasa".
// Jika tidak, tampilkan "Lansia".
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]
