enum State {
    // Varianten des Enums
    Idle,
    Running,
    Paused,
    Finished,
}

fn main() {
    // Verwendung des Enums
    let current_state = State::Running;

    // Pattern Matching, um abhängig vom Zustand unterschiedliche Aktionen auszuführen
    match current_state {
        State::Idle => println!("Das System ist im Leerlauf."),
        State::Running => println!("Das System läuft."),
        State::Paused => println!("Das System ist pausiert."),
        State::Finished => println!("Das System ist beendet."),
    } // => ist eine Kurzschreibweise für { println!("Das System läuft.") }
}     // => wird für einen Einzeiler verwendet