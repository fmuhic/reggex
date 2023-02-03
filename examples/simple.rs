use reggex::Reggex;

fn main() {

    let rgx = Reggex::new("^fu.o$");
    rgx.dump();

    println!("fudo  => {}", rgx.matches("fudo"));
    println!("fumo  => {}", rgx.matches("fumo"));
    println!("fudu  => {}", rgx.matches("fudu"));
    println!("afudo => {}", rgx.matches("afudo"));
    println!("fudod => {}", rgx.matches("fudod"));
}
