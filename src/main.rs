use std::io::{self, Write};
use shamir_secret_sharing::*;
use winterfell::Serializable;
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
    
    let mut polynome_coefficients: Vec<u128> = Vec::with_capacity(number_of_sol_parts as usize);
    unsafe {polynome_coefficients.set_len(number_of_sol_parts as usize);}
    generate_random_numbers(&mut polynome_coefficients);
    println!("{:?},\n size:{}",polynome_coefficients,polynome_coefficients.len());

    println!("test winterfall: {:?},",test_winter_math());
    let mut test:Cipher = cypher_aes(&secret);
    println!("source text:{}, key:{}, cyphered_text:{}",secret,&test.keys.base64_key,&test.ciphered_text);

    let fernet = fernet::Fernet::new(&test.keys.base64_key).unwrap();
    let s = match String::from_utf8(fernet.decrypt(&test.ciphered_text).unwrap()){
        Ok(v) => v,
        Err(_) => panic!("Invalid UTF-8 sequence"),
    };
    println!("deciphered text:{}",s);

    
    
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
