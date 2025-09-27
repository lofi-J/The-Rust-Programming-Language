// 수학 관련 함수들을 모아둔 모듈

pub fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract_i32(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply_i32(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide_i32(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}
