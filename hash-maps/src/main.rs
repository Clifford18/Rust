use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    //add values
    marks.insert("Rust", 90);
    marks.insert("Web Dev", 75);
    marks.insert("Ux Design", 60);

    //find length of hashmaps
    println!("How many subjects have you studied? {}", marks.len());

    //get a single value
    match marks.get("Web Dev") {
        Some(mark) => println!("you got {} for Wb Dev", mark),
        None => println!("you did not study Web Dev")
    }

    //remove a value
    marks.remove("Ux Design");

    //loop through HashMap
    for(subject, mark) in &marks{
        println!("For {} you got {}%",subject,mark);
    }

   // check for value
    println!("Did you study C++? {}", marks.contains_key("C++ Programming"));

}
