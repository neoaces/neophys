use super::Force;

pub struct Basic {
    m: f32,
}

impl Force for Basic {
    fn calc(&self) -> f32 {
        self.m
    }
}
