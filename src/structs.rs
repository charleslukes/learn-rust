struct Color {
    red: u8,
    green: u8, 
    blue: u8
}

struct  Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn new(first: &str, last: &str, age: u8) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            age
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub fn run () {
    let color: Color = Color {
      red: 255,
      green: 50,
      blue: 70
    };

    println!("Red is: {} \nGreen is: {} \nBlue is: {}", color.red, color.green, color.blue);

    let person = Person::new("Charles", "Chiakwa", 26);
    println!("My name is {} {}, I am {} years old", person.first_name, person.last_name, person.age);
    println!("My full names are {}", person.full_name());
}