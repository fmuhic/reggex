use reggex::Reggex;

fn main() {
    //let rgx = Reggex::new("(f|s)(x|o|u)g|(fu)(du|do)");
    let rgx = Reggex::new("fudo(ab)(ac)");
    println!("fudo  => {}", rgx.matches("fudoabac"));
    // println!("fu => {}", rgx.matches("fu"));
    // println!("fudo1  => {}", rgx.matches("fudo1"));
}
