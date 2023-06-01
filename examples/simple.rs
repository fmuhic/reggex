use reggex::Reggex;

fn main() {
    let rx = Reggex::new("(fu)(du|do)(du|do)");
    println!("fudo  => {}", rx.matches("fudodo"));
}
