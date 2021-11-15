fn main() {
    let animals = vec!["rabbit", "Dog","cat"];
    for  (index, a) in animals.iter().enumerate(){
        println!("the index is {} animal name is {}",index, a);
    }

    let numbers =30..51;
    for  i in numbers{
        println!("the number is {}",i);
    }

    for  i in 1..11{
    println!("the number is {}",i);
    }

}
