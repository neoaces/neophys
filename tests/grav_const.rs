use rbt::{
    body::Body,
    engine::Engine,
    force::{gravity::Gravity, Force},
};

#[test]
fn grav_const() {
    let x = Gravity::default();
    assert_eq!(9.82, x.calc())
}
