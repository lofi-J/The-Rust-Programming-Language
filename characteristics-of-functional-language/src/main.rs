use std::thread;

fn main() {
    // 클로저는 함수 처럼 매개변수에 타입을 명시할 수 있다.
    let closure_example = |x: i32| x + 1;
    dbg!(closure_example(4));

    // 클로저의 타입 추론
    let closure_example2 = |x| x;

    let e1 = closure_example2(1); // 해당 시점에서 closer_example2 클로저의 매개변수 x의 타입이 추론된다.
    dbg!(e1);
    // let e2 = closure_example2(2.0); // Error: x의 타입이 고정되어 f64가 허용되지 않음.

    println!("--------------------------------");

    // 클로저의 캡처
    // 클로저는 함수가 매개변수를 취하는 3가지 방법과 직접적으로 대응된다. (소유권 획득, 불변 참조, 가변 참조)
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    println!(
        "--------------------------------list라는 이름의 벡터에 대해 불변 참조자를 캡처, 그저 출력을 위함 이므로 소유권을 획득하지 않았음.\n"
    );

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(4); // 가변 참조 생성

    // println!("Before calling closure: {list:?}"); // Error: 가변 참조가 존재하는 동안 다른 참조(불변 포함)를 만들 수 없다(Rust borrow checker)
    borrows_mutably();
    println!("After calling closure: {list:?}");

    println!("--------------------------------");

    // 클로저가 소유권을 갖도록 해야하는 경우 매개변수 리스트 전에 move 키워드를 사용할 수 있다.
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let handle = thread::spawn(move || println!("{list:?}")); // 새로운 스레드를 생성해 클로저를 실행
    handle.join().unwrap(); // 스레드가 종료될 때까지 대기
    // let test_closure = move || println!("{list:?}");
    // test_closure();
    // println!("After defining closure: {list:?}"); // Error: list는 이미 소유권이 넘어갔기 때문에 유효하지 않다.

    println!("--------------------------------");

    // 캡처된 값을 클로저 밖으로 이동하기와 Fn 트레이트
    // FnOnce는 한 번만 호출될 수 있는 클로저에게 적용됩니다. 모든 클로저들은 호출될 수 있으므로, 최소한 이 트레이트는 구현해 둡니다. 캡처된 값을 본문 밖으로 이동시키는 클로저에 대해서는 FnOnce만 구현되며 나머지 Fn 트레이트는 구현되지 않는데, 이는 이 클로저가 딱 한 번만 호출될 수 있기 때문입니다.
    // FnMut은 본문 밖으로 캡처된 값을 이동시키지는 않지만 값을 변경할 수는 있는 클로저에 대해 적용됩니다. 이러한 클로저는 한 번 이상 호출될 수 있습니다.
    // Fn은 캡처된 값을 본문 밖으로 이동시키지 않고 캡처된 값을 변경하지도 않는 클로저는 물론, 환경으로부터 아무런 값도 캡처하지 않는 클로저에 적용됩니다. 이러한 클로저는 자신의 환경을 변경시키지 않으면서 한번 이상 호출될 수 있는데, 이는 클로저가 동시에 여러 번 호출되는 등의 경우에서 중요합니다.

    trait MyOptionTrait<T> {
        fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T;
    }

    impl<T> MyOptionTrait<T> for Option<T> {
        fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T,
        {
            match self {
                Some(v) => v,
                None => f(),
            }
        }
    }
}
