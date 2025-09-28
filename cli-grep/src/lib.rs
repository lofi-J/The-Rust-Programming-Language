use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; // ? 연산자를 활용한 축약형 에러 처리 (사용하기 위해서는 해당 함수가 Result를 반환해야함)

    match config.ignore_case {
        true => {
            for line in search_case_insentive(&config.query, &contents) {
                println!("{line}");
            }
        }
        _ => {
            for line in search(&config.query, &contents) {
                println!("{line}");
            }
        }
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // 첫 번째 요소는 프로그램 이름이므로 무시

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config {
            query,
            file_path,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

// 매개변수로 전달된 contents의 수명만큼 search함수의 반환값이 유효함을 명시
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect() // search 함수의 반환값 명시로 collect() 메서드의 변환 타입을 명시 하지 않아도 됨 (암묵적으로 Vec<&'a str>로 추론)
}

pub fn search_case_insentive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line.trim());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
";

        assert_eq!(
            vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
            search("nobody", contents),
        );
        assert_eq!(
            vec!["How dreary to be somebody!", "How public, like a frog"],
            search("How", contents),
        );
    }

    #[test]
    fn search_case_insentive_test() {
        let query = "rUsT"; // 대소문자 구분 없이 검색하기 위함
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["Rust:"], search_case_insentive(query, contents));
    }
}
