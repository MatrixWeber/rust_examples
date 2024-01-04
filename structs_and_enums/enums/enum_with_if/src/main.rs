#[derive(PartialEq)]
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

    // If-Else-Struktur, um abhängig vom Zustand unterschiedliche Aktionen auszuführen
    if State::Idle == current_state {
        println!("Das System ist im Leerlauf.");
    } else if let State::Running = current_state {
        println!("Das System läuft.");
    } else if let State::Paused = current_state {
        println!("Das System ist pausiert.");
    } else if let State::Finished = current_state {
        println!("Das System ist beendet.");
    }
}
