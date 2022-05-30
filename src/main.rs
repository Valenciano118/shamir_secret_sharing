use std::io::{self, Write};
use shamir_secret_sharing::*;
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

    let test_value = 1234 as f64;
    let test_hashed_value = calculate_hash(&test_value.to_string());
    let test_cyphered_value = cipher_message(test_hashed_value, &test_value.to_string());

    println!("The hash value of {} is {:?} and the cypher is {:?}",test_value,test_hashed_value,test_cyphered_value);

    let test_value = 12345 as f64;
    let test_hashed_value = calculate_hash(&test_value.to_string());
    let test_cyphered_value = cipher_message(test_hashed_value, &test_value.to_string());

    println!("The hash value of {} is {:?} and the cypher is {:?}",test_value,test_hashed_value,test_cyphered_value);

    
}


fn text_input(prompt_text: &String) -> String{
    print!("{} ",prompt_text);
    io::stdout().flush().unwrap();
    let mut in_text = String::new();
    io::stdin()
        .read_line(&mut in_text)
        .expect("Failed to read the line");
    
    if in_text.ends_with("\n"){
        in_text.pop().unwrap();
        if in_text.ends_with("\r"){
            in_text.pop().unwrap();
        }
    }
    in_text
}
