// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id_number: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("quantity = {:?}", item.quantity);
}

fn display_id_number(item: &GroceryItem) {
    println!("id_number = {:?}", item.id_number);
}

fn main() {
    let pasta = GroceryItem {
        quantity: 14,
        id_number: 4000, 
    };
    display_quantity(&pasta);
    display_id_number(&pasta);
}

