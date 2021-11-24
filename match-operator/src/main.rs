fn main() {
    let  number =11;
    match number{
        1 => println!("it is one!"),
        2 => println!("there is two of them!"),
        3..=9 => println!("It is between 3 and 9 "),
        10|11 => println!("It is either 10 or 11 "),
        _ => println!("It doesn't match!")
    }
    let name = "CJ";
    match name {
        "Cj" => println! ("Nice name, mate!"),
        "CJ" => println! ("Nice name, CJ!"),
        _ => println! ("Dont know your name!")

    }

}
