fn main() {
    let x: Result<u8, &str> = Ok(5);
    // unwrap wird verwendet,
    // um den Wert aus einem Ok Result zu bekommen.
    // Wenn Result ein Err ist, wird das Programm mit panic abstürzen.
    let y = x.unwrap();
    println!("y: {}", y); // y: 5 wird ausgegeben

    let x: Result<u8, &str> = Err("There was an error");
    // expect wird verwendet,
    // um den Wert aus einem Ok Result zu bekommen.
    // Wenn Result ein Err ist, wirft das Programm panic
    // und zeigt die bereitgestellte Nachricht an.
    let y = x.expect("Failed to get the value");
    println!("y: {}", y); // panic mit "Failed to get the value"
}