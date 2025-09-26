/**
 * Generic 코드의 성능
 * 러스트는 컴파일 시점에 제네릭을 사용하는 코드를 monomorphization하여 코드를 실제 구체 타입으로 변환해 수행한다.
 * 즉, 제네릭 코드의 런타임 성능 오베헤드가 존재하지 않아 런타임에서의 성능 차이가 없다.
 */

fn get_largest<T: PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];

    for item in list {
        if largest < item {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<I, F> {
    x: I,
    y: F,
}

impl<I, F> Point<I, F> {
    fn x(&self) -> &I {
        &self.x
    }
    fn y(&self) -> &F {
        &self.y
    }
}

impl Point<f32, f32> {
    fn distance_x_y(&self) -> f32 {
        (self.x - self.y).abs()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("largest: {}", get_largest(&number_list));
    println!("largest: {}", get_largest(&number_list2));

    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.0, y: 2.0 };
    let different_types = Point { x: 1, y: 2.0 };

    println!("integer.x: {:?}, integer.y: {:?}", integer.x, integer.y);
    println!("float.x: {:?}, float.y: {:?}", float.x, float.y);
    println!(
        "different_types.x: {:?}, different_types.y: {:?}",
        different_types.x, different_types.y
    );

    println!("integer.x: {:?}", integer.x());
    println!("integer.y: {:?}", integer.y());

    let float_point = Point { x: 1.0, y: 5.7 };
    println!("float_point.distance_x_y: {:?}", float_point.distance_x_y());
    // let integer_point = Point { x: 1, y: 5 };
    // println!(
    //     "integer_point.distance_x_y: {:?}",
    //     integer_point.distance_x_y()
    // ); // Error distance_x_y는 f32, f32 타입의 인스턴스에서만 사용할 수 있다.
}
