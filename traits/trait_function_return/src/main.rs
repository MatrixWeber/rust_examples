struct UpCounter {
    name: String,
    value: u8,
}

struct DownCounter {
    name: String,
    value: u8,
}

trait Count {
    fn count(&mut self);
    fn get_value(&self) -> u8;
}

impl Count for UpCounter {
    fn count(&mut self) {
        self.value += 1;
    }
    fn get_value(&self) -> u8
    {
        self.value
    }
}

impl Count for DownCounter {
    fn count(&mut self) {
        self.value -= 1;
    }
    fn get_value(&self) -> u8
    {
        self.value
    }
}

fn main() {
    // Erstellen von Up- und DownCounter Objekten
    let mut up_counter = create_up_counter("UpCounter".to_string(), 0);
    let mut down_counter = create_down_counter("DownCounter".to_string(), 30);
    count_on_trait(&mut up_counter);    // Up counter 1
    count_on_trait(&mut down_counter);  // Down counter 29
    // Die aktuellen Counter Werte ausgeben
    println!("up: hat folgenden Wert erreicht {}", up_counter.get_value());
    println!("down: hat folgenden Wert erreicht {}", down_counter.get_value());
}

fn create_up_counter(name: String, value: u8) -> impl Count {
    UpCounter { name, value }
}

fn create_down_counter(name: String, value: u8) -> impl Count {
    DownCounter { name, value }
}

fn count_on_trait(some_counter: &mut impl Count) {
    some_counter.count();
}