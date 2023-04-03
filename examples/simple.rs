use reggex::Reggex;

fn main() {
    let rgx = Reggex::new("^(fu)(do$)$");
    println!("fudo  => {}", rgx.matches("fudo"));
    println!("fudo1  => {}", rgx.matches("fudo1"));
}
