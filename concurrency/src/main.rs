use std::thread;
use std::time::{Duration, Instant};

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
            for i in 1..=10 {
                println!("thread1 is running: [{i}]");
                thread::sleep(Duration::from_secs(1));
            }
        });

        for i in 1..=5 {
            println!("main thread is running: [{i}]");
            thread::sleep(Duration::from_secs(1));
        }
        handle.join().expect("Thread1 panicked");

        let duration = start.elapsed();
        println!("duration: {:?}", duration);
    }
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        println!("vec: {:?}", vec);
    });

    handle.join().expect("Thread1 panicked");

    spawn_example::main();
}
