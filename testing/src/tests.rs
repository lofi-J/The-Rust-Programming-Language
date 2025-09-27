// 수학 모듈의 테스트들을 모아둔 파일

use super::math::*;

#[cfg(test)]
mod math_tests {
    use super::*;

    #[test]
    fn test_add_i32() {
        assert_eq!(add_i32(1, 3), 4, "add_i32(1, 3) must be 4");
        assert_eq!(add_i32(-1, 1), 0, "add_i32(-1, 1) must be 0");
        assert_eq!(add_i32(0, 0), 0, "add_i32(0, 0) must be 0");
    }

    #[test]
    fn test_subtract_i32() {
        assert_eq!(subtract_i32(5, 3), 2, "subtract_i32(5, 3) must be 2");
        assert_eq!(subtract_i32(1, 1), 0, "subtract_i32(1, 1) must be 0");
        assert_eq!(subtract_i32(0, 5), -5, "subtract_i32(0, 5) must be -5");
    }

    #[test]
    fn test_multiply_i32() {
        assert_eq!(multiply_i32(3, 4), 12, "multiply_i32(3, 4) must be 12");
        assert_eq!(multiply_i32(-2, 3), -6, "multiply_i32(-2, 3) must be -6");
        assert_eq!(multiply_i32(0, 100), 0, "multiply_i32(0, 100) must be 0");
    }

    #[test]
    fn test_divide_i32() {
        assert_eq!(
            divide_i32(10, 2),
            Some(5),
            "divide_i32(10, 2) must be Some(5)"
        );
        assert_eq!(
            divide_i32(7, 3),
            Some(2),
            "divide_i32(7, 3) must be Some(2)"
        );
        assert_eq!(divide_i32(10, 0), None, "divide_i32(10, 0) must be None");
        assert_eq!(
            divide_i32(0, 5),
            Some(0),
            "divide_i32(0, 5) must be Some(0)"
        );
    }

    #[test]
    fn test_edge_cases() {
        // 경계값 테스트
        assert_eq!(add_i32(i32::MAX, 0), i32::MAX);
        assert_eq!(add_i32(i32::MIN, 0), i32::MIN);
        assert_eq!(multiply_i32(1, i32::MAX), i32::MAX);
    }
}
