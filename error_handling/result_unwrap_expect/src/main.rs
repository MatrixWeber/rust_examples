struct Counter {
    name: String,
    value: u8,
    limit: u8,
}

impl Counter {
    fn new(name: String, value: u8, limit: u8) -> Counter {
        Counter { name, value, limit }
    }
    fn count(&mut self) { self.value += 1; }
    fn get_value(&self) -> u8 { self.value }
    fn set_value(&mut self, value: u8) -> Result<(), &str> {
        if value >= self.limit {
            return Err("input parameter overflow");
        }
        self.value = value;
        return Ok({});
    }
}

fn main() {
    // Erstellen einer "mutablen Instanz" namens "Timer" mit dem "Limit: 30"
    let mut timer = Counter::new(String::from("Timer"), 0, 30);
    let result = timer.set_value(20).unwrap(); // Wert, der größer als der Limit ist!
    let result = timer.set_value(200); // Wert, der größer als der Limit ist!
    match result {
        Err(s) => println!("Error: {}", s),    // Ausgabe: Error: input parameter overflow
        Ok(_) =>
        // Der Zählerwert wird ausgeben, da das System nicht mehr abstürzt
            println!("{}: hat folgenden Wert erreicht {}", timer.name, timer.get_value())
    }
}