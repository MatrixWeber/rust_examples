fn main() {
    let s1 = String::from("Hallo"); // s1 ist "Owner" des Strings "Hallo"
    takes_ownership(s1); // s1 wird "besitzlos", das "Ownership" wird weitergegeben
    println!("{}", s1); // Fehler: s1 wurde bereits verschoben
}

fn takes_ownership(s: String) { // ownership wird an die Variable "s" übergeben
    println!("{s}"); // ownership wird an die Variable "s" übergeben
} // Variable "s", ursprünglich s1, wird aufgeräumt