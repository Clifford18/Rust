use std::process::Command;

fn main() {
    //python cj.py
    let mut cmd = Command::new("python");
    cmd.arg("cj.py");

    //execute the command
    match cmd.output() {
        Ok(o) => {
            unsafe {
                println!("Output : {}", String::from_utf8_unchecked(o.stdout));
            }
        },
        Err(e) => {
            println!("there was an error {}", e)
        }
    }
}
