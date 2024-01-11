struct UpCounter {
    name: String,
    value: u8,
}

struct DownCounter {
    name: String,
    value: u8,
}

trait Countable {
    fn count(&mut self);
    fn get_value(&self) -> u8;
}

impl Countable for UpCounter {
    fn count(&mut self) {
        self.value += 1;
    }
    fn get_value(&self) -> u8
    {
        self.value
    }
}

impl Countable for DownCounter {
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
    // Den aktuellen Counter Werte ausgeben
    println!("up: hat folgenden Wert erreicht {}", up_counter.get_value());
    println!("down: hat folgenden Wert erreicht {}", down_counter.get_value());
}

fn create_up_counter(name: String, value: u8) -> impl Countable {
    UpCounter { name, value }
}

fn create_down_counter(name: String, value: u8) -> impl Countable {
    DownCounter { name, value }
}


fn count_on_trait(some_counter: &mut impl Countable) {
    some_counter.count();
}