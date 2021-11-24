fn main() {
    let my_vector1: Vec<i32> = Vec::new();
    let mut my_vector = vec![1, 2, 3];
    println!("Vector [2] :{}", my_vector[2]);
    my_vector.push(49);
    my_vector.remove(2);

    for number in my_vector.iter() {
        println!("{}", number);
    }
}
