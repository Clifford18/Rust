extern crate reqwest;

fn main() {
    let response_text = reqwest::get("https://docs.rs/crate/reqwest/0.8.8/hhhh")
        .expect("Couldn't make requestt")
        .text().expect("Couldn't read the response text");
    println!("Response Text: {}",response_text);
//     match reqwest::get("https://docs.rs/crate/reqwest/0.8.8/jjj"){
//         Ok(mut response)=>{
//             if response.status() == reqwest::StatusCode::Ok {
// match response.text() {
//     Ok(text)=>println!("Response text: {}",text),
//     Err(_)=>println!("Could not make read response text!")
//
// }
//             }else {
//                 println!("Response was not 200 OK");
//             }
//
//         }
//         Err(_)=>println!("could not make the response")
//     }

}
