use joris::login::get_two_factor_authentication_ticket;

fn main() {
    let mut email = String::new();
    let mut password = String::new();
    let stdin = std::io::stdin();

    println!("Enter email:");
    stdin.read_line(&mut email).expect("Failed to read line");

    println!("Enter password:");
    stdin.read_line(&mut password).expect("Failed to read line");

    println!(
        "{}",
        &get_two_factor_authentication_ticket(&email, &password).unwrap()
    );
}
