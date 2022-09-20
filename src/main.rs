use std::io::{self, Write};
use aes::cipher::consts::U16;
use shamir_secret_sharing::*;
use std::fs;
use serde::{Serialize, Deserialize};

use std::env;
use std::path::Path;
use aes::cipher::generic_array::GenericArray;


fn main() -> io::Result<()> {
    let args:Vec<String> = env::args().collect();

    if args.len() != 2{
        panic!("Error!  Usage: shamir_secret_sharing <file>/<directory>")
    }

    let path = Path::new(&args[1]);

    if path.is_dir(){
        let entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        let mut read_files: Vec<SerializedData> = Vec::new();

        for entry in entries{
            let file_content = String::from_utf8_lossy(&fs::read(entry)?).into_owned();
            
            let deserialized: SerializedData = serde_json::from_str(&file_content).unwrap();
            read_files.push(deserialized);
        }

        let min_shares = read_files[0].minimum_shares;
        if read_files.len() < min_shares as usize{
            panic!("There are not enough files to decipher the message, minimum needed:{}, and {} were provided",min_shares, read_files.len()); 
        }

        let ciphered_message = read_files[0].ciphered_message.clone();
        let mut initialization_vector:GenericArray<u8,U16> = GenericArray::default();
        initialization_vector.copy_from_slice(&read_files[0].initialization_vector[..]);

        let mut shares: Vec<Point> = Vec::new();
        for data in read_files{
            shares.push(data.point);
        }



        let result = SecretSharing::solve(ciphered_message, &initialization_vector, shares);
    
        let result_path = "result.txt";

        fs::write(result_path, result)?;
    }

    else if path.is_file(){
        let file_content = String::from_utf8_lossy(&fs::read(path)?).into_owned();

        let mut text:String;

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


        let secret = SecretSharing::new(&file_content,total_shares,minimum_shares);

        let serialized_data = generate_json(secret);

        let save_dir = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        fs::create_dir_all(&save_dir)?;

        let size = serialized_data.len();

        for (i,data) in serialized_data.iter().enumerate(){
            let filepath = format!("{}/{}.json",&save_dir,size-i);
            fs::write(&filepath, data)?;
        }

    }
    else{
        panic!("Error! argument provided is not a directory nor a file");
    }

    Ok(())
}


fn text_input(prompt_text: &String) -> String{
    print!("{} ",prompt_text);
    io::stdout().flush().unwrap();
    let mut in_text = String::new();
    io::stdin()
        .read_line(&mut in_text)
        .expect("Failed to read the line");
    
    if in_text.ends_with('\n'){
        in_text.pop().unwrap();
        if in_text.ends_with('\r'){
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
                    point,
                    minimum_shares : min_shares,
                    total_shares,
                    ciphered_message : ciphered_message.clone(),
                    initialization_vector : secret_sharing.initialization_vector().to_vec() 
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
    initialization_vector : Vec<u8>
}
