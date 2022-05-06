use rand::{Rng,thread_rng};

//TODO:
//Remove the pub keyword since this function should be private, it's public right now for testing purposes only 
pub fn generate_random_numbers(vector: &mut Vec<u128>) { 
    for i in 0..vector.len(){
        vector[i] = thread_rng().gen();
    }
}

