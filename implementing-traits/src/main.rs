struct Person {
    name: String,
    age: u8,
}
impl ToString for Person{
    fn to_string(&self) -> String{
        return format!("My name is {} and i am {}",self.name,self.age);
    }

}


fn main() {
    let myself = Person { name: String::from("CJ"), age: 30 };
    println!("{}", myself.to_string());
}
