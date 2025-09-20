/**
 * Rust에는 프렐루드(prelude)라는 표준 라이브러리에 정의된 아이템 집합을 가지고있다.
 * io 모듈도 프렐루드에 포함되어 있어서 std에서 바로 사용이 가능하다.
 *
 * rand crate에서는 rand::rng()를 사용하기 위해서 Rng가 스코프에 포함되어있어야한다.
 */
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_range = 0..=100;
    let mut rng = rand::rng();
    let secret_number: i32 = rng.random_range(secret_range.clone());

    let mut input_count: i32 = 0;

    println!("Gussing Game Start!");
    println!("The secret number's range is {:?}", secret_range);

    let mut guess = String::new(); // String은 표준 라이브러리에서 제공하는 확장 가능한 UTF-8 인코딩 문자열 타입이다.

    loop {
        println!("Please input your guess...");
        guess.clear(); // 이전 입력 내용 제거

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        input_count += 1; // 입력 횟수 증가

        let guess: i32 = match guess.trim().parse::<i32>() {
            Ok(num) => {
                println!("You guessed: {}", num);
                num
            }
            Err(err) => {
                println!("Please type a number! {}", err);
                continue; // guess.clear() 제거 - 루프 시작에서 이미 처리됨
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You guessed {} times", input_count);
                break;
            }
        }
    }
}
