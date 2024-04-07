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

#[test]
fn adding_body() {
    let mut x = Engine::default();
    x.add_body(Body::new(rbt::body::BodyType::GPoint, 1.0));
    assert_eq!(1, x.count_bodies())
}
