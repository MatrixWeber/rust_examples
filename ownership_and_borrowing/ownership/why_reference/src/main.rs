fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // weder schön noch intuitiv, Abhilfe Referenzen
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // ein Tuple aus String und der Länge wird zurückgegeben
}