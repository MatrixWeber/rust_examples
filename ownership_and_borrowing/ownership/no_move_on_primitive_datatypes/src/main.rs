fn main() {
    let x = 5; // x ist vom Typ i32, der einen "Copy-Trait" hat
    let y = x; // eine Kopie von "x" wird erstellt und "y" zugewiesen

    println!("{x}"); // Die "5" wird ausgegeben
}
