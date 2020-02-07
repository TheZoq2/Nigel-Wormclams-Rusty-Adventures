use crate::math::{vec2, Vec2};
use crate::msg::{Msg, Cmd};
use crate::inventory::Inventory;

use sdl2::render::{Canvas, RenderTarget};
use sdl2::pixels::Color;
use sdl2::rect::Rect;



/**
  An item that can be stored in the inventory
*/
pub struct InventoryItem {
    name: String,
    image: String,
    context_menu_entries: Vec<(String, Msg)>
}


pub enum InventoryUiState {
    /// Nothing special is happening
    Default,
    /// A context menu is active at the specified position relative
    /// to the top left corner. If the mouse was last seen hovering
    /// over one of the items, the index of that item is in highlighted
    ContextMenuVisible{position: Vec2, highlighted: Option<usize>},
}
pub struct InventoryUi {
    // Amount of columns per row of the inventory
    columns: usize,
    // Size of each item in pixels
    item_size: Vec2, // TODO: Replace with a vec2 once we have one
    // Content of the inventory
    pub inventory: Inventory<InventoryItem>,
    // Current state of the inventory
    state: InventoryUiState
}


impl InventoryUi {
    pub fn new(capacity: usize, columns: usize, item_size: Vec2) -> Self {
        Self {
            columns,
            item_size,
            inventory: Inventory::new(capacity),
            state: InventoryUiState::Default,
        }
    }

    pub fn on_right_click(self, position: Vec2) -> (Self, Vec<Cmd>) {
        let (with_mouse_move, cmds) = self.on_mouse_move(position);
        (
            Self {
                state: InventoryUiState::ContextMenuVisible{
                    position,
                    highlighted: None
                }, .. with_mouse_move},
            cmds
        )
    }

    pub fn on_left_click(self, position: Vec2) -> (Self, Vec<Cmd>) {
        let (with_mouse_move, cmds) = self.on_mouse_move(position);
        match &with_mouse_move.state {
            InventoryUiState::Default => {
                (with_mouse_move, cmds)
            }
            InventoryUiState::ContextMenuVisible{..} => {
                // TODO: Handle clicks in context menu
                (with_mouse_move, cmds)
            }
        }
    }

    pub fn on_mouse_move(self, position: Vec2) -> (Self, Vec<Cmd>) {
        (self, vec![])
    }


    pub fn size(&self) -> Vec2 {
        vec2(
            self.columns as f32 * self.item_size.x,
            (self.inventory.capacity() / self.columns) as f32 * self.item_size.y
        )
    }
}


impl InventoryUi {
    pub fn draw<T>(&self, canvas: &mut Canvas<T>) -> Result<(), String>
        where T: RenderTarget
    {
        let pos = vec2(300., 100.);
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Draw outer shape
        let size = self.size();
        canvas.draw_rect(
            Rect::new(pos.x as i32, pos.y as i32, size.x as u32, size.y as u32)
        )?;

        for (i, item) in self.inventory.content().iter().enumerate() {
            let x = (i % self.columns) as f32 * self.item_size.x;
            let y = (i / self.columns) as f32 * self.item_size.y;
            let rect = Rect::new(
                (pos.x + x) as i32,
                (pos.y+y) as i32,
                (self.item_size.x) as u32,
                (self.item_size.y) as u32
            );

            if let None = item {
                canvas.draw_rect(rect)?;
            }
            else {
                canvas.fill_rect(rect)?;
            }
        }
        Ok(())
    }
}
