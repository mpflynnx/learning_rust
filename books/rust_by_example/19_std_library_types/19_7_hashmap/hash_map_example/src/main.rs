use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "01321 435723" => "The number you have dialed cannot be connected.",
        "01234 984352" => "You have reached the pizza department of PizzaLand.",
        "11111 222222" => "Hi it's Jacob, please leave a message after the tone.",
        _ => "Please dial a valid number",
    }
}

fn main() {
    // get a HashMap with a default initial capacity of 0
    let mut contacts = HashMap::new();
    // Inferred as HashMap<&str, &str>` in this example)

    //  .insert()
    contacts.insert("Luke", "01321 435723");
    contacts.insert("John", "01234 984352");
    contacts.insert("Rose", "02345 985685");
    contacts.insert("Lizzy", "07983 756742");
    contacts.insert("Jacob", "11111 222222");

    match contacts.get(&"Jacob") {
        Some(&number) => println!("Calling contact... {}", call(number)),
        _ => println!("Don't have contacts number."),
    }

    // `HashMap::insert()` returns `None`
    // if the inserted value is new, `Some(value)` otherwise
    // updates value
    contacts.insert("Jacob", "01634 164674");

    match contacts.get(&"Jacob") {
        Some(&number) => println!("Calling contact... {}", call(number)),
        _ => println!("Don't have contacts number."),
    }

    // .remove()
    contacts.remove(&"Jacob");

    // Iterate all contacts, and call the number
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
}
