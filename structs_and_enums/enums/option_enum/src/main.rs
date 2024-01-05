enum State {
    // Varianten des Enums
    Idle(Option<u64>),        // Variante mit "Optionalen Daten" vom Typ "u64"
    Running(String),
    Paused(usize),
    Finished(Option<String>), // Variante mit "Optionalen Daten" vom Typ "String"
}

fn main() {
    // Bei der Instanziierung des Enums werden Daten mitgegeben
    let mut current_state = State::Finished(None);
    pattern_matching(current_state);
    current_state = State::Idle(Some(50));
    pattern_matching(current_state);
}

fn pattern_matching(current_state: State) {
    // Pattern Matching, um abhängig vom Zustand unterschiedliche Aktionen auszuführen
    match current_state {
        State::Idle(counter) => println!("Das System ist {} mal im Leerlauf gewesen.", counter.unwrap()),
        State::Running(s) => println!("Das System wurde vom {} gestartet!", s),
        State::Paused(seconds) => println!("Das System ist pausiert für {}s.", seconds),
        State::Finished(error) => println!("Das System wurde mit dem Fehler '{}' beendet.",
                                           error.unwrap_or(String::from("kein fehler"))),
    }
}