struct Counter {
    name: String,
    value: u8,
    limit: u8,
}

fn main() {
    // Erstellen einer "mutablen Instanz" der Counter-Struktur namens "Timer"
    let mut timer = create_counter(String::from("Timer"), 0, 30);

    // Den Counter Wert inkrementieren
    increment_counter(&mut timer);

    // Den aktuellen Zählerwert ausgeben
    println!("{}: mit einem Limit von {} hat folgenden Wert erreicht {}",
             timer.name, timer.limit, timer.value);
}

fn create_counter(name: String, value: u8, limit: u8) -> Counter {
    Counter { name, value, limit } // kurzschreibweise möglich
}

fn increment_counter(counter: &mut Counter) {
    counter.value += 1;
    counter.value += 1;
    counter.value += 1;
}

