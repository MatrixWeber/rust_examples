fn main() {
    let s1 = gives_ownership();         // verschiebt seinen Rückgabewert in s1
    let s2 = String::from("hello");  // s2 wird angelegt
    let s3 = takes_and_gives_back(s2);  // s2 wird nach takes_and_gives_back verschoben
    // welches seinen Rückgabewert ebenfalls in s3 verschiebt
} // Hier, s3 geht aus dem Scope und wird gelöscht. s2 wurde verschoben, also passiert nichts.
  // s1 geht ebenfalls aus dem Scope und wird gelöscht.

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string wird angelegt
    some_string // some_string wird zurückgegeben und somit die Variable an die aufrufende Funktion verschoben
}

fn takes_and_gives_back(a_string: String) -> String { // a_string kommt in den Scope
    a_string  // a_string wird zurückgegeben und somit die Variable an die aufrufende Funktion verschoben
}