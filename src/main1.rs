


fn main() {
    // variable declaration
    let x: i32 = 16;
    println!("{}",x);

    let z: String = String::from("Hello, Soroban!");
    let y: &str = "Hello, Stellar!";

    println!("{y}");
    println!("{z}");

    //functions

    // pub fn Event(name: String) {
    //     let name: String = String::from("James");
    //     println!("{}",name);
    // }

    let e = EventForKids{
        name: String::from("Kidco"),
        date: String::from("2021-09-01"),
        number_of_participants: 10,
        age: 10,
        location: String::from("Lagos"),
    };
    
    //add your enum in here

}

//structs

struct EventForKids {
    name: String,
    date: String,
    number_of_participants: i32,
    age: i32,
    location: String,
}

//enums -> compiling error in one class

enum ErrorsForEvent{
    NoEvent,
    CancelledEvent,
    EventType
}