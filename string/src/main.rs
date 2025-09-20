fn take_ref_str(s: &str) {}

fn take_ref_string(s: &String) {}

fn main() {
    let string: String = String::from("String"); // String타입
    let literal: &str = "literal"; // str타입
    let slice = &string[..]; // str타입

    // &String 타입을 매개변수로 받는 함수
    take_ref_string(&string);
    // take_ref_string(&str); // Error
    // take_ref_string(&slice); // Error

    // &str 타입을 매개변수로 받는 함수
    take_ref_str(&string); // Ok
    take_ref_str(literal); // Ok
    take_ref_str(slice); // Ok
}
