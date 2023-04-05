use reggex::Reggex;

fn main() {
    let rgx = Reggex::new("(f|s)(x|o|u)g|(fu)(du|do)");
    println!("fudo  => {}", rgx.matches("fudo"));
    // println!("fu => {}", rgx.matches("fu"));
    // println!("fudo1  => {}", rgx.matches("fudo1"));
}
