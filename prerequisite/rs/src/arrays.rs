use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [7, 8, 9, 10, 11];

    // Re-assign value
    numbers[2] = 69;

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}