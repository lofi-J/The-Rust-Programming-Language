// Rust에서 iterator는 사용하는 메서드를 호출하여 반복자를 소비하기 전까지는 동작을 하지 않는다
// 많은 부분에서 for문과 같은 반복문을 통해 리스트의 요소들을 순회했는데 이는 내부적으로 반복자를 생성해 소비하는 방식으로 구현되어있다.

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // 기본 구현이 있는 메서드는 생략
// }

fn main() {
    let vec1 = [1, 2, 3];

    // iterator의 next 메서드는 내부 상태를 변경해 현재의 스퀀스를 추적하기 때문에 mut로 선언해야 next 메서드를 호출할 수 있다.
    let mut vec1_iter = vec1.iter(); // vec1을 반복할 수 있는 iter 생성 

    let one = vec1_iter.next();
    let two = vec1_iter.next();
    let three = vec1_iter.next();
    let four = vec1_iter.next(); // None이 반환된다. (모두 소비했기 때문)

    println!("vec1_iter: {vec1_iter:?}");

    println!("one: {one:?}");
    println!("two: {two:?}");
    println!("three: {three:?}");
    println!("four: {four:?}");

    println!("vec1_iter: {vec1_iter:?}");

    let vec2 = [99, 100, 101];

    let total = vec2.iter().sum::<i64>(); // sum 메서드는 내부적으로 반복자를 소비하기 때문에 소비 후 반복자를 사용할 수 없다.

    println!("total: {total:?}");

    // ------------------------------------------------------------------------------

    // loop와 iterator의 성능 차이
    // 러스트의 Iterator는 비용 없는 추상화 (zero-cost abstraction) 중 하나이며, 그 추상을 사용하는 것은 추가적인 런타임 오버헤드가 없다는 것을 의미합니다
}
