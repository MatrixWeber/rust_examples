enum State {
    // Varianten des Enums
    Idle(Option<u64>),        // Variante mit "Optionalen Daten" vom Typ "u64"
    Running(String),
    Paused(usize),
    Finished(Option<String>), // Variante mit "Optionalen Daten" vom Typ "String"
}

impl State {
    fn call(&self) {
        match self {
            State::Idle(counter) => println!("Das System ist {} mal im Leerlauf gewesen.", counter.unwrap()),
            _ => println!("Für alle anderen Varianten tuhe nichts")
        }
    }
}

fn main() {
    // Bei der Instanziierung des Enums werden Daten mitgegeben
    let mut current_state = State::Finished(None);
    current_state.call();
    current_state = State::Idle(Some(50));
    current_state.call();
}