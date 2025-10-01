/// unsafe 키워드를 통한 안전하지 않은 코드 작성
/// 안전하지 않은 코드를 러스트에서 작성한다는 것은 null pointer 역참조와 같은 메모리
/// 불안정성을 허용한다.
///
/// unsafe 키워드로 가능한것
/// 1. 원시 포인터 (raw pointer) 역참조
/// 2. 안전하지 않은 함수 혹은 메서드 호출
/// 3. 가변 정적 변수에 접근 및 수정
/// 4. 안전하지 않은 트레이트 구현
/// 5. union의 필드 접근
///
/// unsafe 키워드를 사용한다고 대여 검사기나 다른 안정성 검사를 비활성화 하지는 않는다

// Rust에서 전역 변수는 정적(static) 변수라고 부른다.
pub static mut SOME_VALUE: i32 = 0;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32; // 불변 원시 포인터 (메모리 주소)
    let r2 = &mut num as *mut i32; // 가변 원시 포인터 (메모리 주소)

    unsafe {
        println!("r1: {:?}", *r1); // 불변 원시 포인터 역참조 (값)
        println!("r2: {:?}", *r2); // 가변 원시 포인터 역참조 (값)
    }

    // unsafe function 정의
    unsafe fn dangerous() {
        println!("dangerous");
    }

    // unsafe 블록에서 호출
    unsafe {
        dangerous();
    }

    // 안전하지 않은 코드를 감싸는 안전한 추상화 만들기
    // let mut v = vec![1, 2, 3, 4, 5];
    // let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);

    // unsafe {
    //     assert_eq!(a, &mut [1, 2, 3]);
    //     assert_eq!(b, &mut [4, 5, 6]);
    // }

    // 전역 변수 사용
    fn increase_static_value() {
        unsafe {
            SOME_VALUE += 1;
        }
    }

    unsafe {
        let pointer = std::ptr::addr_of!(SOME_VALUE);
        println!("SOME_VALUE: {}", *pointer);
        increase_static_value();
        let t = SOME_VALUE;
        println!("SOME_VALUE: {}", t);
    }
}
