// 모듈 선언
pub mod math;

// 테스트 모듈 선언 (테스트 환경에서만 컴파일)
#[cfg(test)]
mod tests;

// 기본 테스트들
#[cfg(test)]
mod basic_tests {
    #[test]
    fn it_works() {
        let result = 2 + 3;

        // assert_eq!는 두 값이 같은지 확인하는 매크로
        // assert_ne!는 두 값이 다른지 확인하는 매크로
        // 두 값이 같으면 아무일도 일어나지 않지만 다르면 panic!을 발생시킴
        assert_eq!(result, 5, "assert_eq! failed result must be 5");
        assert_ne!(result, 6, "assert_ne! failed result must be 5");
    }

    #[test]
    fn exploration() {
        assert!(4 == 2 + 2, "assert! 4 == 2 + 2 failed"); // assert! 매크로는 bool타입을 받아 값이 false면 panic!을 발생시킨다.
    }

    // should_panic 어노테이션은 함수가 panic!을 발생시키는지 확인하는 어노테이션
    // 의도한 것과는 다른 이유로 패닉이 발생하더라도 should_panic 테스트는 통과할 수 있음으로
    // expected매개변수를 추가해 포함되어야 하는 실패 메세지를 지정해 테스트를 작성 할 수도 있다.
    #[test]
    #[should_panic(expected = "ABC")]
    fn should_panic() {
        fn panic_trigger() {
            // panic!("something is wrong"); 테스트 실패
            panic!("ABC"); // 테스트 통과
        }
        panic_trigger();
    }
}
