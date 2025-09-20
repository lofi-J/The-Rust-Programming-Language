fn main() {
    let s = "hello";
    {
        let ss = "TEST";
    }
    println!("{}", s);
    // println!("{}", ss); // Error ss는 스코프 밖으로 벗어낫기 때문에 Drop되었음.

    let s1 = String::from("Hello");
    let s2 = s1; // s1의 소유권이 s2로 이동

    // println!("{}", s1); // Error s1은 이미 소유권이 넘어갔기 때문에 유효하지 않다.
    let s3 = s2.clone();

    println!("{}", s2);
    println!("{}", s3);

    // Stack 데이터 타입의 경우
    let x1 = 5;
    let x2 = x1; // move가 아닌 깊은 copy가 발생

    println!("x1 = {}, x2 = {}", x1, x2);
}
