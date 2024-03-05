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

fn say_dog() -> String {
    let dog = "woof";
    dog.to_string()
}

println!("{}",say_dog());

let (a, b) = (5, 10);
println!("{}", a*b);

let int: i32 = 32;
let bint: u128 = int.try_into().unwrap();
println!("{bint}");

// SCOPE

let c = 11;
{
    let c = 22;
    println!("c i statement block = {c}");
}
println!("c uden for block ={c}");


let s1 = String::from("hello"); // s1 owns the string
let len = calculate_length(&s1); // s1 is borrowed (immutable reference)
println!("The length of '{}' is {}.", s1, len);

let s2 = String::from("hello");
let len2 = calculate_length2(s2); // Ownership of s1 is moved to the function

// println!("The length of '{}' is {}.", s1, len); // This would cause a compile-time error because s1 is no longer valid here
println!("The length of the string is {}.", len2);







}

// This function takes a reference to a String and returns its length
// without taking ownership of it, so s1 remains valid after the call.
fn calculate_length(s: &String) -> usize {
    s.len()
}

// This function takes ownership of the String, so it is dropped when the function ends.
fn calculate_length2(s: String) -> usize {
    s.len()
}