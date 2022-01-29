
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    fn construct_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    use_struct();

    let rect = (30, 50);
    println!(" area of the rectangle using tuples {}", area(rect));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect4 = Rectangle {
        width: 40,
        height: 60,
    };

    let rect5 = Rectangle::construct_square(50);

    println!(" DEBUG: rect: {:?}", rect2);
    println!(" area of the rectangle using struct reference {}", area_with_borrow(&rect2));
    println!(" area of the rectangle using struct method {}", rect2.area());

    println!(" Rect3 can be hold inside Rect2 - {}", rect2.can_hold(&rect3));
    println!(" Rect4 can be hold inside Rect2 - {}", rect2.can_hold(&rect4));

    println!(" Square using associated function, {:?} ", rect5);
}

fn area_with_borrow(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn use_struct() {
    let user1 = User {
        username: String::from("arvi123"),
        email: String::from("aravindsekr@gmail.com"),
        active: true,
        sign_in_count: 1
    };

    println!(" user1 {}, {}, {}, {}", user1.username, user1.email, user1.active, user1.sign_in_count);

    let user2 = build_user(
        String::from("arvi1234"), 
        String::from("arvindsekr@tmail.com")
    );
    println!(" user 2 {}, {}, {}, {}", user2.username, user2.email, user2.active, user2.sign_in_count);

    let user3 = User {
        username: String::from("arvi12345"),
        email: String::from("arvindsekr@ymail.com"),
        ..user2
    };
    println!(" user 3 {}, {}, {}, {}", user3.username, user3.email, user3.active, user3.sign_in_count);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}
