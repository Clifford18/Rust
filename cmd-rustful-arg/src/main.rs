use std:: env;

fn main() {
    let arg: Vec<String> = env::args().collect();

    for arguments in arg.iter(){
        println!("{}",arguments);
    }

}
