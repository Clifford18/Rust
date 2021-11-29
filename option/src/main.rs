fn main() {
    // let name = String::from("CJ");
    // println!("Character at index 8: {}", match name.chars().nth(1){
    //   Some(c)=>c.to_string(),
    //     None => "No character at index 8!".to_string()
    // });
    println!("Occupation is {}",match get_occupation("CJ"){
        Some(o)=>o,
        None =>"No occupation found"
    })
}
fn get_occupation(name: &str)->Option<&str>{
    match name {
        "CJ" => Some("Software Developer"),
        "Micheal"=> Some("Dentist"),
        _=> None
    }

}
