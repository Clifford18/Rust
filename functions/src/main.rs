fn main() {
    print_number_to(10);

    if is_even(30){
        println!("It is even")
    }
}
fn print_number_to(num: u32) {
    for n in 1..num {
        println!("{}", n);
    }
}
fn is_even (num:u32) -> bool{
    return num%2 ==0 ;
}