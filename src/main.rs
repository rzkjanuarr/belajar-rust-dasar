fn main() {
    println!("Hello, world!");
}


#[test]
fn materi_variable() {
    // 1. Imutable = Variable yang tidak dapat dirubah!
    let name = "Rizki Januar I.";
    println!("{}", name);

    // 2. Mutable = Variable yang dapat dirubah!
    let mut age = 20;
    println!("{}", age);

    age = 24;
    println!("{}", age);
}



#[test]
fn materi_comment() {
    // 1. Ini namanya Single Line Comment (pake dua garis miring)
    // Biasanya buat catatan pendek di atas baris kode.

    let skor = 100; // Bisa juga diletakan di samping kode kayak gini

    /* 2. Ini namanya Multi-line Comment.
       Bisa buat catatan yang panjang lebar
       sampai berbaris-baris.
    */

    println!("Cek terminal, komentar nggak bakal muncul di sini!");
    println!("Skor gue: {skor}");
}


#[test]
fn materi_data_types_scalar_integer(){
    let a: i8 = -4;
    println!("{}", a);

    let b: u32 = 1000;
    println!("{}", b);

    let c: usize = 8;
    println!("{:?}", c);
}


#[test]
fn materi_data_types_scalar_konversi_tipe_number_data() {
    // Konversi kecil ke besar
    let a: u8 = 10;
    let b: u32 = a as u32; // Konversi dari u8 ke u32
    println!("Nilai : {b}");

    // Konversi besar ke kecil
    let c: i32 = 256;
    let d: i8 = c as i8;
    println!("Nilai : {d}");
}


#[test]
fn materi_data_types_scalar_float() {
    let pi = 3.14;
    println!("Nilai : {pi}");
}


#[test]
fn materi_data_types_scalar_boolean() {
    // 1. Deklarasi eksplisit
    let is_rust_hard: bool = false;

    // 2. Deklarasi otomatis (Type Inference)
    let is_learning = true;

    // 3. Boolean dari hasil perbandingan
    let angka = 10;
    let apakah_lebih_besar = angka > 5; // Isinya bakal true

    println!("--- BOOLEAN ---");
    println!("Apakah Rust susah? {is_rust_hard}");
    println!("Lagi belajar? {is_learning}");
    println!("Apakah 10 > 5? {apakah_lebih_besar}");
}



#[test]
fn materi_data_types_scalar_char() {
    // 1. Karakter biasa
    let huruf = 'R';

    // 2. Karakter angka (tapi tipenya char, bukan integer)
    let angka_char = '7';

    // 3. Karakter Spesial / Emoji
    let emoji = '🚀';
    let mandarin = '学';

    println!("--- CHARACTER ---");
    println!("Huruf: {huruf}");
    println!("Angka (Char): {angka_char}");
    println!("Emoji: {emoji}");
    println!("Karakter Mandarin: {mandarin}");
}


#[test]
fn materi_data_types_compound_membuat_tuple() {
    let personal = ("Rizki", 24, true);
    println!("{:?}", personal);
}


#[test]
fn materi_data_types_compound_mengakses_tuple() {
    let produk = ("Kopi Susu", 15000, true);

    let nama_produk = produk.0;
    let harga = produk.1;

    println!("--- AKSES PAKE INDEX ---");
    println!("Produk: {nama_produk}, Harga: {harga}");
}



#[test]
fn materi_data_types_compound_destructuring_tuple() {
    let koordinat = (10.5, 20.0, 5.2);

    let (x, y, z) = koordinat;

    println!("--- HASIL DESTRUCTURING ---");
    println!("Titik X: {x}");
    println!("Titik Y: {y}");
    println!("Titik Z: {z}");

    // Ignored Values (Kalau cuma mau ambil X tapi sisanya dibuang/dicuekin)
    let (titik_awal, _, _) = koordinat;
    println!("--- CUMA AMBIL SATU NILAI ---");
    println!("Titik Awal: {titik_awal}");
}



#[test]
fn materi_tuple_mutable() {
    // 1. Deklarasi Tuple yang mutable pake 'mut'
    let mut status_mesin = ("OFF", 0);

    println!("--- SEBELUM DIUBAH ---");
    println!("Status: {}, Suhu: {}", status_mesin.0, status_mesin.1);

    // 2. Update nilai pake index (titik)
    status_mesin.0 = "ON";
    status_mesin.1 = 85;

    println!("--- SESUDAH DIUBAH ---");
    println!("Status: {}, Suhu: {}", status_mesin.0, status_mesin.1);

    // 3. Inget: Tipe data gak boleh diganti!
    // status_mesin.1 = "Panas"; // Ini bakal ERROR karena index .1 harus integer
}


#[test]
fn materi_tuple_unit() {
    // 1. Membuat Unit Value
    let unit_kosong = ();

    println!("--- UNIT TYPE ---");
    // Karena isinya kosong, kita pake {:?} (Debug mode) buat nampilinnya
    println!("Isi dari unit_kosong: {:?}", unit_kosong);

    // 2. Unit biasanya muncul dari fungsi yang gak balikin apa-apa
    fn fungsi_gak_jelas() {
        println!("Lagi lari kodenya...");
    }

    let hasil = fungsi_gak_jelas();
    println!("Nilai yang dibalikin fungsi: {:?}", hasil);
}



#[test]
fn materi_membuat_array() {
    // 1. Cara manual
    let bulan: [&str; 3] = ["Januari", "Februari", "Maret"];

    // 2. Cara instan (isi 10 angka 0 semua)
    let angka = [0; 10];

    println!("--- MEMBUAT ARRAY ---");
    println!("Bulan pertama: {}", bulan[0]);
    println!("Daftar angka: {:?}", angka);
}



#[test]
fn materi_mengakses_array() {
    let daftar_harga = [15000, 20000, 25000];

    let harga_satu = daftar_harga[0];
    let harga_dua = daftar_harga[1];

    println!("--- AKSES ARRAY ---");
    println!("Harga 1: {harga_satu}, Harga 2: {harga_dua}");
}
