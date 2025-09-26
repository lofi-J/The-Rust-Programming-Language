/**
 * Lifetime
 * 라이프타임은 어떤 참조자가 필요한 기간동안 유효함을 보장하도록 하는 개념
 * 대부분의 상황에서 타입이 암묵적으로 추론되듯, 라이프타임도 암묵적으로 추론이 가능하다. 하지만
 * 참조자의 수명이 여러 방식으로 서로 연관될 수 있는 상황에서는 라이프타임을 명시해 주어야 한다.
 */

/*
fn lifetime_test() {
    // 라이프타임으로 댕글링(dangling reference: 엉뚱한 데이터를 참조하게 되는 원인) 방지하기
    let r;
    {
        let x = 5;
        r = &x; // borrowed value does not live long enough
    } // x의 수명이 끝나는 지점

    println!("r: {}", r);
}
*/

// 함수에서의 제네릭 라이프타임
// longest 함수가 매개변수의 소유권을 얻지 않도록 참조자인 문자열 슬라이스를 전달 받음.
// 에러의 원인: 반환값으로 참조자를 반환하지만 x, y어느 참조자를 반환할 지 모름.

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// just_return 함수의 경우 어느 참조자를 반환할지 컴파일러가 알 수 있음 (즉 참조자의 수명을 컴파일러가 보장할 수 있다)
// Rust 컴파일러는 라이프타임 생략 규칙(lifetime elision rules)을 가지고있어
// 컴파일러가 "반환값의 라이프타임 = 입력 매개변수의 라이프타임"이라고 자동으로 추론
fn just_return(x: &str) -> &str {
    x
}

// longest 함수의 라이프타임 명시하기
// 함수는 두 매개변수를 갖고 둘 다 적어도 라이프타임 'a만큼 살아있는 문자열 슬라이스이며,
// 반환하는 문자열 슬라이스도 라이프타임 'a만큼 살아있다는 정보를 명시.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        println!("x is longer");
        x
    } else {
        println!("y is longer");
        y
    }
}

fn main() {
    let string2 = String::from("short string");
    let mut result;
    {
        let string1 = String::from("long string is long");
        // result = longest(string2.as_str(), string1.as_str()); // Error borrowed value does not live long enough
    }

    println!("result: {result}");
    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);
}

// 정리
// 라이프타임 어노테이션은 값의 수명을 보장하는 것이 아니라 이미 존재하는 라이프타임의 관계를 컴파일러에게 명시하는것이다.
// 즉 라이프타임 어노테이션으로 참조자의 수명이 늘어나거나 하지 않는다.
//
// 결국 참조자의 수명은 스코프에 의해 결정된다.
