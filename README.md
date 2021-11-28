### Sorting in Rust
Sorting a 32 bit unsigned integer vector by using:
- Radix Sort
- Quick Sort
- Merge Sort
- Inbuilt Sort (Probably Tim Sort)

This program takes a vector length as input and generates an array of random numbers of that size.
Then it sorts this vector with each of the sorting algorithms and reports the time.

How to run?
```bash
cargo run --release <length of vector to sort>
```

Observations:
- Radix Sort is the fastest with linear time complexity
