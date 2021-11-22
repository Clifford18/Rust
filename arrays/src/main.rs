fn main() {
    let numbers= [1, 2, 3, 4, 5];

    let numbers2:[i32;5] = [2;5];

    for n in numbers.iter() {
        println!("{}", n);
    }
    for i in 0..numbers.len() {
        println!("number old {}", numbers[i]);
    }
    for i in 0..numbers2.len() {
        println!("number {}", numbers2[i]);
    }
}
