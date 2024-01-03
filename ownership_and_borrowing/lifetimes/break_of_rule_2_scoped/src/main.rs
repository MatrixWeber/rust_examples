fn main() {
    let s1 = String::from("long string is long");       // ---------+-- 'a
    let result;                                             //          |
    { // neuer Scope -> neue Lebenszeit 'b für "s2"              // -+-- 'b  |
        let s2 = String::from("xyz");                  //  |       |
        result = longest(&s1, &s2);                              //  |       |
    } // "s2" und "result" werden gedropped                      // -+       |
    println!("The longest string is {}", result);                //          |
}                                                                // ---------+

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}