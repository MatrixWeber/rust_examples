fn main() {
    let x = 5;
    let y = &x; // Referenz nur lesend verwendbar
    // *y += 1; // Kompilierfehler
    println!("The value of y and x is: {}", *y);
    // es muss Dereferenziert werden mit "*",
    // um den Wert von "x" auf den "y" zeigt zu verwenden!
}