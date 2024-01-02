fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Wert wird per Referenz übergeben
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}