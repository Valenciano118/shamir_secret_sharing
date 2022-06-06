use std::io::{self, Write};
use shamir_secret_sharing::*;
use std::fs;
use serde::{Serialize, Deserialize};
use chrono;

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


    let testing = SecretSharing::new(&secret_message,total_shares,minimum_shares);

    println!("The struct is: {:?}",testing);

    let mut polynomial = testing.polynomial();

    let mut shares:Vec<Point> = Vec::new();

    for _ in 0..minimum_shares{
        match polynomial.pop(){
            Some(point) => shares.push(point),
            None => ()
        }
    }

    let result = SecretSharing::solve(testing.ciphered_message(), &testing.initialization_vector(), shares);
    
    println!("And the message was:{}",result);

    
    //The directory will be named as the current date
    let path = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    match fs::create_dir_all(&path){
        _ => ()
    }
    let serialized = generate_json(testing);
    let size = serialized.len();
    for (i,data) in serialized.iter().enumerate(){
        let filepath = format!("{}/{}.json",&path,size-i);
        match fs::write(&filepath, data){
            _ => ()
        }
    }





    
    
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

fn generate_json(secret_sharing : SecretSharing) -> Vec<String>{

    let mut result:Vec<String> = Vec::new();
    let mut polynomial = secret_sharing.polynomial();
    let min_shares = secret_sharing.minimum_shares();
    let total_shares = secret_sharing.total_shares();
    let ciphered_message = secret_sharing.ciphered_message();

    for _ in 0..total_shares{
        match polynomial.pop(){
            Some(point) => {
                let serialized_data = SerializedData{
                    point : point,
                    minimum_shares : min_shares,
                    total_shares : total_shares,
                    ciphered_message : ciphered_message.clone(), 
                };
                result.push(serde_json::to_string(&serialized_data).unwrap());
            },
            None => break
        }   
    }
    result

}

#[derive(Serialize, Deserialize, Debug)]
struct SerializedData{
    point : Point,
    minimum_shares : u32,
    total_shares : u32,
    ciphered_message : Vec<u8>,
}
