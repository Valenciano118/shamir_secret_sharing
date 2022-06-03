use rand::{Rng,thread_rng};

use aes::cipher::{
    KeyIvInit, StreamCipher,
    generic_array::GenericArray
};
use aes::cipher::StreamCipherSeek;
use sha2::{Sha256,Digest};
use sha2::digest::generic_array::typenum::U32;
use sha2::digest::generic_array::typenum::U16;

type Aes256Ctr128BE = ctr::Ctr128BE<aes::Aes256>;

const PRIME:f64 = (2u64.pow(61)-1) as f64;


pub fn calculate_hash<T: AsRef<[u8]>>(t: &T) -> GenericArray<u8,U32> {
    let mut hasher = Sha256::new();
    hasher.update(t);
    hasher.finalize()
}



pub fn cipher_message(key: &GenericArray<u8,U32>, message: &str, initialization_vector: &GenericArray<u8,U16>) -> Vec<u8> {
    let mut buf = message.as_bytes().to_vec();
    let mut cipher= Aes256Ctr128BE::new(&key,&initialization_vector);

    cipher.apply_keystream_b2b(message.as_bytes(),&mut buf).unwrap();
    buf
}

fn decipher_message(key: &GenericArray<u8,U32>, initialization_vector: &GenericArray<u8,U16>, ciphered_message: Vec<u8>) -> String{
    let mut cipher = Aes256Ctr128BE::new(key, initialization_vector);

    let mut buf = vec![0u8;ciphered_message.len()];

    cipher.apply_keystream_b2b(&ciphered_message, &mut buf).unwrap();

    String::from_utf8(buf).unwrap()
}

fn generate_random_initialization_vector() -> GenericArray<u8,U16>{
    let temp_iv_array: [u8;16] = rand::random();
    let mut iv:GenericArray<u8,U16> = GenericArray::default();
    iv.copy_from_slice(&temp_iv_array);
    
    iv
}


pub struct SecretSharing {
    ciphered_message: Vec<u8>,
    hashed_secret: GenericArray<u8,U32>,
    initialization_vector: GenericArray<u8,U16>,
    total_shares: u32,
    minimum_shares: u32,
    polynomial: Vec<Point>,
}

impl SecretSharing {
    pub fn new (message: &str, total_shares:u32, minimum_shares:u32) -> Self {
        let mut rng = thread_rng();
        let secret:f64 = rng.gen();
        
        let hashed_secret = calculate_hash(&secret.to_string());

        let iv = generate_random_initialization_vector();

        let ciphered_message = cipher_message(&hashed_secret, &message,&iv);

        let polynomial = secret_sharing(secret, total_shares, minimum_shares);

        Self { 
            ciphered_message: ciphered_message ,
            hashed_secret: hashed_secret,
            initialization_vector: iv,
            total_shares: total_shares,
            minimum_shares: minimum_shares,
            polynomial: polynomial }


    }

    pub fn ciphered_message(self : &Self) -> Vec<u8> {
        self.ciphered_message.clone()
    }

    pub fn hashed_secret(self : &Self ) -> GenericArray<u8,U32> {
        self.hashed_secret
    }

    pub fn initialization_vector( self : &Self) -> GenericArray<u8,U16>{
        self.initialization_vector
    }

    pub fn total_shares(self : &Self) -> u32 {
        self.total_shares
    }
    
    pub fn minimum_shares(self : &Self) ->u32 {
        self.minimum_shares
    }

    pub fn polynomical(self : &Self) -> Vec<Point> {
        self.polynomial.clone()
    }

    pub fn solve(ciphered_message: Vec<u8>, initialization_vector: &GenericArray<u8,U16>, shares: Vec<Point> ) -> String {
        let secret = interpolate(shares);

        let key = calculate_hash(&secret.to_string());

        decipher_message(&key, &initialization_vector, ciphered_message)
    }
}






#[derive(Debug, Clone, Copy)]
pub struct Point{
    x:f64,
    y:f64
}
impl Point {
    pub fn new(x:f64, y:f64) -> Self {
        Point{
            x:x,
            y:y
        }
    }
}

fn calculate_Y(x:u32, poly:&Vec<f64> ) -> f64 {
    
    let mut y:f64 = 0.0;
    let mut temp:f64 = 1.0; 

    for coefficient in poly{
        y = y+(coefficient * temp);
        temp = temp * x as f64;
    }
    y
    
}

fn secret_sharing(secret:f64, total_shares:u32, minimum_shares:u32) -> Vec<Point>{
    //This will hold the coefficients of the polynome
    let mut polynome:Vec<f64> = Vec::new();

    //The first element of the polynome is the secret
    polynome.push(secret);

    //We initialise the random number generator
    let mut rng = thread_rng();

    for _ in 1..minimum_shares{
        let mut p:f64 = 0.0;

        //This while loop ensures that we are not adding a value of 0 into the polynome
        while p == 0.0{
            p = rng.gen::<f64>() % PRIME;
        }
        polynome.push(p);
    }

    let mut result:Vec<Point> = Vec::new();


    //We calculate the f(x) for every x, which is the total number of shares
    //
    //The points from this resulting vector are the ones that we share to form the secret back
    for x in 1..total_shares{
        let y:f64 = calculate_Y(x,&polynome);
        
        result.push(Point::new(x as f64,y));
    }

    result


}

fn generate_random_points(n_points:u32) ->Vec<Point>{ 
    let mut rng = thread_rng();
    let mut result:Vec<Point> = Vec::new();
    for i in 1..n_points+1{
        result.push(
            Point::new(i as f64,rng.gen())
        );
    }
    result
}   


pub fn generate_polynomial(secret:f64, n_points:u32) -> Vec<Point>{
    let mut result:Vec<Point> = Vec::new();
    result.push(Point::new(0.0,secret));
    result.append(&mut generate_random_points(n_points));

    result
    
}



pub fn interpolate (polynome:Vec<Point>) -> f64{

    let n_elements = polynome.len();
    let mut result = 0 as f64;

    for i in 0..n_elements{
        let mut product = polynome[i].y;
        for j in 0..n_elements{
            if i!=j{
                let denominator = polynome[i].x -polynome[j].x;
                let numerator = -polynome[j].x;
                product = product * (numerator/denominator);
            }
        }
        result += product;
    }
    result.round()
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn random_points_generation(){
        assert_eq!(3,generate_random_points(3).len());
    }
    #[test]
    fn create_point(){
        let point = Point::new(1.0,2.0);
        assert_eq!(1.0,point.x);
        assert_eq!(2.0,point.y);
    }
    #[test]
    fn check_interpolate(){
        let polynome = secret_sharing(1234 as f64,10,5);
        let vec:Vec<Point> = Vec::from_iter(polynome[4..=8].iter().cloned());
        assert_eq!(interpolate(vec),1234 as f64);
    }

    #[test]
    fn check_interpolate_with_less_than_minimum_shares(){
        let polynome = secret_sharing(1234 as f64, 10 ,5);
        let vec:Vec<Point> = Vec::from_iter(polynome[4..=7].iter().cloned()); // 4 shares 4,5,6,7
        assert_ne!(interpolate(vec),1234 as f64);
    }

    #[test]
    fn check_polynome_length(){
        let polynome = secret_sharing(1234 as f64,10,5);
        assert_eq!(polynome[4..=8].len(),5); //from 4 to 8
    }

    #[test]
    fn check_hash_is_the_same_for_a_given_value(){
        let mut hasher1 = Sha256::new();
        let mut hasher2 = Sha256::new();

        hasher1.update(b"hola");
        hasher2.update(b"hola");

        let result1 = hasher1.finalize();
        let result2 = hasher2.finalize();

        assert_eq!(result1,result2);
    }

    #[test]
    fn check_secret_sharing_solver(){
        let message = "probando";
        let total_shares = 10;
        let minimum_shares = 5;
        let instance = SecretSharing::new(message, total_shares, minimum_shares);

        let solved_message = SecretSharing::solve(instance.ciphered_message(), &instance.initialization_vector(), instance.polynomical());

        assert_eq!(message,solved_message);
    }
    #[test]
    fn check_ciphering_and_deciphering_with_aes256(){
        let plaintext = "probando";
        let mut buf1 = vec![0u8;plaintext.len()];
        let mut buf2 = vec![0u8;plaintext.len()];

        let key = [0x42; 32];
        let iv = [0x24; 16];

        let mut cipher = Aes256Ctr128BE::new(&key.into(), &iv.into());

        
        cipher
            .apply_keystream_b2b(&plaintext.as_bytes(), &mut buf1)
            .unwrap();
        
        cipher.seek(0u32);
        cipher
            .apply_keystream_b2b(&buf1, &mut buf2)
            .unwrap();

        let string = String::from_utf8(buf2.to_vec()).unwrap();

        assert_eq!(plaintext,string);

    }

    #[test]
    fn check_ciphering_and_deciphering_with_my_functions() {
        let plaintext = "probando";
        let key = 1234 as f64;
        let hashed_key = calculate_hash(&key.to_string());
        let iv = generate_random_initialization_vector();

        let ciphered_message = cipher_message(&hashed_key, plaintext, &iv);

        let polynomial = secret_sharing(key, 10, 5);

        let interpolated_hashed_key =calculate_hash(&interpolate(polynomial).to_string());
        
        let string = decipher_message(&interpolated_hashed_key,&iv, ciphered_message);

        assert_eq!(plaintext,string);

        
    }


}
