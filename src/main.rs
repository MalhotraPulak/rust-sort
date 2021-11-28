use std::vec::Vec;
use rand::Rng;
use std::time::Instant;
mod radix_sort;
use crate::radix_sort::radix_sort;
mod merge_sort;
use crate::merge_sort::merge_sort;
mod quick_sort;
use crate::quick_sort::quick_sort;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    array_len: u32,
}

fn main() {
    let args = Cli::from_args();
    let count = args.array_len;
    let mut vec =  Vec::<u32>::new();
    let mut rng = rand::thread_rng();
    
    // fill array with random numbers
    for _ in 0..count {
        let a: u32 = rng.gen();
        vec.push(a);
    }

      
    let now = Instant::now();
    let mut vec_clone = vec.clone();
    radix_sort(&mut vec_clone); 
    let elapsed = now.elapsed();
    println!("Radix sort time: {:.2?}", elapsed);

    let now = Instant::now();
    let mut vec_clone_2 = vec.clone();
    merge_sort(&mut vec_clone_2); 
    let elapsed = now.elapsed();
    println!("Merge sort time: {:.2?}", elapsed);

    let now = Instant::now();
    let mut vec_clone_3 = vec.clone();
    quick_sort(&mut vec_clone_3); 
    let elapsed = now.elapsed();
    println!("Quick sort time: {:.2?}", elapsed);

    let now = Instant::now();
    vec.sort();     
    let elapsed = now.elapsed();
    println!("Inbuilt sort time: {:.2?}", elapsed);

    for i in 0..vec.len(){
        // println!("{}", vec_clone_3[i]);
        if vec[i] != vec_clone_2[i] || vec[i] != vec_clone[i] || vec[i] != vec_clone_3[i] { 
            println!("All sorts are not same");
            return;
        }
    }
    println!("All sorts are correct");
}
