fn main() {
    let x = 1;
    let y = 2;
    let sum = add(x, y); // zwei integer Werte addieren
    println!("The sum is {sum}"); // Ausgabe "The sum is 3"

    let x = 1.1;
    let y = 2.9;
    let sum = add(x, y); // zwei float Werte addieren
    println!("The sum is {sum}"); // Ausgabe "The sum is 4"
}

fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
