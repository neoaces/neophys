use log::{debug, info};
use neophys::{body::Body, engine::Engine};

fn main() {
    env_logger::init();

    let mut engine = Engine::default();
    engine.add_body(Body::new(neophys::body::BodyType::GPoint, 10.0));
    println!("{:#?}", engine);
    for i in 1..=1000 {
        info!("Iteration #{}", i);
        engine.calc(i as f32, 0.001);
        println!("{:?}", engine);
    }
}
