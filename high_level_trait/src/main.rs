pub trait Iterator {
    type Item; // 자리 표시자

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    println!("counter: {:?}", counter.next().unwrap());
    println!("counter: {:?}", counter.next().unwrap());
    println!("counter: {:?}", counter.next().unwrap());
    println!("counter: {:?}", counter.next().unwrap());
    println!("counter: {:?}", counter.next().unwrap());
}
