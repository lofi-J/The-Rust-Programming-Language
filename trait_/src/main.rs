/**
 * Trait
 * 트레이트를 사용하면 공통된 기능을 추상적으로 정의할 수 있다. (like Interface)
 * 트레이트 바운드(Trait Bound)를 이용해 어떤 제네릭 타입 자리에 특정한 동작을 갖춘 타입이 올 수 있음을 명시할 수 있다. (like Type Narrowing)
 */

// 어떤 트레이트를 정의하는 법(시그니처만)
trait Speak {
    fn speaking(&self) -> (); // 함수 시그니처를 정의
}

// 트레이트 레벨에서 구현
trait Eat {
    fn eating(&self) -> () {
        println!("I am eating.");
    }
}

struct Person {
    name: String,
    age: u8,
}

impl Speak for Person {
    fn speaking(&self) -> () {
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}

impl Eat for Person {}

// 매개변수로서의 트레이트
fn do_something(item: &(impl Speak + Eat)) -> () {
    item.speaking();
    item.eating();
}

// 트레이트 바운드 문법
fn do_something2<T: Speak + Eat>(item: &T) {
    item.speaking();
    item.eating();
}

// 트레이트 바운드 문법(where 키워드 사용, 여러 트레이트 바운드 문법을 정리해 표현 가능)
fn do_something3<T, U>(item1: &T, item2: &U)
where
    T: Speak,
    U: Eat,
{
    item1.speaking();
    item2.eating();
}

// 트레이트를 구현하는 타입을 반환하기
fn return_something() -> impl Speak {
    Person {
        name: String::from("Generated Name"),
        age: 255,
    }
}

fn main() {
    let person = Person {
        name: String::from("John"),
        age: 20,
    };

    person.speaking();
    person.eating();

    do_something(&person);
    do_something2(&person);
    do_something3(&person, &person);

    let person2 = return_something();
    person2.speaking();
}
