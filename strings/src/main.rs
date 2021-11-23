fn main() {
    let mut my_string = String::from ("How's it going? My name is CJ. ");
    //length
    println!("length {}",my_string.len());
    //is empty
    println!("String is empty {}",my_string.is_empty());

    for token in my_string.split_whitespace(){
        println!("{}",token);
    }
    println! ("Does the string contain 'CJ' {}",my_string.contains("CJ"));

    my_string.push_str("Welcome to a tutorial on Strings!");

    println!("{}",my_string);



}
