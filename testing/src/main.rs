// lib.rs의 math 모듈에서 함수를 가져오기
use testing::math::add_i32;

fn main() {
    println!("Hello, world!");
    println!("add_i32(1, 2) = {}", add_i32(1, 2));
    println!("subtract_i32(5, 3) = {}", testing::math::subtract_i32(5, 3));
    println!("multiply_i32(3, 4) = {}", testing::math::multiply_i32(3, 4));

    match testing::math::divide_i32(10, 2) {
        Some(result) => println!("divide_i32(10, 2) = {}", result),
        None => println!("divide_i32(10, 2) = Division by zero!"),
    }
}
