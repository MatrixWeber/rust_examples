fn main() {
    // Eine Referenz auf eine Zeichenkette, die im read only memory alloziert wird!
    let hello_world: &'static str = "Hello, World!";
    // Die Typannotationen werden nicht zwingend benötigt
    // &'static str können auch weggelassen werden
    println!("{}", hello_world);
}
