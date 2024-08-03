use std::env;
use std::process::Command;
use std::process::Output;
use std::io::{self, Write};

fn main() {
    match env::var("PG_PASSWORD") {
        Ok(value) => println!("El valor de la variable es: {}", value),
        Err(e) => println!("No se pudo leer la variable: {}", e), 
    }
    
    make_backup()
    
}

fn make_backup(){
    
    let output: Output = Command::new("pg_dump")
        .arg("-U")
        .arg("postgres")
        .arg("-h")
        .arg("localhost")
        .arg("-p")
        .arg("5432")
        .arg("db_name")
        .output()
        .expect("Fail on execute pg_dump");

    if output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        eprintln!("Error backup database pg_dump: {}", String::from_utf8_lossy(&output.stderr));
    }

}
fn upload_file() {
    
}