use rand::{Rng,thread_rng};
use bacon_sci::{polynomial,interp};

//TODO:
//Remove the pub keyword since this function should be private, it's public right now for testing purposes only 
pub fn generate_random_numbers(vector: &mut Vec<u128>) { 
    for i in 0..vector.len(){
        vector[i] = thread_rng().gen();
    }
}

pub fn test_bacon_sci() -> f64{

    let xs:Vec<f64> = vec![0 as f64,1 as f64,2 as f64];
    let ys:Vec<f64> = vec![1234 as f64 ,166 as f64 ,94 as f64];


    let poly = interp::lagrange(&xs, &ys,1e-6).unwrap();

    poly.evaluate(xs[0])

}

