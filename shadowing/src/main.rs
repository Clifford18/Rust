fn main() {
    let x  = 10;
    {
        let x =  15;
        println!("x inside : {}", x);
    }
    println!("x inside : {}", x);

    let x ="Xis a string";
    println!("x is : {}", x);

    let x =true;
    println!("x is : {}", x);
}
