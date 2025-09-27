use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
};

/**
 * 에러 처리
 * 러스트에서 에러는 복구 가능한 에러와 복구 불가능한 에러 2가지로 나눌 수 있다.
 * 1. 프로그램 실행도중 마딱트린 복구 불가능한 에러 (ex: index out of bounds in array)
 * 2. 프로그래머가 명시적으로 panic!(매크로)를 호출한 경우
 * 3. 특정 함수의 반환값으로 Result<T, E> 타입을 반환해 함수의 에러 처리를 propagation 할 수 있다.
 */

fn main() {
    let v = vec![1, 2, 3]; // index가 2까지 존재하는 벡터
    // println!("v[3] = {}", v[3]); // thread 'main' panicked at src/main.rs:10:28:

    /*
     * C에서는 buffer overread 로 해당 위치의 메모리 주소에 존재하는 어떤 값이든 읽어오지만 이는 공격자의 배열뒤의 인덱스에 접근해 의도적으로 정보를 탈취할 수 있는 보안 취약점이 있다.
     * Rust에서는 이를 막기위해 존재하지 않는 인덱스에 접근하는 것을 막기위해 panic을 발생시킨다.
     */

    /*
     * Rust에서 백트레이스 정보를 확인하고싶다면 환경변수 RUST_BACKTRACE를 0이 아닌 다른 값으로 설정하면된다 (ex: RUST_BACKTRACE=1)
     * 또한 디버그 심볼이 활성화 되어있어야 백트레이스 정보를 확인할 수 있다.
     * 디버그 심볼은 cargo run 명령어나 cargo build 명령어를 수행할 때 --release플래그 없이 실행할 경우 기본적으로 활성화된다.
     */

    // Result로 복구 가능한 에러를 처리하기

    // enum Result<T, E> {
    //   Ok(T),
    //   Err(E),
    // }

    let cargo_toml = File::open("will be not found");
    match cargo_toml {
        Ok(file) => {
            println!("{:?}", file);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found, creating new file(package.json)");
                match File::create("package.json") {
                    Ok(file) => {
                        println!("File created successfully");
                    }
                    Err(error) => {
                        println!("Error creating file: {}", error);
                    }
                }
            }
            _ => {
                println!("Error opening file: {}", error);
            }
        },
    }

    // Rust에서 에러 전파하기(propagate the error)
    fn read_username_from_file() -> Result<String, Error> {
        // Result<T, E> 타입을 반환하는 함수
        let username_file_result = File::open("username.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    let username = read_username_from_file();
    match username {
        Ok(username) => println!("username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }

    // ? 연산자를 활용한 축약형 에러 처리
    // ? 연산자는 사용된 해당 함수의 반환값이 Result<T, E> 타입이라면 에러가 발생할 경우 Err(e)를 반환하고 Ok(t)를 반환하는 것과 같은 역할을 한다.
    fn some_function() -> Result<String, Error> {
        let username = fs::read_to_string("username.txt")?;
        Ok(username)
    }

    let username = some_function().expect("Failed to read username");

    println!("username: {:?}", username);
}
