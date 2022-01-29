fn main() {
    // utf-8 encoded format
    // variable byte length for each character
    // to support multiple languages
    let _s = String::new();
    let _s = "initial data".to_string();

    let s2 = " another string pushed";
    let mut s = String::from("initial data");

    // not taking ownership of s2, type is &str - slice borrowing
    s.push_str(s2);
    s.push('l');
    println!(" s2 {}", s2);

    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");

    // s3 ownership is moved and s4 is borrowed
    let s5 = s3 + &s4;

    // cant use s3 here
    println!("s5 {}, {}", s5, s4);

    // better way to concatenate
    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");

    // indexing into string chars wont always work
    // because of utf-8 encoding where there variable bytes
    // for single character
    // let s1 = String::from("hello");
    // let h = s1[0];

    // macro takes reference only borrowing
    let s9 = format!("{}-{}-{}", s6, s7, s8);
    println!("s9 {}", s9);

    let hello = String::from("नमस्ते");

    for c in hello.chars() {
        println!("{}", c);
    }

    for c in hello.bytes() {
        println!("{}", c);
    }

    println!(" String: {}", s);
    println!(" Hello in hindi:  {}", hello);
}
