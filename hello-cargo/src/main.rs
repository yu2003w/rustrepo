

fn main() {
    println!("Hello, world!");
    test_scalar();
    test_string();
    test_reference();
    let s = String::from("Hello world");
    let i = first_word(&s);
    //println!("s is {}, i is {}, first word is {}", s, i, &s[0..i]);
    println!("first word is {}", i);

    let s_lit = "hello world";
    let first = first_word(&s_lit[..]);
    println!("first word is {}", &first);

    println!("first word is {}", first_word(&s));
    
    hello_cargo::teststruct::test_struct();
}

fn test_scalar() {
    let x: u32 = 5;
    let y: i32 = 5/3;
    println!("{x}/3 is {y}");
    let z = 1;
    println!("z is {z}");
}

fn test_string() {
    let mut s = String::from("hello");
    s.push_str(", world");
    //let mut s1 = s;
    // above is move operation, which is shallow copy in rust
    let mut s2 = s.clone();
    println!("s is {s}, and it's copy is {s2}");

    let slen = calculate_len(&s);
    println!("The length of {s} is {slen}");
    change_str(&mut s2);
    println!("s2 is changed to {s2}");
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change_str(s: &mut String) {
    s.push_str(" changed!");
}

fn test_reference() {
    let mut s = String::from("Testing reference");
    {
        let s2 = &mut s;
        println!("s2 is {s2}");
    }
    let s1 = &mut s;
    //let s2 = &mut s;
    //println!("s1 is {s1}, s2 is {s2}");
    //Above code doesn't compile, due to reference restriction.
    // Instead using following code wich introduce other scope to finish that.
   
    println!("s1 is {s1}");

    let r1 = &s;
    let r2 = &s;
    println!("r1 is {}, r2 is {}", r1, r2);
    let r3 = &mut s;
    println!("r3 is {}", r3);
    //println!("r1 is {}, r2 is {}, r3 is {}", r1, r2, r3);
    // s couldn't be borrowed as reference and mutable reference at the same time.
}

/*
fn first_word(s: &String)-> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //return i;
            return &s[..i];
        }
    }
    //s.len()
    &s[..]
}
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
