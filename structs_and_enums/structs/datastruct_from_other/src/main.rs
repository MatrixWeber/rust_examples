struct Counter {
    name: String,
    value: u8,
    limit: u8,
}

fn main() {
    let hours = create_counter(String::from("Hours"), 0, 24);
    let mut timer = from_counter(String::from("Timer"), hours);
    increment_counter(&mut timer);
    println!("{}: mit einem Limit von {} hat folgenden Wert erreicht {}",
             timer.name, timer.limit, timer.value);
    // println!("{}: mit einem Limit von {} hat folgenden Wert erreicht {}",
    //         hours.name, hours.limit, hours.value); Kompilierfehler
}

fn from_counter(name: String, counter: Counter) -> Counter {
    Counter {
        name,
        ..counter // update Syntax
    }
}

fn create_counter(name: String, value: u8, limit: u8) -> Counter {
    Counter { name, value, limit } // kurzschreibweise möglich
}

fn increment_counter(counter: &mut Counter) {
    counter.value += 1;
    counter.value += 1;
    counter.value += 1;
}

