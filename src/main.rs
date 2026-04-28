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
