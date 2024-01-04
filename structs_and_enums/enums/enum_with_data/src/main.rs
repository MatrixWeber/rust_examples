enum State {
    // Varianten des Enums
    Idle,
    Running(String), // Variante mit Daten vom Typ "String"
    Paused(usize),   // Variante mit Daten vom Typ "usize"
    Finished,
}

fn main() {
    // Bei der Instanziierung des Enums werden Daten mitgegeben
    let current_state = State::Running(String::from("Alex"));

    // Pattern Matching, um abhängig vom Zustand unterschiedliche Aktionen auszuführen
    match current_state {
        State::Idle => println!("Das System ist im Leerlauf."),
        State::Running(s) => println!("Das System wurde vom {} gestartet!", s),
        State::Paused(seconds) => println!("Das System ist pausiert für {}s.", seconds),
        State::Finished => println!("Das System ist beendet."),
    }
}