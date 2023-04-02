use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Calling itself");
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter for state {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    // Options

    let some_number = Some(5);
    let some_char = Some('a');
    println!("Plus one: {}", plus_one(some_number).unwrap());

    let absent_number: Option<i32> = None;

    let coin = Coin::Quarter(UsState::Alabama);
    println!("Value of coin is {}", value_in_cents(&coin));
    println!("Coin was {:?}", coin);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Max is {}", max);
    }

    let v = vec![[1, 2, 3]];

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let mut s = String::from("_initial contents_");
    s.push_str("more stuff");
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let tic_tac_toe = format!("{}-{}-{}", s1, s2, s3);
    println!("format is {}", tic_tac_toe);

    for (i, ch) in tic_tac_toe.chars().enumerate() {
        println!("{} char is {}", i, ch);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 30);
    scores.insert(String::from("Red"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Yellow")).or_insert(60);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
