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
    fn set_value(&mut self, value: u8) {
        if value >= self.limit {
            panic!("input parameter overflow");
        }
        self.value = value;
    }
}

fn main() {
    // Erstellen einer "mutablen Instanz" der Counter-Struktur namens "Timer"
    let mut timer = Counter::new(String::from("Timer"), 0, 30);
    timer.set_value(200); // Einen Wert einspeisen, der größer als der Limit ist! -> panic!
    // Der Zählerwert wird nicht ausgeben, da das System mit panic! abstürzt
    println!("{}: hat folgenden Wert erreicht {}", timer.name, timer.get_value());
}