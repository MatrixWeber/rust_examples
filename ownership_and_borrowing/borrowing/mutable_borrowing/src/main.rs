fn main() {
    let mut x = 5; // ein mutable Variable anlegen
    increment(&mut x); // wichtig: beim übergeben in die Funktion "mut" darf nicht fehlen
    println!("Wert von x nach der Änderung: {}", x); // 6 wird Ausgegeben
}

fn increment(y: &mut i32){ // wichtig: Funktions Parameter müssen auch "mut" sein
    *y += 1; // erst dann funktioniert die Berechnung!
}
