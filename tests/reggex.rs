use reggex::Reggex;

#[test]
fn test_match() {
    let rgx = Reggex::new("^fu.o$");

    assert!(rgx.matches("fudo"));
}
