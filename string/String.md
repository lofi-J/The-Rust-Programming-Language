# Rust의 문자열

Rust에는 2가지의 핵심 문자열 타입이 있다.

- String
- str

이 둘 모두 연속적인 메모리 영역에 저장된 임의의 길이를 가지는 UTF-8 문자 시퀀스를 나타낸다.

## 소유권(Ownership) 관점에서의 차이점

**String** 타입은 힙 메모리(Heap)에 할당된 문자열 데이터를 **소유**한다. 변수가 스코프를 벗어나게되면 자동으로 해제된다.

**str** 타입은 문자열 자체를 소유하지 않고 **참조**만 한다. 즉, 실제 데이터는 String, Heap, Stack등에 존재한다.

## 메모리 관점에서의 이해

**String** 항상 Heap에 동적으로 할당되며, 런타임에 그 크기가 결정된다.

**str** str타입 자체는 직접 생성하는 것이 불가하다. <br>
아래와 같이 문자열 리터럴에서 생성되거나, String의 대여로 사용된다. (함수의 매개변수도)

```rust
let literal: &str = "literal";
let ref_str: &str = &string;
```

## str VS &str의 관계

- `str`: 크기를 알 수 없는 타입 (Dynamically Sized Type)
- `&str`: str에 대한 참조(실제로 사용하게 되는 타입)

위와 같은 관계는 배열과 슬라이스의 관계와 비슷하다.

## &String과 &str

아래의 예제 처럼 &str 타입이 유연해 더 많은 타입을 받을 수 있다.
&String을 &str로 러스트 컴파일러가 자동으로 변환해 줄 수 있기 때문이다. 하지만 그 반대로 &str을 &String으로 변환이 불가하다.

```rust
  let string: String = String::from("String");
  let literal: &str = "literal";
  let slice = &string[..]; // str타입

  // &String 타입을 매개변수로 받는 함수
  take_ref_string(&string);
  // take_ref_string(&str); // Error
  // take_ref_string(&slice); // Error

  // &str 타입을 매개변수로 받는 함수
  take_ref_str(&string); // Ok
  take_ref_str(literal); // Ok
  take_ref_str(slice); // Ok
```

## 정리

문자열을 수정하거나 소유권이 필요한 경우 `String` 타입을 사용해 가변 길이의 문자열 타입을 사용하면 되는것이고, 소유권이 필요없고 읽기 전용으로 문자열 참조나 함수 매개변수로 사용될 때 `str`(실제로 사용될 때는 `&str`) 타입을 사용하면 효율적으로 Rust에서 문자열을 사용할 수 있다.
