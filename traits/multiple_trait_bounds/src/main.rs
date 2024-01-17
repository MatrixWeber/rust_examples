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
}

trait Reset {
    fn reset(&mut self);
}

impl Count for UpCounter {
    fn count(&mut self) {
        self.value += 1;
    }
}

impl Reset for UpCounter {
    fn reset(&mut self) {
        self.value = 0;
    }
}

impl Count for DownCounter {
    fn count(&mut self) {
        self.value -= 1;
    }
}

fn main() {
    let mut up_counter = UpCounter { name: "UpCounter".to_string(), value: 50 };
    let mut down_counter = DownCounter { name: "DownCounter".to_string(), value: 30 };
    count_on_trait(&mut up_counter);
    //   count_on_trait_bound_syntax(&mut down_counter);  // Kompilierfehler
    count_on_trait_where_clause(&mut up_counter, &mut down_counter);
    println!("{}: hat folgenden Wert erreicht {}", up_counter.name, up_counter.value);
    println!("{}: hat folgenden Wert erreicht {}", down_counter.name, down_counter.value);
}

fn count_on_trait(some_counter: &mut (impl Count + Reset)) {
    some_counter.reset();
    some_counter.count();
}
fn count_on_trait_bound_syntax<T: Count + Reset>(some_counter: &mut T) {
    some_counter.reset();
    some_counter.count();
}
fn count_on_trait_where_clause<T, U>(some_counter: &mut T, other_counter: &mut U)
where
    T: Count + Reset,
    U: Count,
{
    some_counter.reset();
    some_counter.count();

    other_counter.count();
}

