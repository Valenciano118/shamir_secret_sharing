use rand::{Rng,thread_rng};

pub fn generate_random_numbers(vector: &mut Vec<u128>) {
    for i in 0..vector.len(){
        vector[i] = thread_rng().gen();
    }
}

