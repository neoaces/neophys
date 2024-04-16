use neophys::force::{Force, gravity::Gravity};

#[test]
fn grav_const() {
    let x = Gravity::default();
    assert_ne!(9.82, x.calc())
}
