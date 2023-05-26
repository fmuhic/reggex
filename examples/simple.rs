use reggex::Reggex;

fn main() {
    //let rgx = Reggex::new("(f|s)(x|o|u)g|(fu)(du|do)");
    let rgx = Reggex::new("ff*ffudo(ab)(ab)");
    //println!("fudo  => {}", rgx.matches(""));
    // println!("fu => {}", rgx.matches("fu"));
    // println!("fudo1  => {}", rgx.matches("fudo1"));
}
