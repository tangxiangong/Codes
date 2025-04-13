use std::thread;
use std::time::Duration;

pub fn threads() {
    let v = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            for i in 1..10 {
                println!("Hello number {i} from spawned thread 1");
                thread::sleep(Duration::from_millis(1));
            }
            println!("Spawned thread 1 is finished");
        });

        s.spawn(move || {
            for val in &v {
                println!("Value from spawned thread 2: {val}");
                thread::sleep(Duration::from_millis(1));
            }
            println!("Spawned thread 2 is finished");
        });

        s.spawn(|| {
            for i in 1..5 {
                println!("Hello number {i} from spawned thread 3");
                thread::sleep(Duration::from_millis(1));
            }
            println!("Spawned thread 3 is finished");
        });

        println!("Scope is finished");
    });

    for i in 1..5 {
        println!("Hello number {i} from main thread");
    }

    // println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threads() {
        threads();
    }
}
