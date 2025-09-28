/// Rust의 스마트 포인터
/// 포인터 즉, 메모리 주소를 가지고있는 변수에 대한 일반적인 개념으로 Rust에서는 & (참조자)가 있지만 이는 단순히 값을 참조한는것 이상의 기능은 없다.
/// 반면 스마트 포인터는 포인터(&)처럼 동작함뿐만 아니라 추가적인 메타데이터와 기능을 제공한다.
/// 참조자와 스마트 포인터 사이에는 추가적인 차이점이 존재하는데 참조자가 데이터를 빌리기만 하는 반면, 대부분의 경우 스마트 포인터는 가리킨 데이터를 소유한다. (위 2개의 자료구조도 스마트 포인터이다. String, Vec<T>)
/// 스마트 포인터는 보통 struct로 구현되어있으며 보통의 구조체와는 달리 스마트 포인터는 Deref와 Drop 트레이트를 구현한다.
///
/// std 라이브러리에 존재하는 가장 기본적인 스마트 포인터
/// 1. Box<T>
/// 2. Rc<T>
/// 3. RefCell<T>, Ref<T>, RefMut<T>
/// 4. Mutex<T>

mod box_example {
    // Box<T>를 사용해 Heap에있는 데이터를 카리키기
    // 사용되는 경우
    // 1. 컴파일 타임에는 크기를 알 수 없는 타입이 있는데, 정확한 크기를 요구하는 컨텍스트 내에서 그 타입의 값을 사용하고 싶을 때
    // 2. 커다란 데이터를 가지고 있고 소유권을 옮기고 싶지만 그렇게 했을 때 데이터가 복사되지 않을 것을 보장하고 싶을 때 (크기가 큰 데이터는 Heap과 Stack에 복사되고 이동하는데 소요되는 리소스가 크다 Box를 이용해 Heap에 저장된 데이터가 Stack에 작은양의 데이터만 가지게 된다)
    // 3. 어떤 값을 소유하고 이 값의 구체화된 타입보다는 특정 트레이트를 구현한 타입이라는 점만 신경 쓰고 싶을 때 (어떤 값을 소유한 후 값의 타입보다는 기능에 집중하고싶을 때)

    trait Vehicle {
        fn drive(&self);
    }

    #[derive(Debug)]
    struct Truck {
        // Why Error?
        // Truct구조체의 Option<Trcuk>은 재귀적으로 1byte(enum 태그) + 1byte + ... 무한한 사이즈를 가지게 되므로 컴파일이 불가능
        // next_truck: Option<Truck>, // Error: recursive type `Truck` has infinite size

        // Why possible?
        // Box<T>를 이용해 Heap에 값을 저장할 경우 stack에는 8byte의 고정된 참조 데이터만 저장되기 때문에 가능해진다.
        pub next_truck: Option<Box<Truck>>, // Ok
    }

    impl Vehicle for Truck {
        fn drive(&self) {
            println!("Truck is driving, next_truck: {:?}", self.next_truck);
        }
    }

    pub fn main() {
        // Box를 사용하는 경우 1, 3번에 해당하는 예제
        let t: Box<dyn Vehicle>; // Vehicle trait를 구현한 타입을 가리키는 변수
        t = Box::new(Truck { next_truck: None });
        t.drive();
    }
}

mod rc_example {
    use std::rc::Rc;

    // RC<T>는 참조 카운트를 관리하는 스마트 포인터로 마지막 참조 범위를 벗어날 때까지 해당 메모리를 활성 상태로 유지

    #[derive(Debug)]
    struct Truck {
        capacity: u32,
    }

    pub fn main() {
        let truck1 = Rc::new(Truck { capacity: 100 });
        let truck2 = Rc::new(Truck { capacity: 200 });
        let truck3 = Rc::new(Truck { capacity: 300 });

        let facility_one = vec![Rc::clone(&truck1), Rc::clone(&truck2), Rc::clone(&truck3)]; // 여기서 Rc::clone을 통해 참조 카운트를 증가시키고 소유권 문제를 해결
        let facility_two = vec![Rc::clone(&truck2), Rc::clone(&truck3)]; // vec는 Copy 트레이트를 구현하지 않는 타입의 경우 소유권을 소유하기 때문에 Rc::clone를 하지 않았다면 에러가 발생

        println!("Ref_count: {}", Rc::strong_count(&truck1));
        println!("Ref_count: {}", Rc::strong_count(&truck2));
        println!("Ref_count: {}", Rc::strong_count(&truck3));

        println!(
            "facility_one: {:?}",
            facility_one
                .iter()
                .map(|t| t.capacity)
                .collect::<Vec<u32>>()
        );
        println!(
            "facility_two: {:?}",
            facility_two
                .iter()
                .map(|t| t.capacity)
                .collect::<Vec<u32>>()
        );
    }
}

mod refcell_example {
    // 내부 가변성(Interior Mutability)은 어떤 데이터에 대한 불변 참조자가 있을 때라도 데이터를 변경할 수 있게 해주는 러스트의 디자인 패턴이다.
    // 보통 이러한 동작은 러스트의 기본 대여 규칙을 위반하기 때문에 불가능하지만 데이터를 변경하기 위해서, unsafe 코드를 사용하여 변경과
    // 대여를 자유롭게 할 수 있다. (unsafe코드는 코드 검사를 컴파일러에게 맡기는 대신 수동으로 하는 중임을 컴파일러에게 알린다)
    // 컴파일러는 대여 규칙을 준수함을 보장할 수 없을지라도, 우리가 이를 런타임에 보장할 수 있는 경우라면 내부 가변성 패턴을 쓰는 타입을 사용할 수 있다.

    pub trait Messager {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messager> {
        messager: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messager,
    {
        pub fn new(messager: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messager,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messager.send("Error: you are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messager
                    .send("Warning: You've used up to 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messager
                    .send("Warning: You've used up to 75% of your quota!");
            } else if percentage_of_max >= 0.5 {
                self.messager
                    .send("Warning: You've used up to 50% of your quota!");
            } else if percentage_of_max >= 0.25 {
                self.messager
                    .send("Warning: You've used up to 25% of your quota!");
            } else if percentage_of_max >= 0.1 {
                self.messager
                    .send("Warning: You've used up to 10% of your quota!");
            } else {
                self.messager
                    .send("Warning: You've used up to 5% of your quota!");
            }
        }
    }

    pub fn main() {
        // RefCell<T>를 가지고서 대여 규식의 불변성을 어기게되면 panic을 일으키고 종료된다.
        // RefCell<T>는 싱글 스레드 시나리오에서만 사용이 가능하고 멀티 스레드 시나리오에서는 사용이 불가능하다.(컴파일 에러)(방법은 존재)

        // let x = 5;
        // let y = &mut x; // Error x는 불변이므로 변경이 불가능하다.
    }
}

mod deref_example {
    // deref 트레이트를 구현해 임의의 타입을 참조자처럼 다룰 수 있다.
    // deref 트레이트를 구현해 *연산자로 역참조를 할 수 있다.
    use std::ops::Deref;

    #[derive(Debug)]
    struct MyBox<T> {
        value: T,
    }

    impl<T> MyBox<T> {
        fn new(value: T) -> MyBox<T> {
            MyBox { value }
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    pub fn main() {
        let some_num = 610;
        let my_box = MyBox::new(610);

        // assert_eq!(some_num, *my_box); // `MyBox<{integer}>` cannot be dereferenced
        // deref 트레이트 구현 후
        assert_eq!(some_num, *my_box); // 역참조를 통해 해당 구조체의 value를 참조할 수 있도록 트레이트를 구현했다.
    }
}

mod drop_example {
    // drop트레이트를 구현하면 어떤 값이 스코프 밖으로 벗어나려고 할 때 어떤일을 수행할지 커스터마이징 할 수 있다.
    // 어떠한 타입이든 drop트레이트를 구현할 수 있고 해당 기능은 스마트 포인터를 구현할 때 거의 항상 이용되기 때문에 중요하다.
    // 예를 들어 Box<T> 가 버려질 때 해당 Box가 가리키는 Heap공간을 해제
    // Drop트레이트는 프렐루드에 포함되어있어 직접 가져오지 않아도 된다.

    #[derive(Debug)]
    pub struct CustomDrop {
        data: String,
    }

    impl Drop for CustomDrop {
        fn drop(&mut self) {
            println!("Dropping CustomDrop data: {}", self.data);
        }
    }

    pub fn main() {
        println!("main start");
        {
            println!("scope start");
            let custom_drop1 = CustomDrop {
                data: "Lofi-J".to_string(),
            };
            let custom_drop2 = CustomDrop {
                data: "Lofi-J2".to_string(),
            };

            println!("CustomDrop data: {}", custom_drop1.data);
            println!("CustomDrop data: {}", custom_drop2.data); // 먼저 drop된다.
            // drop이 호출되는 순서는 만들어진 순서의 역순이다.
            println!("scope end");
        }
        println!("main end");
    }
}

fn main() {
    println!("---------------- Box Example ----------------");
    box_example::main();

    println!("---------------- RC Example ----------------");
    rc_example::main();

    println!("---------------- RefCell Example ----------------");
    refcell_example::main();

    println!("---------------- Deref Example ----------------");
    deref_example::main();

    println!("---------------- Drop Example ----------------");
    drop_example::main();
}

// 사실 대부분의 현실 상황에서는 Heap보다는 Stack에 값을 저장하는것이 더 낫다.
// 하지만 Stack에 데이터를 저장하기 위해서는 컴파일러가 데이터의 크기를 알아야하는데 이는 컴파일 타임에 알 수 없는 타입의 경우에는 불가능하다.
