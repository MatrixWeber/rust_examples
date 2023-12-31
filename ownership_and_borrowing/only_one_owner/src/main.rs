fn main() {
    let s1 = String::from("Hallo"); // s1 ist "Owner" des Strings "Hallo"
    let s2 = s1; // s1 wird "besitzlos", s2 wird neuer "Owner"
    // println!("{}", s1); // Fehler: s1 wurde bereits verschoben
    println!("{}", s2); // Funktioniert, da s2 jetzt der Owner ist
}
