use crate::constants;

#[derive(Clone)]
pub struct Inventory {
	pub v: Vec<Box<dyn Item>>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            v = vec!(),
        }
    }
    
    pub fn add(&mut self, Box<dyn item>) {
        v.push(item);
    }
}