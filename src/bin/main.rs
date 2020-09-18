use std::thread;
use std::time::Duration;
use ::rust_thread_pool::ThreadPool;
fn main() {

    let pool = ThreadPool::new(4);

    for i in 1..10 {        
        pool.execute(move || {
            do_task(i);
        });
    }

    thread::sleep(Duration::from_secs(10));
}

fn do_task(i: i32) {
    thread::sleep(Duration::from_secs(2));
    println!("task {} done", i)
}
