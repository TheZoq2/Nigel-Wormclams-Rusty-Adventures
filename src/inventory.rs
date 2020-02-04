use crate::constants;

#[derive(Clone)]
pub struct Inventory {
	pub name v: Vec<Box<dyn Item>>,
}