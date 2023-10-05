//! This gets the token from a discord account with two factor auth

use joris::login::get_two_factor_authentication_ticket;
use joris::Joris;

fn main() {
    let mut email = String::new();
    let mut password = String::new();
    let mut code = String::new();
    let stdin = std::io::stdin();

    println!("Enter email:");
    stdin.read_line(&mut email).expect("Failed to read line");
    let email = email.trim();

    println!("");

    println!("Enter password:");
    stdin.read_line(&mut password).expect("Failed to read line");
    let password = password.trim();

    println!("");

    let ticket = get_two_factor_authentication_ticket(&email, &password).unwrap();

    println!("Enter code:");
    stdin.read_line(&mut code).expect("Failed to read line");
    let code = code.trim();

    println!("");

    println!(
        "Your token: {}",
        Joris::new_with_ticket(ticket, code.to_string())
            .unwrap()
            .token
    );
}
