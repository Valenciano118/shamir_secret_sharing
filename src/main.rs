use std::io::{self, Write};
use shamir_secret_sharing::*;
fn main() {
    let mut text = String::from("Introduce your secret message:");
    let secret_message:String = text_input(&text);
    
    if secret_message.len()==0{
        panic!("The secret must have at least 1 character");
    }

    text = String::from("Introduce the number of parts to divide the secret:"); 
    let total_shares:u32 = match text_input(&text).parse(){
        Ok(value) => value,
        Err(_) => panic!("Number of parts introduced wasn't a number")
    };

    if total_shares <=1{
        panic!("Number of total parts has to be bigger than 1")
    }

    text = String::from("Introduce the number of parts needed to form the secret back:");
    let minimum_shares:u32 = match text_input(&text).parse(){
        Ok(value) => value,
        Err(_) => panic!("Number of parts introduced wasn't a number")
    };

    if minimum_shares <=1{
        panic!("Number of parts to form the secret has to be bigger than 1");
    }
    if minimum_shares > total_shares {
        panic!("Number of parts to form the secret back is larger than the total number of parts");
    }
    

    println!("\nThe message is: \"{}\"\nYou need {} out of {} parts to reveal the secret",secret_message,minimum_shares,total_shares);


    let testing = SecretSharingGenerator::new(&secret_message,total_shares,minimum_shares);

    println!("The cyphered message is:{:?}",testing.ciphered_message());
    println!("The hashed secret is: {:?}",testing.hashed_secret())
    
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
