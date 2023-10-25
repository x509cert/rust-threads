use std::thread;

fn main() {
    let handles: Vec<_> = (0..20).map(|_| {
        thread::spawn(|| {
            let current_thread = thread::current();
            let thread_id = current_thread.id();
            println!("Thread with ID: {:?}", thread_id);

            let mut x = 0;
            for _ in 0..2_140_000_000 {
                x += 1
            }
            x
        })
    }).collect();

    for h in handles {
        println!("Thread finished with count={}",
        h.join().map_err(|_| "Could not join a thread!").unwrap());
    }
}