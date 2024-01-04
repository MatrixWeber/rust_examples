struct Counter {
    name: String,
    value: u8,
    limit: u8,
}

fn main() {
    // Erstellen einer "mutablen Instanz" der Counter-Struktur namens "Timer"
    let mut timer = Counter { name: String::from("Timer"), value: 0, limit: 30 };

    // Den Counter Wert inkrementieren
    timer.value += 1;
    timer.value += 1;
    timer.value += 1;

    // Den aktuellen Zählerwert ausgeben
    println!("{}: mit einem Limit von {} hat folgenden Wert erreicht {}",
             timer.name, timer.limit, timer.value);
}