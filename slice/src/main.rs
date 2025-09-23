fn main() {
    let s = String::from("hello world");

    let str = "일이삼사오";

    println!("s length: {}", &s.chars().count());

    println!("str length: {}", &str.chars().count());

    let hello = &s[0..5];
    let blank = &s[5..=5];
    let world = &s[6..];

    println!("{}", hello);
    println!("{}", blank);
    println!("{}", world);

    let test = "안녕하세요. 저는 홍길동입니다.";

    println!("test length: {}", &test.len()); // bytes 개수
    println!("test length: {}", &test.chars().count()); // 문자 길이
}
