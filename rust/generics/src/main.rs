
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x, 
            y
        }
    }
}

// conditionally implementing a method for Pair struct
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// types which implements Partial ordering and Copy trait
// like char, integers fixed sized types
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!(" largest number is {}", result);

    let char_list = vec!['a', 'b', 'c', 'q'];

    let result = largest(&char_list);

    println!(" largest character is {}", result);

}
