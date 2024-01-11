struct UpCounter {
    name: String,
    value: u8,
    limit: u8,
}

struct DownCounter {
    name: String,
    value: u8,
    limit: u8,
}

trait Countable {
    // default implementation
    fn count(&mut self)
    {
        // some common stuff before
        self.count_with_limit();
        // some common stuff after
    }
    fn count_with_limit(&mut self);
}

impl Countable for UpCounter {
    fn count_with_limit(&mut self) {
        self.value += 1;
        if self.value >= self.limit {
            println!("{}: limit was reached {}",
                     self.name, self.limit);
        }
    }
}

impl Countable for DownCounter {
    fn count_with_limit(&mut self) {
        self.value -= 1;
        if self.value <= self.limit {
            println!("{}: limit was reached {}",
                     self.name, self.limit);
        }
    }
}

fn main() {
    // Erstellen von Up- und DownCounter Objekten
    let mut up_counter = UpCounter { name: "UpCounter".to_string(), value: 0 , limit: 3};
    let mut down_counter = DownCounter { name: "DownCounter".to_string(), value: 30, limit: 27};

    // Die Counter zählen lassen
    for i in 0..5 {
        up_counter.count();
        down_counter.count();
    }
    // Den aktuellen Counter Werte ausgeben
    println!("{}: hat folgenden Wert erreicht {}", up_counter.name, up_counter.value);
    println!("{}: hat folgenden Wert erreicht {}", down_counter.name, down_counter.value);
}