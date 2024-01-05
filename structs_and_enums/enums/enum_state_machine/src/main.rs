enum State {
    Idle(Option<u64>),
    Running(String),
    Paused(usize),
    Finished(Option<String>),
}

impl State {
    fn next_state(self) -> State {
        match self {
            State::Idle(Some(counter)) => {
                println!("Das System ist {} mal im Leerlauf gewesen.", counter);
                State::Running(String::from("next state"))
            } State::Running(s) => { println!("Das System wurde vom {} gestartet!", s);
                State::Paused(30)
            } State::Paused(seconds) => { println!("Das System ist pausiert für {}s.", seconds);
                State::Finished(None)
            } State::Finished(None) => { println!("Das System wurde ohne Fehler beendet.");
                State::Idle(Some(60))
            } _ => { println!("Keine Zustandsänderung.");
                self // Der aktuelle Zustand wird zurückgegeben
            } // alle anderen Fälle wie z.B. State::Idle(None)
        }
    }
}

fn main() {
    let mut current_state = State::Idle(Some(10));
    current_state = current_state.next_state(); // Übergang zu "Running"
    current_state = current_state.next_state(); // Übergang zu "Paused"
    current_state = current_state.next_state(); // Übergang zu "Finished"
    current_state = current_state.next_state(); // Übergang zu "Idle"

    current_state = State::Idle(None);
    current_state.next_state();
}