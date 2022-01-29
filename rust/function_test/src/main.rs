fn main() {
    println!("Hello, world!");
    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!(" Y is an expression evaluation to {}",y);

    let f = five(y);

    println!(" Value of f {}", f);

    let condition = true;

    // should be same type else results in error
    let number = if condition { 5 } else { 6 };

    println!(" number {}", number);
}

fn five(y: i32) -> i32 {
    //expression so there is no semicolon, expressions by default return the value whereas statements dont return
    y + 1
}

fn another_function(x: i32, ch: char) {
    println!("another function, {}, {}", x, ch);
}
