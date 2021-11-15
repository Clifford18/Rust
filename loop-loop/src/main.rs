fn main() {
    let mut n =0;
    loop {

        if n>=10 {
            break;
        }
        n+=1;

        if n==5 {
            continue;
        }
        println!("the value of n = {}",n);



    }
}
