use std::io;
fn main() {
    println!("Introduce el secreto: ");
    let mut secret = String::new();

    io::stdin()
        .read_line(&mut secret)
        .expect("Fallo al leer la linea");
    
    println!("El secreto introducido es:{}",secret);
}
