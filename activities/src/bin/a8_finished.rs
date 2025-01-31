// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Orange, 
    Grape,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor:: Orange => println!("flavor: orange"),
        Flavor:: Grape => println!("flavor: grape"),
    }
    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let orange = Drink {
        flavor: Flavor:: Orange,
        fluid_oz: 6.0
    };
    print_drink(orange);
    let grape = Drink {
        flavor: Flavor:: Grape,
        fluid_oz: 10.0,
    };
    print_drink(grape);
}

