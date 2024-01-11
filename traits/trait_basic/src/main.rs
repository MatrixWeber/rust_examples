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
}

impl Countable for UpCounter {
    fn count(&mut self) {
        self.value += 1;
    }
}

impl Countable for DownCounter {
    fn count(&mut self) {
        self.value -= 1;
    }
}

fn main() {
    // Erstellen von Up- und DownCounter Objekten
    let mut up_counter = UpCounter { name: "UpCounter".to_string(), value: 0 };
    let mut down_counter = DownCounter { name: "DownCounter".to_string(), value: 30 };

    // Die Counter zählen lassen
    for i in 0..3 {
        up_counter.count();
        down_counter.count();
    }
    // Den aktuellen Counter Werte ausgeben
    println!("{}: hat folgenden Wert erreicht {}", up_counter.name, up_counter.value);
    println!("{}: hat folgenden Wert erreicht {}", down_counter.name, down_counter.value);
}