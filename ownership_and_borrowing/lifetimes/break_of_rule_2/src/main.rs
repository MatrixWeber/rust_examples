fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);
}
// zwei Referenzen -> Kompilierfehler explizite Lifetime Annotation erforderlich
// Lifetime Annotationen erforderlich
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
// es bedeutet, dass für den Return Wert immer die kürzeste Lebenszeit verwendet wird.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}