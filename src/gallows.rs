use core::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    State1 = 1,
    State2 = 2,
    State3 = 3,
    State4 = 4,
    State5 = 5,
    State6 = 6,
    State7 = 7,
    StateLast = 8,
}
impl State {
    pub fn as_str(self) -> &'static str {
        match self {
            State::State1 => STATE_1,
            State::State2 => STATE_2,
            State::State3 => STATE_3,
            State::State4 => STATE_4,
            State::State5 => STATE_5,
            State::State6 => STATE_6,
            State::State7 => STATE_7,
            State::StateLast => STATE_8,
        }
    }
}

pub struct StateManager { state: State }
impl StateManager {
    pub fn new() -> Self {
        StateManager { state: State::State1 }
    }

    pub fn next_state(&mut self) {
        /* Tries to set next state, if it reached finish it leaves the last state */
        let new_state = StateManager::get_state_from_u8(
            self.state as u8 + 1,
        );
        self.state = match new_state {
            Some(v) => v,
            None => self.state
        };
    }

    pub fn is_last_state(&self) -> bool {
        self.state == State::StateLast
    }

    fn get_state_from_u8(num: u8) -> Option<State> {
        match num {
            1 => Some(State::State1),
            2 => Some(State::State2),
            3 => Some(State::State3),
            4 => Some(State::State4),
            5 => Some(State::State5),
            6 => Some(State::State6),
            7 => Some(State::State7),
            8 => Some(State::StateLast),
            _ => None
        }
    }
}
impl fmt::Display for StateManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state.as_str())
    }
}

const STATE_1: &'static str = r#"
            -------------
             |          |
                        |
                        |
                        |
                        |
                        |
                 --------------
                ----------------
               ------------------
"#;

const STATE_2: &'static str = r#"
            -------------
             |          |
             0          |
                        |
                        |
                        |
                        |
                 --------------
                ----------------
               ------------------
"#;

const STATE_3: &'static str = r#"
            -------------
             |          |
             0          |
             |          |
                        |
                        |
                        |
                 --------------
                ----------------
               ------------------
"#;

const STATE_4: &'static str = r#"
            -------------
             |          |
             0          |
            /|          |
                        |
                        |
                        |
                 --------------
                ----------------
               ------------------
"#;

const STATE_5: &'static str = r#"
            -------------
             |          |
             0          |
            /|\         |
                        |
                        |
                        |
                 --------------
                ----------------
               ------------------
"#;

const STATE_6: &'static str = r#"
            -------------
             |          |
             0          |
            /|\         |
             |          |
                        |
                        |
                 --------------
                ----------------
               ------------------
"#;

const STATE_7: &'static str = r#"
            -------------
             |          |
             0          |
            /|\         |
             |          |
              \         |
                        |
                 --------------
                ----------------
               ------------------
"#;

const STATE_8: &'static str = r#"
            -------------
             |          |
             0          |
            /|\         |
             |          |
            / \         |
                        |
                 --------------
                ----------------
               ------------------
"#;
