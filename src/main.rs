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
