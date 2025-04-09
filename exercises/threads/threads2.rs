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
    // Arc本身只提供共享不可变数据的能力，无法直接修改其内部的数据，如果需要共享可变数据，
    // 那么需要结合Mutex使用，这样才能允许线程安全的修改共享数据
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        // 线程在spawn之后会立刻启动，
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut status_s = status_shared.lock().unwrap();
            status_s.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        // Answer: Yes, you need to 'join' on all the handles to ensure that all threads
        // have completed their execution before accessing the shared value.
        // Without joining, the main thread might access the shared value before
        // all threads have finished updating it, leading to inconsistent results.
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
