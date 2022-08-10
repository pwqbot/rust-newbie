struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("{x}{y}");

    let _ = 10;
    match p {
        // Point { x, y } => println!("asdf"),
        Point { x, y: 0 } => println!("on the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("on neither axis: {} {}", x, y),
    };

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("move");
        }
        Message::Write(text) => {
            println!("{text}");
        }
        Message::ChangeColor(r, g, b) => println!("rgb"),
    }
}
