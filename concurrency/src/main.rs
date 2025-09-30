/// Rust의 동시성
/// 1. 스레드를 생성해 여러 코드 조각을 동시에 실행하기
/// 2. 채널들의 스레드간 메시지를 보내는 (메시지 - 패싱) 동시성
/// 3. 여러 스레드가 어떤 동일한 데이터에 접근할 수 있는 상태 - 공유(shared-state) 동시성
/// 4. 사용자 정의 타입으로 확장하는 Sync와 Send 트레이트

mod spawn_example {
    use std::{
        thread,
        time::{Duration, Instant},
    };

    pub fn main() {
        let start = Instant::now();

        let handle = thread::spawn(|| {
            for i in 1..=3 {
                println!("thread1 is running: [{i}]");
                thread::sleep(Duration::from_secs(1));
            }
        });

        for i in 1..=3 {
            println!("main thread is running: [{i}]");
            thread::sleep(Duration::from_secs(1));
        }
        handle.join().expect("Thread1 panicked");

        let duration = start.elapsed();
        println!("duration: {:?}", duration);
    }
}

mod channel_example {
    use std::{sync::mpsc, thread, time::Duration}; // mpsc는 multiple producer, single consumer의 약자

    pub fn main() {
        let (tx, rx) = mpsc::channel::<String>();

        let copied_tx1 = tx.clone();
        thread::spawn(move || {
            // tx의 소유권이 스레드로 이동하기 때문에 move 키워드 사용
            let var = String::from("척추혈관 혈액 공급: (from: copied_tx1)");
            copied_tx1.send(var).unwrap();
        });

        let copied_tx2 = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            let var = String::from("말초혈관 혈액 공급: (from: copied_tx2)");
            copied_tx2.send(var).unwrap();

            drop(tx); // clone을 통해 여러 송신자를 생성한 경우 송신이 완료되었을 때 원본 송신자를 명시적으로 drop해줘야한다.
        });

        // for문은 채널이 받힐때까지 수신대기를 진행한다.
        for received in rx {
            println!("received: {}", received);
        }
    }
}

// 메세지 패싱은 동시성을 다루는 좋은 도구지만, 유일한 수단은 아니다.
// Mutex를 통한 메모리 공유
// 1. 데이터를 사용하기전에 lock을 획득
// 2. 데이터를 사용한 후 lock을 해제
mod mutex_example {
    use std::{
        sync::{Arc, Mutex}, // Arc는 Atomic Reference Counted의 약자
        thread,
    };

    pub fn main() {
        let mutex = Mutex::new(5); // { data: 5, poisoned: false }

        {
            // .lock()을 통해 lock 획득 요청을 보냄
            // 요청을 보내면 lock을 획득하기 전 까지 해당 스레드는 멈추게된다.
            let mut num = mutex.lock().unwrap();
            *num = 6;
            drop(num);
        }

        println!("number: {:?}", mutex.lock().unwrap()); // { data: 6, poisoned: false}
    }

    pub fn multi_thread() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}

/// std::marker 트레이트인 Sync와 Send는 언어에 내장되어있다.
/// Send 마커 트레이트는 Send가 구현된 타입의 소유권이 스레드 사이에서 이동될 수 있음을 나타낸다. (대부분이 이에 해당하지만 Rc<T>와 같은 예외도 존재한다)
/// Sync 마커 트레이트는 Sync가 구현된 타입이 여러 스레드로부터 안전하게 참조 가능함을 나타낸다.

fn main() {
    println!("---------------- Spawn Example ----------------");
    spawn_example::main();

    println!("---------------- Channel Example ----------------");
    channel_example::main();

    println!("---------------- Mutex Example ----------------");
    mutex_example::main();
    mutex_example::multi_thread();
}
