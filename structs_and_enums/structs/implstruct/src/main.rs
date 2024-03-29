struct Counter {
    name: String,
    value: u8,
    limit: u8,
}

impl Counter {
    // Assoziierte Funktion "new" eine Art Konstruktor
    fn new(name: String, value: u8, limit: u8) -> Counter {
        Counter { name, value, limit }
    }
    // mutable Referenz auf das Objekt
    fn count(&mut self) {
        self.value += 1;
    }
    // immutable Referenz auf das Objekt
    fn get_value(&self) -> u8 {
        self.value
    }
}

fn main() {
    // Erstellen einer "mutablen Instanz" der Counter-Struktur namens "Timer"
    let mut timer = Counter::new(String::from("Timer"), 0, 30);

    // Den Counter Wert inkrementieren
    timer.count();
    timer.count();
    timer.count();

    // Den aktuellen Zählerwert ausgeben
    println!("{}: hat folgenden Wert erreicht {}", timer.name, timer.get_value());
}

#[cfg(test)] // um nur fürs Testen kompiliert zu werden
mod tests {  // beachten Sie, dass die Tests im tests-Untermodul sind
use super::*;
    #[test]  // die #[test]-Annotation darf nicht fehlen.
    fn count_test() {
        let mut timer = Counter::new(String::from("Timer"), 10, 30);
        assert_eq!(10, timer.get_value());
        timer.count();
        assert_eq!(11, timer.get_value());
    }
}