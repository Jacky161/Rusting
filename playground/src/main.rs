fn main() {
    //println!("Hello, world!");
    //ch3();
    //ch4();
    //ch5();
    //ch6();
}

fn ch3() {
    // Variables are immutable by default (not the same as const)
    let x = 5;
    println!("The value of x is: {x}");

    // ILLEGAL
    // x = 6;
    // println!("The value of x is: {x}");

    // Mutable Variables
    let mut y = 6;
    println!("The value of y is: {y}");
    y = 7;
    println!("The value of y is: {y}");

    // Const variables
    // Explicit type is required for const. Normal variables is inferred.
    // Like constexpr in C++
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");


    // Shadowing
    // Like making a new variable, can change the type. Unlike mut.
    let x: i16 = 5;
    let x: i32 = (x as i32) + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Arrays
    let a = [3; 5];

    // Statements and Expressions
    // Expression evaluates to whatever the last line is. Same with functions.
    let y = {
        let x = 3;
        x + 1
    };

    // Control Flow
    let number = 6;

    // Must be bool.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // If is an expression.
    let number = if 6 == 7 { 5 } else { 6 };

    // Infinite loops
    //loop {
    //    println!("again!");
    //}

    // Loop is also an expression.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // Loops can be labelled like so
    'hello: loop {
        'world: loop {
            break 'hello;
        }
    }

    // While loops
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// Understanding Ownership
fn ch4() {
    // s is stored on the heap.
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // this will print `hello, world!`

    let s1 = String::from("hello");

    // Steals s1 into s2. s1 is no longer valid.
    let s2 = s1;
    // Illegal
    //println!("{s1}, world!");

    // Functions can take ownership of variables in the same way if they are passed without a reference.

    // Immutable references.
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    // Mutable references
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    // Illegal. Cannot have two mutable references in use at once. Cannot have immutable and mutable refernces at once.
    // println!("{r1}, {r2}");

    // Slices
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let mut s = String::from("hello world");
    let word = first_word(&s);
    //s.clear(); // error! borrow as mutable reference, but there is a immutable reference.
    println!("the first word is: {word}");

    // Slices of arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// Structs
fn ch5() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Struct update syntax
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Note that this moves username and email
    let user2 = User {
        active: false,
        ..user1
    };

    println!("{}", user1.active);
    // println!("{}", user1.email);  // illegal

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Debugging
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // impl
    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

// Enums
fn ch6() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::Move { x: 62, y: 63 };

    fn plus_one(x: Option<i32>) -> Option<i32> {
        // Must be exhaustive
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // If let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
