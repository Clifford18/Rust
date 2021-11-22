fn main() {
    let mut x = 10;
    {
        let mcj = &mut x;//mutable refrences
        *mcj += 1;
    }


    println!("x is {}", x);
}
