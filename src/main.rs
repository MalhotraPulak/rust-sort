use std::vec::Vec;
use rand::Rng;
use std::time::Instant;
mod radix_sort;
use crate::radix_sort::radix_sort;
mod merge_sort;
use crate::merge_sort::merge_sort;
mod quick_sort;
use crate::quick_sort::quick_sort;
mod bubble_sort;
use crate::bubble_sort::bubble_sort;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    array_len: usize,
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
    let mut vec_clone_4 = vec.clone();
    bubble_sort(&mut vec_clone_4); 
    let elapsed = now.elapsed();
    println!("Bubble sort time: {:.2?}", elapsed);

    let now = Instant::now();
    vec.sort();     
    let elapsed = now.elapsed();
    println!("Inbuilt sort time: {:.2?}", elapsed);

}


#[test]
fn checker() {
    let mut vec1 = vec![1, 3, 3, 2, 1, 1, 5, 5, 10, 13, 3, 33, 0, 11, 1]; 
    let mut vec2 = vec1.clone(); 
    radix_sort(&mut vec2);
    let mut vec3 = vec1.clone(); 
    quick_sort(&mut vec3);
    let mut vec4 = vec1.clone(); 
    merge_sort(&mut vec4);
    let mut vec5 = vec1.clone(); 
    bubble_sort(&mut vec5);
    vec1.sort();
    assert_eq!(vec1, vec2);
    assert_eq!(vec1, vec3);
    assert_eq!(vec1, vec4);
    assert_eq!(vec1, vec5);
}


