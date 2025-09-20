fn main() {
    // 문자열 데이터를 숫자로 변환하기 위해서는 컴파일러가 어떤 타입으로 변환해야하는지 정보가 필요하다.
    let guess: u32 = "50".parse().expect("Not a number");

    // 정수 타입 (부호 있는 정수, 부호 없는 정수)
    // 부호 있는 정수: i8, i16, i32, i64, i128, isize
    // 부호 없는 정수: u8, u16, u32, u64, u128, usize
    let n_i8: i8 = -1;
    let n_i16: i16 = -1;
    let n_i32: i32 = -1;
    let n_i64: i64 = -1;
    let n_i128: i128 = -1;
    let n_isize: isize = -1;

    // let temp_u8: u8 = -1; // Error
    let n_u8: u8 = 1;
    let n_u16: u16 = 1;
    let n_u32: u32 = 1;
    let n_u64: u64 = 1;
    let n_u128: u128 = 1;
    let n_usize: usize = 1;
    // println!("calc : {}", n_u8 + n_i8); // Error
    // println!("calc : {}", n_u8 + n_u16); // Error
    // println!("calc : {}", n_u8 + n_u8); // Ok

    // 부동 소수점 숫자
    let n_f32: f32 = 1.0;
    let n_f64: f64 = 1.0;

    // 불리언
    let b_true: bool = true;
    let b_false: bool = false;

    // 문자
    // 문자 타입은 작은 따옴표로 표시된다.
    // 기본 크기는 4바이트이다.
    let c = 'z';
    let z: char = 'ℤ'; // 명시적인 타입 어노테이션
    let heart_eyed_cat = '😻';

    // 문자열
    let s_string: String = String::from("String");
    let s_str: &str = "String";

    // 기본 타입
    let number = 1; // i32
    let float = 1.0; // f64

    // 복합 타입(튜플)
    let tuple: (i32, f64, u8) = (500, 3.14, 1); // tuple타입
    let (num_i32, float_f64, num_u8) = tuple;

    // 패턴 매칭을 이용해 튜플의 값을 해체할 수 있다.
    println!("num_i32: {}", num_i32);
    println!("float_f64: {}", float_f64);
    println!("num_u8: {}", num_u8);

    // 복합 타입(배열)
    let arr = [1, 2, 3, 4, 5]; // 배열타입
    let arr2 = [3; 5]; // [3, 3, 3, 3, 3]

    println!("arr: {:?}", arr);
    println!("arr2: {:?}", arr2);
}
