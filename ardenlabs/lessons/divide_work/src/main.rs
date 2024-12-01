use std::thread;
fn main() {
    const N_THREADS: usize = 8;
    let data: Vec<u64> = (0..1275001).collect();
    let mut thread_handles = Vec::new();
    let chunks = data.chunks(N_THREADS);

    chunks.into_iter().for_each(|chunk| {
        let my_chunk = chunk.to_owned();
        let new_thread = thread::Builder::new()
            .stack_size(std::mem::size_of::<usize>() * 1000)
            .spawn(move || my_chunk.iter().sum::<u64>())
            .unwrap();
        // thread::spawn(move || my_chunk.iter().sum::<u64>());
        thread_handles.push(new_thread);
    });

    let mut sum: u64 = 0;
    thread_handles.into_iter().for_each(|handle| {
        sum += handle.join().unwrap();
    });
    // let sum = data.iter().sum::<u64>();

    println!("{sum}")
}
