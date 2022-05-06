use std::io;
fn main() {
    println!("Input you secret: ");
    let mut secret = String::new();

    io::stdin()
        .read_line(&mut secret)
        .expect("Failed to read the line");
    
    println!("The secret introduced is:{}",secret);
}
