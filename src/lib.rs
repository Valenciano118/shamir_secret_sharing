use rand::{Rng,thread_rng};
use winter_math::{polynom, StarkField};
use winterfell::math::fields::f128::BaseElement;
use fernet::Fernet;
use base64;

pub struct Cipher{
    pub keys : KeyData,
    pub ciphered_text : String,
}

pub struct KeyData{
    pub key : String,
    pub hex_key : [u8;16],
    pub int_key : u128,
    pub base64_key : String,
}

//TODO:
//Remove the pub keyword since this function should be private, it's public right now for testing purposes only 
pub fn generate_random_numbers(vector: &mut Vec<u128>) { 
    for i in 0..vector.len(){
        vector[i] = thread_rng().gen();
    }
}

pub fn test_winter_math() -> u128{

    let xs = vec![BaseElement::new(1),BaseElement::new(2),BaseElement::new(3)];
    let ys = vec![BaseElement::new(1234),BaseElement::new(166),BaseElement::new(94)];
    let poly:Vec<BaseElement> = polynom::interpolate(&xs, &ys, true);
    match polynom::eval_many(&poly, &xs).get(0){
        Some(&value) => value.as_int(),
        _ => 0 as u128, 
    }
} 

pub fn cypher_aes(string: &String) ->Cipher{
    let keys: KeyData = generate_hex_key();
    //let key = Fernet::generate_key();
    println!("value of key_hex:{:?} value of key_base64:{}, value of int_key:{}",keys.hex_key,keys.base64_key,keys.int_key);
    let fernet = Fernet::new(&keys.base64_key).unwrap();
    let ciphertext = fernet.encrypt(string.as_bytes());
    Cipher { keys: keys, ciphered_text: ciphertext }
}

fn duplicate_string(string: &String) -> String{
    let mut result = String::new();
    result.push_str(string);
    result.push_str(string);
    result
}
fn generate_hex_key() -> KeyData{
    let mut rng = rand::thread_rng();
    let mut hex_key:[u8;16] = [0;16];
    let mut key:String = String::from("");
    for i in 0..hex_key.len(){
        let tmp = rng.gen_range(0..15);
        hex_key[i] =  tmp as u8;
        key.push(char::from_digit(tmp , 16).unwrap());
    } 
    let int_key = u128::from_ne_bytes(hex_key);
    let key_base64 = base64::encode(duplicate_string(&key));
    KeyData { key:key, hex_key: hex_key, int_key: int_key, base64_key: key_base64 }
}


