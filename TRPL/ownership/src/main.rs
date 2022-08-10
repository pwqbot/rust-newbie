use std::io::Bytes;

fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello");
    let mut s2 = s;
    let sm = &s2;
    println!("{}", sm);
    let sn = &mut s2;
    let mut ss = "sadfsaf";
    first_word(ss);

    // let s: str = "fsadf";
    // println!("{}", s);
    test_enum();
}

struct User {
    active: bool,
    username: String,
    sign_in_count: i32,
}

impl User {
    fn area(&self) -> u32 {
        1
    }
}

enum op<T> {
    None,
    Some(T),
}

enum wd {
    Ok,
    Nono,
}

fn test_enum() {
    let some_number = Some(5);
    let some_string = Some("i am a strign");
    let absent_number: Option<i32> = None;

    let sb = what(true);
    // let sb = what(false);
    let sbb = sb.unwrap();
    let mut sbbb = wd::Ok;
    sbbb = wd::Nono;
}

fn what(wd: bool) -> Option<i8> {
    if wd {
        return None;
    } else {
        return Some(5);
    }
}

fn first_word(s: &str) -> &str {
    let mut user1 = User {
        active: true,
        username: String::from("???"),
        sign_in_count: 1,
    };
    user1.active = false;

    let user2 = User {
        sign_in_count: user1.sign_in_count,
        // active: true,
        // username: user1.username,
        ..user1
    };

    println!("{}", user1.active);
    println!("{}", user2.username);

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
