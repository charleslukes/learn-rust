// Arrays length are fixed, where elements are the same data types;

pub fn arrays () {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("arrays is {:?}", numbers);
    println!("First index in the array is {}", numbers[0]);
    println!("Arrays length is {}", numbers.len());
}