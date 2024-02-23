fn main() {
    println!("Hello, world!");

    // Creating an empty vector of i32 type
let mut numbers: Vec<i32> = Vec::new();

// Creating a vector with initial values
let names = vec!["Alice", "Bob", "Charlie"];

// Adding elements to the vector
numbers.push(10);
numbers.push(20);
numbers.push(30);

// Printing the vector
println!("{:?}", names); // Prints: ["Alice", "Bob", "Charlie"]
println!("{:#?}", numbers); // Pretty prints the vector with each element on a new line

for name in names.iter() {
    println!("{}",name);
}

for pik in numbers.iter_mut() {
    *pik += 1;
    println!("{}", pik);
}

}