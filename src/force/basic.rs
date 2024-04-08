use super::Force;

#[derive(Clone)]
pub struct Basic {
    m: f32,
}

impl Force for Basic {
    fn calc(&self) -> f32 {
        self.m
    }

    fn clone_dyn(&self) -> Box<dyn Force> {
        Box::new(self.clone())
    }
}
