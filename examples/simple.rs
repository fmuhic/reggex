use reggex::Reggex;

fn main() {
    let rgx = Reggex::new("fudo");
    println!("fudo  => {}", rgx.matches("fudo"));
}
