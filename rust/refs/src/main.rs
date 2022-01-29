fn main() {
    simple_literal_string();
    string_in_heap();
    string_borrowing();
    string_borrowing_and_update();
    calculate_first_word();

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// no issues here
fn simple_literal_string() {
    // Already this is borrowing enabled by default since type is &str
    let s1:&str = "hello World";
    // same as borrowing
    let s2 = s1;
    println!(" s1 , {} ; s2 , {}", s1, s2);
}

fn string_in_heap() {
    // growing string, so this is allocated inside heap
    let s1:String = String::from("hello");
    // ownership moved to s2, s1 is dropped from stack
    let s2 = s1;
    println!(" s2 , {}", s2);
}

fn string_borrowing() {
    // s1 is the owner
    let s1:String = String::from("hello");

    // borrowing s1, so s2 and s1 points to same location
    let s2:&str = &s1;
    println!(" s1 , {} ; s2 , {}", s1, s2);
}

fn string_borrowing_and_update() {
    let mut s1:String = String::from("hello");
    let s2:&mut String = &mut s1;

    // s1 should have only one mutable owner at a point
    // let s3:&mut String = &mut s1;

    // s2 already has borrowed s1 as mutable 
    // and it is used last in 47, so we cant update here the s1
    // s1.push_str(", update from s1");

    // we cant do this since s2 is just an address to String type
    // let s2:&str = &s1;
    // s2.push_str(", world");
    s2.push_str(", world from s2 ");

    // same reason as in line 41
    // println!("s2 = {}; s1 = {}", s2, s1);

    println!("s2 = {}", s2);

    // s2 is dropped from scope now we are free to modify s1
    s1.push_str(", update from s1");
    println!("s2 = {}", s1);
}

fn calculate_first_word() {
    let s = String::from("hello world");
    let word:&str = first_word(&s);
    // empties the str, but slice of data is borrowed at 63 and ending at 65
    // s.clear(); 
    println!("the first word is: {}", word);
}

// function that returns a first word by borrowing the string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // enumerate returns a tuple with index and borrowed item
    for (index, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            // string slices
            return &s[0..index];
        }
    }

    &s[..]
}
