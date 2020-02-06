use crate::msg::Msg;


#[derive(Debug, Clone, PartialEq)]
pub enum InventoryError {
    Full,
    OutOfBounds,
}

type Result<T> = std::result::Result<T, InventoryError>;


/**
  A generic inventory content manager. Keeps track of items in the inventory and
  allows addition and removal of them
*/

pub struct Inventory<T> {
    v: Vec<Option<T>>,
}

impl<T> Inventory<T> {
    /**
        Create a new inventory with the specified size
    */
    pub fn new(size: usize) -> Self {
        Self {
            v: (0..size).map(|_| None).collect(),
        }
    }

    pub fn content<'a>(&'a self) -> &'a [Option<T>] {
        &self.v
    }

    /**
      Insert the specified item into the first available slot. Returns
      Inventory::Full if the inventory is full
    */
    pub fn add_item(&mut self, item: T) -> Result<()> {
        for slot in self.v.iter_mut() {
            if slot.is_none() {
                *slot = Some(item);
                return Ok(())
            }
        }
        Err(InventoryError::Full)
    }

    /**
      Set the item at the specified location to a new item
    */
    pub fn set_item(&mut self, position: usize, item: Option<T>) -> Result<()> {
        if position >= self.v.len() {
            Err(InventoryError::OutOfBounds)
        }
        else {
            self.v[position] = item;
            Ok(())
        }
    }

    /**
      Returns the content at the specified location in the inventory
    */
    pub fn peek_item(&self, position: usize) -> Result<&Option<T>> {
        if position >= self.v.len() {
            Err(InventoryError::OutOfBounds)
        }
        else {
            Ok(&self.v[position])
        }
    }

    /**
      Removes and returns  the content at the the specified location in the inventory
    */
    pub fn take_item(&mut self, position: usize) -> Result<Option<T>> {
        if position >= self.v.len() {
            Err(InventoryError::OutOfBounds)
        }
        else {
            Ok(self.v[position].take())
        }
    }

    pub fn capacity(&self) -> usize {
        self.v.len()
    }
}


#[cfg(test)]
mod inventory_tests {
    use super::*;

    #[test]
    fn test_inventory() {
        let mut inventory = Inventory::new(2);
        // Adding items work
        inventory.add_item(1).unwrap();
        inventory.add_item(2).unwrap();
        // Adding more than the capacity causes error
        assert_eq!(Err(InventoryError::Full), inventory.add_item(1));

        // Peeking at items works
        assert_eq!(inventory.peek_item(0), Ok(&Some(1)));
        assert_eq!(inventory.peek_item(1), Ok(&Some(2)));
        // But not if the peek address is out of bounds
        assert_eq!(inventory.peek_item(2), Err(InventoryError::OutOfBounds));

        // Taking items works
        assert_eq!(inventory.take_item(0), Ok(Some(1)));
        // And items are removed
        assert_eq!(inventory.peek_item(0), Ok(&None));
        // But not if the address is out of bounds
        assert_eq!(inventory.take_item(2), Err(InventoryError::OutOfBounds));

        // Setting items works
        assert_eq!(inventory.set_item(0, Some(3)), Ok(()));
        assert_eq!(inventory.peek_item(0), Ok(&Some(3)));

        // But not if the address is out of bounds
        assert_eq!(inventory.set_item(2, None), Err(InventoryError::OutOfBounds));
    }
}
