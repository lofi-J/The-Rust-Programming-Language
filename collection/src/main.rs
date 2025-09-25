fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let vec1 = vec![5, 4, 3, 2, 1];

    for i in &vec1 {
        println!("{i}");
    }

    let mut s1 = String::from("foo");
    let s2 = " bar";
    s1.push_str(s2);

    for byte in s1.bytes() {
        println!("{byte}");
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let some_color = scores.get("Blue").copied().unwrap_or(0);
    println!("some_color: {:?}", some_color);

    for word in "Hello World".split_whitespace() {
        println!("{word}");
    }

    
}
