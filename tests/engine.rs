use rbt::{body::Body, engine::Engine};

#[test]
fn adding_body() {
    let mut x = Engine::default();
    x.add_body(Body::new(rbt::body::BodyType::GPoint, 1.0));
    assert_eq!(1, x.count_bodies())
}
