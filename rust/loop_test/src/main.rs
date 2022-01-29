fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!(" count {}", count);
        let mut remaining = 10;

        loop {
            println!(" remaining = {}", remaining);

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("final value {}", count);

    let mut counter = 0;

    // this is similar to reduce
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!(" result is {}", result);

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!(" value of item is: {}", element);
    }

    for element in (1..4).rev() {
        println!(" value of item is {}", element);
    }
}
