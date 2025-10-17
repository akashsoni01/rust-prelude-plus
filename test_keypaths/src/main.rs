use key_paths_derive::Keypaths;

#[derive(Keypaths, Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Keypaths, Debug, Clone)]
struct Address {
    city: String,
    country: String,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        address: Address {
            city: "New York".to_string(),
            country: "USA".to_string(),
        },
    };

    // Test basic keypath access
    let name_keypath = Person::name_r();
    let age_keypath = Person::age_r();
    let city_keypath = Person::address_r().then(Address::city_r());

    println!("Created keypaths successfully");

    // Test accessing values
    if let Some(name) = name_keypath.get(&person) {
        println!("Person name: {}", name);
    }

    if let Some(age) = age_keypath.get(&person) {
        println!("Person age: {}", age);
    }

    if let Some(city) = city_keypath.get(&person) {
        println!("Person city: {}", city);
    }

    // Test mutable access
    let mut person_mut = person.clone();
    if let Some(name) = name_keypath.get_mut(&mut person_mut) {
        *name = "Bob".to_string();
        println!("Changed name to: {}", name);
    }
}