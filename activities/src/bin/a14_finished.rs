// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


struct Person {
    age: i32,
    name: String,
    color: String, 
}

fn print(data: &str) {
    println!("{:?}", data); 
}

fn main() {
    let people = vec! [
        Person {
            age: 8,
            name: "Courtney".to_owned(),
            color: "blue".to_owned(),
        },
        Person {
            age: 39,
            name: "Jon-Erik".to_owned(),
            color: "green".to_owned(),
        },
        Person {
            age: 10,
            name: "KK".to_owned(),
            color: "blue".to_owned(),
        }
    ]; 
    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.color);
        }
    }
}
