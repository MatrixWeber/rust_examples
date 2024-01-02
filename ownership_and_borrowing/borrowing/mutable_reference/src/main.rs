fn main() {
    let mut x = 5; // um eine mutable Referenz auf "x" zu erzeugen,
    // muss die Variable "x" als mutable, mit dem Schlüsselwort "mut" deklariert werden.
    let y = &mut x; // nun kann eine mutable Referenz auf "x" angelegt werden.
    // let z = &mut x; // Kompilierfehler
    *y += 1; // "y" wird inkrementiert und somit auch "x"
    println!("The value auf x is: {x}"); // Ausgabe 6
}
