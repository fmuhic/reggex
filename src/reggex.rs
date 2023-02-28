use crate::parser::parse_expression;

type State = [u8; STATE_SIZE];
type FinateStateMachine = Vec<State>;

const STATE_SIZE: usize = 130;
const LINE_START: usize = 128;
const LINE_END: usize = 129;
const ASCII_SIZE: usize = 127;

pub struct Reggex {
    fsm: FinateStateMachine
}

impl Reggex {
    pub fn new(exp: &str) -> Reggex {
        let tokens = parse_expression(exp);
        println!("Tokens {:?}", tokens);

        let mut fsm = FinateStateMachine::new();
        for letter in exp.chars() {
            match letter {
                'a'..= 'z' | 'A'..= 'Z' | '0'..= '9' => {
                    let mut state_map = [0; STATE_SIZE];
                    state_map[letter as usize] = fsm.len() as u8 + 1;
                    fsm.push(state_map);
                }
                '^' => {
                    let mut state_map = [0; STATE_SIZE];
                    state_map[LINE_START] = fsm.len() as u8 + 1;
                    fsm.push(state_map);
                }
                '$' => {
                    let mut state_map = [0; STATE_SIZE];
                    state_map[LINE_END] = fsm.len() as u8 + 1;
                    fsm.push(state_map);
                }
                '.' => {
                    #[allow(unused_mut)]
                    let mut state_map = core::array::from_fn(|i| {
                        match i {
                            1 ..= ASCII_SIZE => fsm.len() as u8 + 1,
                            _ => 0
                        }
                    });
                    fsm.push(state_map);
                }
                _ => {
                    println!("Invalid char {}", letter);
                    unreachable!()
                }
            }
        }
        Reggex { fsm }
    }

    pub fn dump(&self) {
        for i in 0 .. STATE_SIZE {
            for state_map in &self.fsm {
                print!("[{}] \t", state_map[i])
            }
            println!();
        }
    }

    pub fn matches(&self, exp: &str) -> bool {
        let mut current_state = self.get_default_state();

        for letter in exp.chars() {
            if current_state >= self.fsm.len() {
                return false;
            }

            let next_state = self.fsm[current_state][letter as usize] as usize;
            println!("Next state is {}", next_state);

            if next_state == 0 {
                return false;
            }

            current_state = next_state;
        }

        return if current_state < self.fsm.len() {
            self.match_line_end(current_state)
        }
        else {
            true
        }
    }

    fn get_default_state(&self) -> usize {
        let default_state = 0;
        if self.fsm[default_state][LINE_START] != 0 {
            default_state + 1
        } else {
            default_state
        }
    }


    fn match_line_end(&self, current_state: usize) -> bool {
        current_state == self.fsm.len() - 1 && self.fsm[current_state][LINE_END] != 0
    }
}
