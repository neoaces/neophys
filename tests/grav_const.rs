use neophys::force::{gravity::Gravity, Force};

#[test]
fn grav_const() {
    let x = Gravity::default();
    assert_ne!(9.82, x.calc())
}
