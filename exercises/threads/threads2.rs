// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 需要一个互斥锁来锁定
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 定义一个互斥锁，然后进行修改
            let mut lock = status_shared.lock().unwrap();
            lock.jobs_completed += 1
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // 取出来最后的状态
        let final_status = status.lock().unwrap();
        println!("jobs completed {}", final_status.jobs_completed);
    }
}
