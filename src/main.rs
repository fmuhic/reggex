struct Reggex {
    fsm: Vec<[u8; 130]>
}

impl Reggex {
    fn new(exp: String) -> Reggex {
        let mut fsm = Vec::new();
        for letter in exp.chars() {
            match letter {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    let mut state_map: [u8; 130] = [0; 130];
                    state_map[letter as usize] = fsm.len() as u8 + 1;
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

    fn dump(&self) {
        for i in 0..130 {
            for state_map in &self.fsm {
                print!("[{}] \t", state_map[i])
            }
            println!();
        }
    }


    fn matches(&self, exp: &str) -> bool {
        let mut current_state = 0;
        for letter in exp.chars() {
            if current_state >= self.fsm.len() {
                return false;
            }

            let next_state = self.fsm[current_state][letter as usize] as usize;
            println!("next state is {}", next_state);

            if next_state == 0  {
                return false;
            }

            current_state = next_state;
        }
        
        if current_state < self.fsm.len() {
            return false
        } 

        println!("done, current is  {}, size is {}", current_state, self.fsm.len());   

        true
    }
}

fn main() {

    let rgx = Reggex::new("fudo".to_owned());
    rgx.dump();

    println!("fudo => {}", rgx.matches("fudo"));
    println!("afudo => {}", rgx.matches("afudo"));
    println!("fudod => {}", rgx.matches("fudod"));
}
