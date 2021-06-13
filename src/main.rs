use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex, MutexGuard};

fn main() {
    let shared_counter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let mut handles:Vec<JoinHandle<()>> = Vec::new();
    for i in 1..=10 {
        let shared_counter_ref:Arc<Mutex<u32>> = shared_counter.clone();
        handles.push(thread::spawn(move || {
            let mut c:MutexGuard<u32> = shared_counter_ref.lock().unwrap();
            *c += 1;
            println!("Hello, from thread. {}, {:?}", i, c);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *shared_counter.lock().unwrap());
}
