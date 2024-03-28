#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64
}

fn test_struct() {
    let u1 = User {
        active: true,
        name: String::from("Smith Robert"),
        email: String::from("smithr@gmail.com"),
        sign_in_count: 35
    };
    Println!("u1 is {}", u1);
}

fn build_user(name: String, email: String) -> User {
    User {
        active: true,
        name: name,
        email: email,
        sign_in_count: 1
    }
}

fn build_user1(name: String, email:String) -> User {
    User {
        active: true,
        name,
        email,
        sign_in_count:1
    }
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn test_rectangle() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };
    println!("The area of rectangle {} is {}", &rect, area);
    rect.area();
}

impl Rectangle {
    fn area(&self)-> u32 {
        self.width * self.height
    }

    fn area(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: usize) -> Self {
        self {
            width: size,
            height: size,
        }
    }
}