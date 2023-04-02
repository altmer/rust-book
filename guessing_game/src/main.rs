use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let i500 = tup.0;

    println!("Tuple values x = {x}, y = {y}, z = {z}");

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a = [1; 5];
    fun(i500);
    fun(arr[0]);
    fun(a[4]);

    for element in a {
        println!("the value is: {element}");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // mutable reference
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    // Here’s a small programming problem: write a function that takes a string of words separated by spaces
    // and returns the first word it finds in that string. If the function doesn’t find a space in the string,
    // the whole string must be one word, so the entire string should be returned.

    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {word}");

    // struct
    let mut user1 = User {
        email: String::from("admin@example.com"),
        username: String::from("admin"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("admin@mail.test");
    println!("User email: {}", user1.email);

    let user2 = build_user(String::from("customer@mail.test"), String::from("customer"));
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    // user2 is no longer valid here!!! it has been moved to user3

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    // GUESS THE NUMBER GAME
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn fun(x: i32) -> i32 {
    println!("function fun x = {x}");
    x + 1
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
