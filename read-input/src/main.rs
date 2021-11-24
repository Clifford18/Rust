use std::io;


fn main() {
    let  mut input = String::new();
    println!("Hey mate! Say something : \n");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("Sucess! You said: {}", input);
        },
        Err(e) => println!("Ooops! Something went wrong: {}",e)
    }
    println!("Hey mate! Say something Else : \n");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("Sucess! You said: {}", input.to_uppercase());
        },
        Err(e) => println!("Ooops! Something went wrong: {}",e)
    }


}
