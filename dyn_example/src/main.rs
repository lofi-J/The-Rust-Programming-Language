/**
 * Generic Trait Bound
 * 제네릭 타입에 트레이트 바운드를 적용할 수 있다.
 * 이 경우 단형성화가 이루어져 런타임에서의 성능 손실이 없다.
 */
mod generic_trait_bound_example {
    use std::fmt::Debug;

    #[derive(Debug)]
    struct Speaker<T> {
        data: T,
    }

    impl<T: Debug> Speaker<T> {
        fn speak(&self) {
            println!("speaking: {:?}", self.data);
        }
    }

    pub fn main() {
        let speaker = Speaker { data: 123 };
        speaker.speak();

        let speaker2 = Speaker {
            data: "Hello, world!",
        };
        speaker2.speak();
    }
}

/**
 * Trait Object
 * 트레이트 객체를 사용할 때 러스트는 동적 디스패치를 이용하게되고. 컴파일러는 트레이트 객체를 사용 중인 코드와 함께
 * 사용될 수 있는 모든 타입을 알 수 없음으로, 런타임에 트레이트 객체 내에 존재하는 포인터를 사용해 메서드를 호출한다.
 * 이는 런타임 비용이 존재한다.
 */
mod dynamic_dispatch_example {
    trait Animal {
        fn make_sound(&self);
    }

    struct Dog {}
    struct Cat {}

    impl Animal for Dog {
        fn make_sound(&self) {
            println!("Dog is barking");
        }
    }

    impl Animal for Cat {
        fn make_sound(&self) {
            println!("Cat is meowing");
        }
    }

    pub fn main() {
        let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog {}), Box::new(Cat {})];
        for animal in animals {
            animal.make_sound();
        }
    }
}

fn main() {
    generic_trait_bound_example::main();
    dynamic_dispatch_example::main();
}
