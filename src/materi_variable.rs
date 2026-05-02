use std::{i32, i64, io::read_to_string, u32};

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
    let x = 3.14;
    let r = 7.0;

    let luas_lingkaran = (x * r * r);
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

// SOAL 1
// Buat variable imutable "nama" bertipe &str dan "umur" bertipe u8,
// tampilkan dengan format: "Nama: ..., Umur: ..."
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]
#[test]
fn soal_21() {
    let nama = "Jhon Paul 777";
    let umur = 25_u8;
    println!("Nama : {nama}, Umur: {umur}");
}

// SOAL 2
// Buat variable "nilai_int" bertipe i32 = 42,
// konversi ke f64 menggunakan "as", simpan ke "nilai_float", tampilkan keduanya.
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]
#[test]
fn soal_22() {
    let nilai_int: i32 = 42;
    let nilai_float = nilai_int as f64;
    println!("Nilai INT : {}", nilai_int);
    println!("Nilai FLOAT : {}", nilai_float);
}

// SOAL 3
// Buat variable mutable "suhu_celcius": f64 = 37.5,
// konversi ke i32 menggunakan "as", tampilkan sebelum dan sesudah konversi.
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]
#[test]
fn soal_23() {
    let mut suhu_celcius: f64 = 37.5;
    let suhu_celcius = suhu_celcius as i32;
    println!("Suhu Celcius Sebelum: {}", suhu_celcius);
    println!("Suhu Celcius Sesudah: {}", suhu_celcius);
}

// SOAL 4
// Buat variable "poin": u8 = 200,
// konversi ke u32 menggunakan "as", lalu kalikan 1000, tampilkan hasilnya.
// Clue: nama function -> fn soal_4(), wajib pakai attribute #[test]
#[test]
fn soal_24() {
    let poin: u8 = 200;
    let konversi_poin = poin as u32;
    let result = konversi_poin * 1000;
    println!("{}", result);
}

// SOAL 5
// Lakukan shadowing: buat variable "angka": f64 = 9.99,
// shadow menjadi i32 menggunakan "as", tampilkan.
// Clue: nama function -> fn soal_5(), wajib pakai attribute #[test]
#[test]
fn soal_25() {
    let angka: f64 = 9.99;
    let angka = angka as i32;
    println!("{}", angka);
}

// SOAL 6
// Buat variable "lebar": u32 = 1920 dan "tinggi": u32 = 1080,
// hitung total pixel (lebar * tinggi), simpan ke "total_pixel", tampilkan.
// Clue: nama function -> fn soal_6(), wajib pakai attribute #[test]
#[test]
fn soal_26() {
    let lebar: u32 = 1920;
    let tinggi: u32 = 1080;

    let total_pixel = (lebar * tinggi);
    println!("{total_pixel}");
}

// SOAL 7
// Buat variable "a": i64 = 1_000_000_000,
// konversi ke i32 menggunakan "as", tampilkan keduanya.
// (perhatikan apa yang terjadi pada nilainya!)
// Clue: nama function -> fn soal_7(), wajib pakai attribute #[test]
#[test]
fn soal_27() {
    let a: i64 = 1000000000;
    let b = a as i32;
    println!("{}", b);
}

// SOAL 8
// Buat variable mutable "level": u8 = 1, tampilkan,
// lalu gunakan assignment untuk tambah 4, tampilkan lagi.
// Clue: nama function -> fn soal_8(), wajib pakai attribute #[test]
#[test]
fn soal_28() {
    let mut level: u8 = 1;
    level = level + 4;
    println!("{}", level);
}

// SOAL 9
// Buat variable "lat": f64 = -7.250445 dan "lon": f64 = 112.768845,
// tampilkan dengan format: "Koordinat: lat={}, lon={}"
// Clue: nama function -> fn soal_9(), wajib pakai attribute #[test]
#[test]
fn soal_29() {
    let lat = -7.250445;
    let lon = 112.768845;
    println!("Koordinat: lat={lat}, lon={lon}");
}

// SOAL 10
// Buat variable "harga": f64 = 150000.0 dan "diskon": f64 = 0.2,
// hitung harga setelah diskon, simpan ke "harga_akhir", tampilkan.
// Clue: nama function -> fn soal_10(), wajib pakai attribute #[test]
#[test]
fn soal_30() {
    let harga = 150000.0;
    let diskon = 0.2;
    let harga_akhir = harga / diskon;

    println!("Harga Promo nih {} bro!", harga_akhir);
}

// SOAL 11
// Buat variable "x": f32 = 7.8,
// konversi ke i32 menggunakan "as" (perhatikan nilainya terpotong!), tampilkan keduanya.
// Clue: nama function -> fn soal_11(), wajib pakai attribute #[test]
#[test]
fn soal_31() {
    let x = 7.8;
    let z = x as i32;

    println!("Nilai Aslinya : {}", x);
    println!("Nilai Sesudahnya : {}", z);
}

// SOAL 12
// Buat variable "is_rust_fun": bool = true dan "bahasa": &str = "Rust",
// tampilkan dengan format: "Belajar {} menyenangkan: {}"
// Clue: nama function -> fn soal_12(), wajib pakai attribute #[test]
#[test]
fn soal_32() {
    let is_rust_fun = true;
    let bahasa = "Rust";

    println!("Belajar {bahasa} menyenangkan: {is_rust_fun}");
}

// SOAL 13
// Buat variable "byte_val": u8 = 255,
// konversi ke i8 menggunakan "as", tampilkan keduanya.
// (perhatikan apa yang terjadi — ini namanya overflow!)
// Clue: nama function -> fn soal_13(), wajib pakai attribute #[test]
#[test]
fn soal_33() {
    let byte_val = 255;
    let x = byte_val as i8;

    println!("{}", x);
}

// SOAL 14
// Buat variable "jarak_km": f64 = 5.7,
// konversi ke meter (kalikan 1000.0), simpan ke "jarak_meter": f64, tampilkan keduanya.
// Clue: nama function -> fn soal_14(), wajib pakai attribute #[test]
#[test]
fn soal_34() {
    let jarak_km = 5.7;
    let konversi_to_integer = jarak_km as u32;
    let jarak_meter = konversi_to_integer * 1000;

    println!("Hasil Sebelum di Konversi: {}", jarak_km);
    println!("Hasil sesudah di konversi : {}", konversi_to_integer);
    println!("Hasil perkalian : {}", jarak_meter);
}

// SOAL 15
// Lakukan shadowing 2 kali:
// "teks" = "100", shadow ke i32 = 100, shadow lagi ke f64 = 100.0, tampilkan.
// Clue: nama function -> fn soal_15(), wajib pakai attribute #[test]
#[test]
fn soal_35() {
    let teks = "100";
    println!("{}", teks);

    let teks = 100_i32;
    println!("{}", teks);

    let teks = 100.0_f64;
    println!("{}", teks);
}

// SOAL 16
// Buat variable "saldo": f64 = 1_500_000.0,
// konversi ke i64 menggunakan "as", tampilkan keduanya.
// Clue: nama function -> fn soal_16(), wajib pakai attribute #[test]
#[test]
fn soal_36() {
    let saldo = 1_500_000.0;
    let konversi_to_integer = saldo as u32;

    println!("Hasil Sebelum Konversi: {}", saldo);
    println!("Hasil sesudah Konversi: {}", konversi_to_integer);
}

// SOAL 17
// Buat variable mutable "hp": u32 = 100, tampilkan,
// kurangi 35 lalu tampilkan, kurangi 25 lagi lalu tampilkan.
// Clue: nama function -> fn soal_17(), wajib pakai attribute #[test]
#[test]
fn soal_37() {
    let mut hp = 100;

    hp = hp - 35;

    println!("Result: {}", hp);
}

// SOAL 18
// Buat variable "nilai_a": f64 = 85.5 dan "nilai_b": f64 = 90.0,
// hitung rata-rata, simpan ke "rata_rata", tampilkan.
// Clue: nama function -> fn soal_18(), wajib pakai attribute #[test]
#[test]
fn soal_38() {
    let nilai_a = 85.5;
    let nilai_b = 90.0;

    let avg = (nilai_a + nilai_b) / 2.0;

    println!("avg: {}", avg);
}

// SOAL 19
// Buat variable "umur_f64": f64 = 20.9,
// konversi ke u8 menggunakan "as", tampilkan keduanya.
// (perhatikan nilai desimalnya terpotong!)
// Clue: nama function -> fn soal_19(), wajib pakai attribute #[test]
#[test]
fn soal_39() {
    let umur = 20.9;
    let convert_to_integer = umur as u8;

    println!("Nilai Sebelum Konvert: {}", umur);
    println!("Nilai sesudah konvert: {}", convert_to_integer);
}

// SOAL 20
// Buat variable "eth_price": f64 = 3_450.75,
// "eth_amount": f64 = 2.5,
// hitung total nilai dalam USD, konversi ke i64 menggunakan "as",
// tampilkan semua: harga, jumlah, total f64, dan total i64.
// Clue: nama function -> fn soal_20(), wajib pakai attribute #[test]
#[test]
fn soal_40() {
    let eth_price = 3_450.75;
    let eth_amount = 2.5;

    let total_dalam_bentuk_usd = eth_price * eth_amount;
    let konversi_usd_to_integer = total_dalam_bentuk_usd as u64;
    println!("Harga ETH PRICE : {}", eth_price);
    println!("Harga ETH AMOUNT : {}", eth_amount);
    println!("Harga dalam bentuk USD : {}", konversi_usd_to_integer);
}

// SOAL 1
// Buat variable "ongkir": i8 dengan nilai 99 (maks pengiriman lokal dalam ribuan rupiah),
// tampilkan dengan format: "Ongkir: Rp {}000"
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]
#[test]
fn soal_41() {
    let ongkir: i8 = 99;
    println!("Rp {}000", ongkir);
}

// SOAL 2
// Buat variable "uang_saku": i16 dengan nilai 32_000 (rupiah harian),
// tampilkan dengan format: "Uang saku: Rp {}"
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]
#[test]
fn soal_42() {
    let uang_saku: i16 = 32_000;
    println!("Uang saku gue: Rp{}", uang_saku);
}

// SOAL 3
// Buat variable mutable "gaji": i32 dengan nilai 5_000_000 (rupiah),
// tampilkan, lalu tambahkan bonus 1_500_000, tampilkan lagi.
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]
#[test]
fn soal_43_mutable() {
    let mut gaji: i32 = 5_000_000;

    gaji = gaji + 1_500_000;

    println!("Gaji Pokok gue adalah : Rp{}", gaji);
    println!("Gaji Pokok + Tunjangan gue : Rp{}", gaji);
}

// SOAL 3
// Buat variable mutable "gaji": i32 dengan nilai 5_000_000 (rupiah),
// tampilkan, lalu tambahkan bonus 1_500_000, tampilkan lagi.
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]
#[test]
fn soal_43_shadowing() {
    let gaji: i32 = 5_000_000;
    println!("Gaji Pokok gue adalah : Rp{}", gaji);

    let gaji = gaji + 1_500_000;
    println!("Gaji Pokok + Tunjangan gue : Rp{}", gaji);
}

// SOAL 4
// Buat variable "tabungan": i64 dengan nilai 80_000_000_000 (rupiah),
// konversi ke i32 menggunakan "as", tampilkan keduanya.
// (perhatikan apa yang terjadi — overflow!)
// Clue: nama function -> fn soal_4(), wajib pakai attribute #[test]
#[test]
fn soal_44() {
    let tabungan: i64 = 80_000_000_000;
    let konversi_to_integer_small = tabungan as i32;
    println!("Nilai Asli: Rp{}", tabungan);
    println!("Nilai Sesudah: Rp{}", konversi_to_integer_small);
}

// SOAL 5
// Buat variable "aset_negara": i128 dengan nilai 1_000_000_000_000_000 (rupiah / 1 kuadriliun),
// tampilkan dengan format: "Aset negara: Rp {}"
// Clue: nama function -> fn soal_5(), wajib pakai attribute #[test1() {
#[test]
fn soal_45() {
    let aset_negara: i128 = 1_000_000_000_000_000;
    println!("Aset Negara: {}", aset_negara);
}

// SOAL 6
// Buat variable "umur": u8 dengan nilai 25,
// tampilkan dengan format: "Umur: {} tahun"
// Clue: nama function -> fn soal_6(), wajib pakai attribute #[test1() {
#[test]
fn soal_46() {
    let umur: u8 = 25;
    println!("umur gue: {}", umur);
}

// SOAL 7
// Buat variable "harga_tiket": u16 dengan nilai 65_000 (rupiah),
// beli 3 tiket, hitung total, konversi ke u32 menggunakan "as", tampilkan.
// Clue: nama function -> fn soal_7(), wajib pakai attribute #[test1() {

#[test]
fn soal_47() {
    let harga_tiket: u16 = 65_000;
    let budi = harga_tiket as u32 * 3;

    println!("Harga asli : Rp{}", harga_tiket);
    println!("Total budi : Rp{}", budi);
}

// SOAL 8
// Buat variable "dollar": u32 dengan nilai 4_294_000 (USD terbesar u32 ~ 4 juta),
// tampilkan dengan format: "Saldo: ${}"
// Clue: nama function -> fn soal_8(), wajib pakai attribute #[test]

#[test]
fn soal_48() {
    let dollar: u32 = 4_294_000;

    println!("Saldo: ${}", dollar);
}

// SOAL 9
// Buat variable "market_cap": u64 dengan nilai 18_000_000_000_000 (USD — market cap crypto),
// tampilkan dengan format: "Market Cap: ${}"
// Clue: nama function -> fn soal_9(), wajib pakai attribute #[test]

#[test]
fn soal_49() {
    let market_cap: u64 = 18_000_000_000_000;
    println!("Market Cap: ${}", market_cap);
}

// SOAL 10
// Buat variable "total_supply": u128 dengan nilai 21_000_000_000_000_000_000_000 (simulasi token supply),
// tampilkan dengan format: "Total Supply: {}"
// Clue: nama function -> fn soal_10(), wajib pakai attribute #[test]

#[test]
fn soal_50() {
    let total_supply: u128 = 21_000_000_000_000_000_000_000;

    println!("Total Suply: {}", total_supply);
}

// SOAL 11
// Buat variable "harga_btc": f32 dengan nilai 96_430.50 (USD),
// tampilkan dengan format: "Harga BTC: ${}"
// Clue: nama function -> fn soal_11(), wajib pakai attribute #[test]
#[test]
fn soal_51() {
    let harga_btc: f32 = 96_430.50;

    println!("Harga BTC: ${}", harga_btc);
}

// SOAL 12
// Buat variable "harga_eth": f64 dengan nilai 3_450.758912345678 (USD),
// tampilkan, perhatikan presisinya dibanding f32!
// Clue: nama function -> fn soal_12(), wajib pakai attribute #[test]
#[test]
fn soal_52() {
    let harga_eth: f64 = 3_450.751281721821281;

    println!("Harga ETH: {}", harga_eth);
}

// SOAL 13
// Buat variable "eth_amount": f64 = 1.575,
// "harga_eth": f64 = 3_450.75,
// hitung total USD, konversi ke i64 menggunakan "as", tampilkan keduanya.
// Clue: nama function -> fn soal_13(), wajib pakai attribute #[test]

#[test]
fn soal_53() {
    let eth_amount: f64 = 1.575;
    let harga_eth: f64 = 3_450.75;
    let total = eth_amount * harga_eth;
    let konversi_data = total as i64;
    println!("Harga ETH AMOUNT: {}", eth_amount);
    println!("Harga ETH: {}", harga_eth);

    println!("Nilai asli sebelum konversi: ${}", total);
    println!("Nilai sesudah di konversi: ${}", konversi_data);
}

// SOAL 14
// Buat variable "rupiah": i64 = 50_000_000,
// "kurs": f64 = 16_350.0 (1 USD = Rp 16.350),
// hitung nilai dalam USD (rupiah as f64 / kurs), tampilkan keduanya.
// Clue: nama function -> fn soal_14(), wajib pakai attribute #[test]
#[test]
fn soal_54() {
    let rupiah: i64 = 50_000_000;
    let kurs: f64 = 16.350;
    let hitung_nilai = rupiah as f64 / kurs;
    println!("hitung nilai : {}", hitung_nilai);
}

// SOAL 15
// Buat variable mutable "saldo_usd": f64 = 1_000.0,
// tampilkan, kurangi 250.75, tampilkan, tambah 500.0, tampilkan.
// Clue: nama function -> fn soal_15(), wajib pakai attribute #[test]
#[test]
fn soal_55() {
    let mut saldo_usd: f64 = 1_000.0;

    saldo_usd = saldo_usd - 250.75 + 500.0;

    println!("Nilai Asli: {}", saldo_usd);
    println!("Nilai Setelah: {}", saldo_usd);
}

// SOAL 16
// Buat variable "is_verified": bool = true dan "username": &str = "rustacean",
// "saldo": i64 = 10_000_000,
// tampilkan: "User: {}, Verified: {}, Saldo: Rp {}"
// Clue: nama function -> fn soal_16(), wajib pakai attribute #[test

#[test]
fn soal_56() {
    let is_verified: bool = true;
    let username: &str = "rustocean";
    let saldo: i64 = 10_000_000;

    println!("User: {username}, Verified: {is_verified}, Saldo: Rp {saldo}");
}

// SOAL 17
// Lakukan shadowing:
// "harga" = 3_450_i64 (harga dalam USD integer),
// shadow ke f64 = 3_450.0,
// shadow ke String representation dengan format tampilkan "$3450.0"
// Clue: nama function -> fn soal_17(), wajib pakai attribute #[test]
#[test]
fn soal_57() {
    let harga = 3_450_i64;
    println!("harga 1: {}", harga);
    let harga = 3_450.0;
    println!("harga 2: {}", harga);
    let harga = "$3450.0";
    println!("harga 3: {}", harga);
}

// SOAL 18
// Buat variable "persen_pajak": f64 = 0.11 (PPN 11%),
// "harga_barang": i32 = 2_500_000,
// hitung pajak (harga_barang as f64 * persen_pajak),
// hitung total (harga_barang as f64 + pajak),
// tampilkan semua: harga, pajak, total.
// Clue: nama function -> fn soal_18(), wajib pakai attribute #[test]
#[test]
fn soal_58() {
    // cari persen pajak!
    let persen_pajak = 0.11;
    let harga_barang = 2_500_000;

    let konversi_tipe_data_barang = harga_barang as f64;

    let nilai_pajak = konversi_tipe_data_barang * persen_pajak;
    let hitung_harga_with_pajak = konversi_tipe_data_barang + nilai_pajak;

    println!("PPN: {}", persen_pajak);
    println!("HARGA BARANG ASLI : {}", harga_barang);

    println!("NILAI PAJAK DIKTEHAUI: {}", nilai_pajak);
    println!(
        "NILAI HARGA BARANG DENGAN NILAI PAJAK: {}",
        hitung_harga_with_pajak
    );
}

// SOAL 19
// Buat variable "is_bull_market": bool = true,
// "harga_eth": f64 = 3_450.75,
// jika bull market harga naik 20% — hitung harga_baru = harga_eth + (harga_eth * 0.20),
// tampilkan: "Bull Market: {}, Harga ETH: ${}, Prediksi: ${}"
// Clue: nama function -> fn soal_19(), wajib pakai attribute #[test]
#[test]
fn soal_59() {
    let is_bull_market = true;
    let harga_eth = 3_450.75;
    let harga_baru = harga_eth + (harga_eth * 0.20);

    println!(
        "Bull Market: {is_bull_market}, Harga ETH: ${harga_eth}, Prediksi Elon Musk: ${harga_baru}"
    );
}

// SOAL 20
// Buat variable "nama_token": &str = "ETH",
// "harga": f64 = 3_450.75,
// "amount": f64 = 5.25,
// "kurs_idr": f64 = 16_350.0,
// hitung total_usd = harga * amount,
// hitung total_idr = total_usd * kurs_idr,
// konversi total_idr ke i64 menggunakan "as",
// tampilkan semua dalam format yang rapi.
// Clue: nama function -> fn soal_20(), wajib pakai attribute #[test]

#[test]
fn soal_60() {
    let nama_token = "ETH";
    let harga = 3_450.75;
    let amount = 5.25;

    let kurs_idr = 16_350.0;

    let hitung_total_dalam_bentuk_usd = harga * amount;
    let hitung_total_dalam_bentuk_idr = hitung_total_dalam_bentuk_usd * kurs_idr;

    let konversi_tipe_data_dalam_bentuk_integer = hitung_total_dalam_bentuk_idr as u16;

    println!("NAMA TOKEN ELON MUSK : {}", nama_token);

    println!("HARGA ASLI : ${}", harga);
    println!("AMOUNT SALDO : ${}", amount);

    println!("KURS IDR : {}", kurs_idr);

    println!(
        "TOTAL DALAM BENTUK USD (HARGA X AMOUNT): ${}",
        hitung_total_dalam_bentuk_usd
    );
    println!(
        "TOTAL DALAM BENTUK IDR (FLOATING/ANGKA DESIMAL): Rp.{}",
        hitung_total_dalam_bentuk_idr
    );
    println!(
        "HARGA DALAM BENTUK IDR (ANGKA BILANGAN BULAT): Rp.{}",
        konversi_tipe_data_dalam_bentuk_integer
    );
}

// SOAL 1
// Budi punya uang Rp500_000, lalu dia belanja dan menghabiskan Rp175_000.
// Berapa sisa uang Budi sekarang?
// Simpan ke variable "sisa_uang", tampilkan.
// Clue: nama function -> fn soal_1(), wajib pakai attribute #[test]

// SOAL 2
// Harga 1 kg apel adalah Rp25_000.
// Ani membeli 4 kg apel.
// Berapa total yang harus Ani bayar?
// Simpan ke variable "total_bayar", tampilkan.
// Clue: nama function -> fn soal_2(), wajib pakai attribute #[test]

// SOAL 3
// Ada 3 orang patungan untuk beli kado.
// Total harga kado adalah Rp450_000.
// Berapa masing-masing orang harus bayar?
// Simpan ke variable "bayar_per_orang", tampilkan.
// Clue: nama function -> fn soal_3(), wajib pakai attribute #[test]

// SOAL 4
// Toko elektronik memberikan diskon 15% untuk laptop seharga Rp8_500_000.
// Berapa harga laptop setelah diskon?
// Simpan ke variable "diskon" dan "harga_akhir", tampilkan keduanya.
// Clue: nama function -> fn soal_4(), wajib pakai attribute #[test]

// SOAL 5
// Dimas punya gaji Rp6_500_000 per bulan.
// Dia rutin menabung Rp1_200_000 dan bayar kos Rp800_000 setiap bulan.
// Berapa uang Dimas yang tersisa setelah tabungan dan bayar kos?
// Simpan ke variable "sisa_gaji", tampilkan.
// Clue: nama function -> fn soal_5(), wajib pakai attribute #[test]
