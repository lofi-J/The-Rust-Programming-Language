enum Coin {
    Penny(i32),
    Dummy,
}

fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("Max: {}", max),
        _ => println!("config_max is None"),
    }

    let some_num = Some(100u8);
    if let Some(x) = some_num {
        println!("x: {}", x);
    }

    let coin = Coin::Penny(999);
    let d = Coin::Dummy;
    if let Coin::Penny(some_val) = coin {
        println!("Penny: {}", some_val);
    }
    if let Coin::Dummy = d {
        println!("Dummy");
    }
}
