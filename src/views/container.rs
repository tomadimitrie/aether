use crate::types::view::View;
use std::any::Any;

pub struct Container {
    pub children: Vec<Box<dyn View>>,
}

impl View for Container {
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
}
