use tokio::*;
use tokio::task::JoinHandle;

#[derive(Clone, Copy)]
enum ComputationResult {
    Divisable,
    NotDivisable,
    NotEnded,
}

#[tokio::main]
async fn main() {

    let delta = std::time::Instant::now();
    
    let number = 1002151;

    let mut tasks: Vec<JoinHandle<()>> = Vec::new();
    let mut chunks_result: Vec<bool> = Vec::new();

    setup_chunks(number, 8, &mut tasks);

}

fn setup_chunks(number: i32, split: u32, array: &mut Vec<JoinHandle<()>>) {
    println!("{}", 1 * ((1/split) as i32) * number);
    let mut a = 2.0;
    for i in 0..split {
        println!("Chunk {} from {} to {}", i, (((i as f32)/(split as f32)) * (number as f32) + a) as usize, ((((i as f32)+1.0)/(split as f32)) * (number as f32)) as usize);
        array.push(task::spawn(chunk_operation(number, (((i as f32)/(split as f32)) * (number as f32) + a) as usize, ((((i as f32)+1.0)/(split as f32)) * (number as f32)) as usize, i)));
        a = 0.0;
    }
}

async fn chunk_operation(n: i32, start: usize, end: usize, chunk_number: u32) {
    for i in start..end {
        if n % (i as i32) == 0 {
            panic!();
        } else {
            println!("chunk {} : {} % {} ok", chunk_number, n, i);
        }
    }
}