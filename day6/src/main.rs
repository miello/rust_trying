fn main() {
    let data = "HELLO WORLD";
    
    let display_data = data.to_string();

    // UTF-8 Supported
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    let hello = String::from("สวัสดี");

    let mut s = String::from("foo");
    s.push_str(data);

    println!("{}", s);

    let s1 = String::from("Hello ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;

    let a1 = String::from("A");
    let a2 = String::from("B");
    let a3 = String::from("C");

    let result_a = format!("{}-{}-{}", a1, a2, a3);

    // Note that String = Wrapper of Vec<u8>
    // let idx = s1[0];

    let hello = "Здравствуйте";

    // Panic !!! -> char boundary 2 byte per character
    // let s = &hello[0..1];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    println!("{} {} {}", s3, s2, result_a);
}
