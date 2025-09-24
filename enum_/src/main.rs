/*
    Rust가 IpAddrKind(Enum)의 Copy 트레이트를 암묵적으로 구현.
    -> 즉, 소유권이 이동되지 않고 copy되어 계속 유효한 상태를 가지게되었음.

    Rust가 Copy트레이트를 자동으로 구현하기 위한 조건.
    1. enum의 모든 variant가 데이터를 갖지 않거나 값이 Copy트레이트를 구현하는 타입인경우.
*/
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing to {:?}", ip_kind);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message called {:?}", self);
    }
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: {:?}", four);
    println!("six: {:?}", six);

    route(four);
    route(six);

    let message = Message::Move { x: 10, y: 20 };
    message.call();

    let some_number = Option::Some(5);
}
