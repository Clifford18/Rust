struct Person{
    name:String,
    age:u8
}
trait HasVoiceBOx{
    //speak
    fn speak(&self);
    //check if can speak
    fn can_speak(&self)-> bool;
}
impl HasVoiceBOx for Person{
    fn speak(&self) {
        println!("Hello my name is {} and can I speak? {}",self.name,self.can_speak());
    }
    fn can_speak(&self) -> bool {
        if self.age>0{
            return true;
        }return false;
    }
}
fn main() {
    let person1 = Person{name:String::from("CJ"),age:0};
    println!("Can {} Speak? {}", person1.name, person1.can_speak());
    person1.speak();

}
