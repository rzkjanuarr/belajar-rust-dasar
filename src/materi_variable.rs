use std::io::read_to_string;

// Example
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

// SOAL 1
// Buat variable imutable bernama "username" dengan nilai nama kamu,
// lalu tampilkan ke layar.
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]
#[test]
fn soal_1() {
    let username = "rizki";
    println!("Soal 1: {}", username);
}

// SOAL 2
// Buat variable mutable bernama "score" dengan nilai 0,
// tampilkan, lalu ubah menjadi 100 dan tampilkan lagi.
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]
#[test]
fn soal_2() {
    let mut score = 0;
    score = 100;

    println!("Soal 2: {}", score);
}

// SOAL 3
// Buat variable imutable dengan tipe data eksplisit i32 bernama "tahun"
// bernilai 2024, lalu tampilkan.
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]
#[test]
fn soal_3() {
    let tahun = 2024;
    println!("Soal 3: {}", tahun);
}

// SOAL 4
// Buat variable mutable bernama "harga" bertipe f64 dengan nilai 10000.50,
// tampilkan, lalu ubah menjadi 20000.75 dan tampilkan lagi.
// Clue: nama function -> fn soal_4(), wajib pakai attribute #[test]
#[test]
fn soal_4() {
    let mut harga = 1000.50;
    harga = 20000.75;

    println!("Soal 4: {}", harga);
}

// SOAL 5
// Buat variable imutable bernama "is_active" bertipe bool dengan nilai true,
// lalu tampilkan.
// Clue: nama function -> fn soal_5(), wajib pakai attribute #[test]
#[test]
fn soal_5() {
    let is_active: bool = true;

    println!("Soal 5: {}", is_active);
}

// SOAL 6
// Buat dua variable: "panjang" dan "lebar" bertipe i32,
// lalu hitung dan tampilkan luas persegi panjang (panjang * lebar).
// Clue: nama function -> fn soal_6(), wajib pakai attribute #[test]
#[test]
fn soal_6() {
    let panjang = 10;
    let lebar = 5;

    let result = panjang * lebar;

    println!("Result Panjang * Lebar: {}", result);
}

// SOAL 7
// Lakukan shadowing: buat variable "angka" bernilai 5,
// lalu shadow dengan nilai "angka" * 2, lalu tampilkan.
// Clue: nama function -> fn soal_7(), wajib pakai attribute #[test]
#[test]
fn soal_7() {
    let angka = 5;
    let angka = angka * 2;
    println!("Soal 7: {}", angka);
}

// SOAL 8
// Lakukan shadowing: buat variable "kata" bernilai "halo",
// lalu shadow dengan nilai panjang string-nya (gunakan .len()),
// lalu tampilkan.
// Clue: nama function -> fn soal_8(), wajib pakai attribute #[test]
#[test]
fn soal_8() {
    let kata = "halo";
    let kata = kata.len();

    println!("Soal 8: {}", kata);
}

// SOAL 9
// Buat variable imutable bernama "huruf" bertipe char dengan nilai 'R',
// lalu tampilkan.
// Clue: nama function -> fn soal_9(), wajib pakai attribute #[test]
#[test]
fn soal_9() {
    let huruf = 'R';
    println!("Soal 9: {}", huruf);
}

// SOAL 10
// Buat variable mutable bernama "counter" bernilai 0,
// tambahkan 1 sebanyak 3 kali (satu per satu), tampilkan setiap perubahannya.
// Clue: nama function -> fn soal_10(), wajib pakai attribute #[test]
#[test]
fn soal_10() {
    let mut counter = 0;
    counter += 1;
    println!("Angka 1: {}", counter);
    counter += 1;
    println!("Angka 2: {}", counter);
    counter += 1;
    println!("Angka 3: {}", counter);
}

// SOAL 11
// Buat variable dengan nilai hasil operasi: (10 + 5) * 2 - 3,
// simpan di variable "hasil", lalu tampilkan.
// Clue: nama function -> fn soal_11(), wajib pakai attribute #[test]
#[test]
fn soal_11() {
    let nilai = (10 + 5) * 2 - 3;
    let hasil = nilai;
    println!("Soal 11: {}", hasil);
}

// SOAL 12
// Buat dua variable mutable "a" = 10 dan "b" = 20,
// tukar nilai keduanya, lalu tampilkan a dan b.
// Clue: nama function -> fn soal_12(), wajib pakai attribute #[test]
#[test]
fn soal_12() {
    let mut a = 10;
    let mut b = 20;

    a = 20;
    b = 10;

    println!("Hasil A: {}", a);
    println!("Hasil B: {}", b);
}

// SOAL 13
// Buat variable bertipe u8 bernama "umur" dengan nilai 25,
// lalu tampilkan dengan format: "Umur saya: 25 tahun".
// Clue: nama function -> fn soal_13(), wajib pakai attribute #[test]
#[test]
fn soal_13() {
    let umur: u8 = 25;
    println!("Umur saya: {umur} tahun");
}

// SOAL 14
// Buat variable imutable "pi" bertipe f32 dengan nilai 3.14,
// dan "r" bernilai 7.0. Hitung luas lingkaran (pi * r * r) dan tampilkan.
// Clue: nama function -> fn soal_14(), wajib pakai attribute #[test]
#[test]
fn soal_14() {
    let pi = 3.14;
    let r = 7.0;

    let luas_lingkaran = (pi * r * r);
    println!("Hasil: {}", luas_lingkaran);
}

// SOAL 15
// Buat variable mutable "is_login" bernilai false, tampilkan,
// lalu ubah menjadi true dan tampilkan lagi.
// Clue: nama function -> fn soal_15(), wajib pakai attribute #[test]
#[test]
fn soal_15() {
    let mut is_login = false;
    is_login = true;

    println!("{}", is_login);
}

// SOAL 16
// Buat variable "kalimat" yang menyimpan string "Belajar Rust",
// tampilkan panjang karakternya menggunakan .len().
// Clue: nama function -> fn soal_16(), wajib pakai attribute #[test]
#[test]
fn soal_16() {
    let kalimat = "Belajar Rust";
    println!("Panjang Karakter Variable Kalimat:{}", kalimat.len());
}

// SOAL 17
// Buat variable imutable "x": i64 = 1_000_000 (gunakan underscore separator),
// lalu tampilkan.
// Clue: nama function -> fn soal_17(), wajib pakai attribute #[test]
#[test]
fn soal_17() {
    let x = 1_000_000;
    println!("{}", x);
}

// SOAL 18
// Buat variable mutable "nilai" = 70.0_f64, tampilkan,
// lalu tambahkan 15.5 dan tampilkan hasilnya.
// Clue: nama function -> fn soal_18(), wajib pakai attribute #[test]
#[test]
fn soal_18() {
    let mut nilai = 70.0_f64;
    nilai = nilai + 15.5;
    println!("{}", nilai);
}

// SOAL 19
// Buat 3 variable: "nama", "kota", "negara",
// lalu tampilkan dalam satu println! dengan format:
// "Nama: ..., Kota: ..., Negara: ..."
// Clue: nama function -> fn soal_19(), wajib pakai attribute #[test]
#[test]
fn soal_19() {
    let nama = "Jhon Paul 777";
    let kota = "Chicago";
    let negara = "USA";

    println!("Nama: {nama} , Kota: {kota}, Negara: {negara}");
}

// SOAL 20
// Buat variable "celcius" = 100.0_f64, konversi ke Fahrenheit
// dengan rumus: (celcius * 9.0 / 5.0) + 32.0,
// simpan di variable "fahrenheit", lalu tampilkan keduanya.
// Clue: nama function -> fn soal_20(), wajib pakai attribute #[test]
#[test]
fn soal_20() {
    let celcius = 100.0_f64;
    let fahrenheit = (celcius * 9.0 / 5.0) + 32.0;
    println!("Celcius: {}", celcius);
    println!("Fahrenheit: {}", fahrenheit);
}
