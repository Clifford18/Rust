fn main() {
    let tup1 = (20, 25, 30, 35);
    println!("Tuple 1 is {}", tup1.2);

    let tup2 = (20, "rust", 3.0, false);
    println!("Tuple 2 is {}", tup2.3);

    let tup3 = (20, "rust", 3.0, false,(1,4,7));
    println!("Tuple 3 is {}", (tup3.4).2);

    let tup4 = (45, 6.7, "computer");
    let (a,b,c)=tup4;
    println!("Tuple a is {}",a);
    println!("Tuple b is {}",b);
    println!("Tuple c is {}",c);
}
