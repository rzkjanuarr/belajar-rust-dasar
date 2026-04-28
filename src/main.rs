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
