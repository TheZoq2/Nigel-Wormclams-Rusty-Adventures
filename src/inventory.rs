use crate::msg::Msg;

#[derive(Debug, Clone, PartialEq)]
pub enum InventoryError {
    Full,
    OutOfBounds,
}


pub trait Item {
    fn effect(&self) -> Msg;
}

pub struct Inventory<T> {
    v: Vec<Option<T>>,
}

impl<T> Inventory<T> {
    pub fn new(size: usize) -> Self {
        Self {
            v: (0..size).map(|_| None).collect(),
        }
    }

    pub fn add_item(&mut self, item: T) -> Result<(), InventoryError> {
        for slot in self.v.iter_mut() {
            if slot.is_none() {
                *slot = Some(item);
                return Ok(())
            }
        }
        Err(InventoryError::Full)
    }

    pub fn set_item(&mut self, item: Option<T>) {
        self.v.push(item);
    }

    pub fn peek_item(&self, position: usize) -> Result<&Option<T>, InventoryError> {
        Ok(&self.v[position])
    }

    pub fn take_item(&self, position: usize) -> Result<Option<T>, InventoryError> {
        unimplemented!()
    }
}




#[cfg(test)]
mod inventory_tests {
    use super::*;

    #[test]
    fn test_inventory() {
        let mut inventory = Inventory::new(1);
        inventory.add_item(1).unwrap();
        assert_eq!(Err(InventoryError::Full), inventory.add_item(1));

        assert_eq!(inventory.peek_item(0), Ok(&Some(1)));
        assert_eq!(inventory.peek_item(1), Err(InventoryError::OutOfBounds));
    }
}
