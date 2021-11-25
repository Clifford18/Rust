fn main() {
    //replace
    {
        let my_string = String::from("Rust is fantastic");
        println!("Before replace : {}", my_string);
        println!("After replace : {}", my_string.replace("fantastic", "great"));
        println!("Now mystring is : {}", my_string);
    }

    //line method
    {
        let my_string1 = String::from("The weather is\n nice \n outside mate");
        for line in my_string1.lines() {
            println!("Lines : [  {}  ]", line);
        }
    }

    //splits
    {
        let my_string = String::from("Leave+a+like+if+you+enjoyed!");
        let tokens: Vec<&str> = my_string.split("+").collect();
        println!("my string : {}", my_string);
        println!("At index 2 : {}", tokens[2]);
    }

    //trim
    {
        let my_string = String::from("          My name is CJ     ");
        println!("Before trim : {}", my_string);
        println!("After trim : {}", my_string.trim());
    }

    //char
    {
        let my_string = String::from("CJ on git hub");

        //get character at index
        match my_string.chars().nth(3) {
            Some(c) => println!("chareacter at index 4: {} ",c),
            None => println!("No chareacter at index 4")

        }
    }
}