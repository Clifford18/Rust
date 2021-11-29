mod cj {
    fn chicken() {
        println!("Chicken!");
    }

    pub fn print_message() {
        chicken();
        println!("How is it going");
    }

    pub mod water {
        pub fn print_message() {
            println!("I am water");
        }
    }
}

fn main() {
    cj::print_message();
    cj::water::print_message();
}
