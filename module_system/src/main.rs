/*
    모듈 시스템 동작 방식
    1. 크레이트 루트 파일에서 시작(main.rs, lib.rs)
    2. 크레이트 루트 파일에서 새로운 모듈을 선언할 수 있음. (mod some_module)
    3. 크레이트 루트 파일이 아닌 곳에서는 서브 모듈을 선언할 수 있음. (mod some_module)
    4. 공개, 비공개 (pub mod, mod)
    5. 모듈을 가져온 후 사용할 때 긴 경로를 단축하기 위해 use 키워드를 사용

    super 키워드를 통해 상위 모듈의 경로를 가져올 수 있음 ../과 비슷
    as 키워드를 통해 동일한 이름의 모듈들을 식별할 수 있음.
    * (glob) 연산자를 통해 정의된 모든 공개 항목을 가져올 수 있음.
*/

use std::fmt::Result;
use std::io::Result as IoResult;
pub mod garden;

use crate::garden::vegetables;

fn function1() -> Result {
    Ok(())
}
fn function2() -> IoResult<()> {
    Ok(())
}

mod front {
    pub mod back {
        pub fn add() {}
    }
    mod inner {
        pub fn test() {
            super::back::add();
        }
    }

    fn test123() {
        crate::front::inner::test();
    }
}

pub fn consumer() {
    front::back::add();
}

fn main() {
    let plant = garden::Plant::Carrot;
    let asparagus = vegetables::Asparagus {};
    println!("plant: {:?}", plant);
    println!("asparagus {:?}", asparagus);

    consumer();
}
