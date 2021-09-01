pub fn run() {
    println!("Heloo world from print.rs file");
    println!("{} the bad guy", "Charles");
    // positional params
    println!("{0} is cool and {0} like to code", "Charles");
    // name params
    println!("{name} is cool and {name} like to code", name="Charles");
    // debug
    println!("{:?}", (12, true, "hello"));
    // math expression
    println!("5 * 20 = {}", 5 * 20);
}