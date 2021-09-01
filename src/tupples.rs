// Tuples group different types
// max 12

pub fn tupple(){
    println!("Hello Tupple");
    let person: (&str, &str, i8) = ("Charles", "Nigeria", 26);
    println!("{} is from {} and he is {}", person.0, person.1, person.2);
}