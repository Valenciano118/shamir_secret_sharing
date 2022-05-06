use std::io::{self, Write};
fn main() {
    let mut text = String::from("Introduce your secret:");
    let secret:String = text_input(&text);
    
    if secret.len()==0{
        panic!("The secret must have at least 1 character");
    }

    text = String::from("Introduce the number of parts to divide the secret:"); 
    let number_of_parts:u16 = match text_input(&text).parse(){
        Ok(value) => value,
        Err(_) => panic!("Number of parts introduced wasn't a number")
    };

    if number_of_parts <=1{
        panic!("Number of total parts has to be bigger than 1")
    }

    text = String::from("Introduce the number of parts needed to form the secret back:");
    let number_of_sol_parts:u16 = match text_input(&text).parse(){
        Ok(value) => value,
        Err(_) => panic!("Number of parts introduced wasn't a number")
    };

    if number_of_sol_parts <=1{
        panic!("Number of parts to form the secret has to be bigger than 1");
    }
    if number_of_sol_parts > number_of_parts {
        panic!("Number of parts to form the secret back is larger than the total number of parts");
    }

    println!("\nThe message is: \"{}\"\nYou need {} out of {} parts to reveal the secret",secret,number_of_sol_parts,number_of_parts);
    

    
}

fn text_input(out_text: &String) -> String{
    print!("{} ",out_text);
    io::stdout().flush().unwrap();
    let mut in_text = String::new();
    io::stdin()
        .read_line(&mut in_text)
        .expect("Failed to read the line");
    
    match in_text.pop(){
        Some(_) => in_text,
        None => String::from("")
    }
}
