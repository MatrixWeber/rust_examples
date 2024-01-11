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
    let mut up_counter = UpCounter { name: "UpCounter".to_string(), value: 0 };
    let mut down_counter = DownCounter { name: "DownCounter".to_string(), value: 30 };
    count_on_trait(&mut up_counter);                 // Up counter 1
    count_on_trait_bound_syntax(&mut down_counter);  // Down counter 29
    println!("{}: hat folgenden Wert erreicht {}", up_counter.name, up_counter.value);
    println!("{}: hat folgenden Wert erreicht {}", down_counter.name, down_counter.value);
}

fn count_on_trait(some_counter: &mut impl Countable) {
    some_counter.count();
}
fn count_on_trait_bound_syntax<T: Countable>(some_counter: &mut T) {
    some_counter.count();
}

