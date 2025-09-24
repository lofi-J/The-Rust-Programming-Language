enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Test {
    V1,
    Val(String),
}

impl Test {
    fn test(&self) {
        match &self {
            Test::V1 => println!("V1"),
            Test::Val(val) => println!("Val: {}", val),
        }
    }
}

fn main() {
    let coin = Coin::Quarter;
    println!("value: {}", value_in_cents(coin));

    let penny = Coin::Penny;
    println!("value: {}", value_in_cents(penny));

    let test1 = Test::V1;
    let test2 = Test::Val(String::from("test"));

    test1.test();
    test2.test();

    // other match
    let some_number = 9;
    match some_number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}
