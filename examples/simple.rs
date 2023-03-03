use reggex::Reggex;

fn main() {

    let rgx = Reggex::new("^fu+.o$");

    println!("fudo  => {}", rgx.matches("fudo"));
    println!("fumo  => {}", rgx.matches("fumo"));
    println!("fuuuuuudo  => {}", rgx.matches("fuuuuuudo"));
    println!("fudu  => {}", rgx.matches("fudu"));
    println!("afudo => {}", rgx.matches("afudo"));
    println!("fudod => {}", rgx.matches("fudod"));

    let rgx2 = Reggex::new("^fu*.o$");
    println!("fudo  => {}", rgx2.matches("fudo"));
    println!("fdo  => {}", rgx2.matches("fddo"));
    println!("fumo  => {}", rgx2.matches("fumo"));
    println!("fuuuuuudo  => {}", rgx2.matches("fuuuuuudo"));
}
