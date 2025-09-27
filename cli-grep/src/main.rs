use std::{env, process};

use cli_grep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect(); // collect()는 반복자를 이용해 특정 컬렉션으로 변환 해당 라인에서는 타입 어노테이션을 명시해 Vec<String>로 변환

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// 표준 출력 스트림을 output.txt파일로 리디렉션 시키고 프로그램을 실행하면 output.txt파일에 출력되고
// 에러 출력은 정상적으로 터미널 화면에 표시된다.
