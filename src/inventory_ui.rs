use crate::math::{vec2, Vec2};
use crate::msg::{Msg, Cmd, MouseButtonChange, MouseButton};
use crate::inventory::Inventory;

use sdl2::render::{Canvas, RenderTarget};
use sdl2::pixels::Color;
use sdl2::rect::Rect;



/**
  An item that can be stored in the inventory
*/
pub struct InventoryItem {
    pub name: String,
    pub image: String,
    pub context_menu_entries: Vec<(String, Msg)>
}


enum State {
    /// Nothing special is happening
    Default,
    /// A context menu is active at the specified position relative
    /// to the top left corner. If the mouse was last seen hovering
    /// over one of the items, the index of that item is in highlighted
    ContextMenuVisible{position: Vec2, highlighted: Option<usize>},
}

pub struct InventoryUi {
    position: Vec2,
    // Amount of columns per row of the inventory
    columns: usize,
    // Size of each item in pixels
    item_size: Vec2, // TODO: Replace with a vec2 once we have one
    // Content of the inventory
    pub inventory: Inventory<InventoryItem>,
    // Current state of the inventory
    state: State
}


impl InventoryUi {
    pub fn new(
        capacity: usize,
        columns: usize,
        item_size: Vec2,
        position: Vec2
    ) -> Self {
        Self {
            columns,
            item_size,
            inventory: Inventory::new(capacity),
            state: State::Default,
            position
        }
    }

    pub fn on_mouse_button(self, event: MouseButtonChange) -> (Self, Vec<Cmd>) {
        let (with_mouse_move, cmds) = self.on_mouse_move(event.pos);
        match event {
            MouseButtonChange{pos, button: MouseButton::Left, pressed: true} => {
                // TODO: Handle left clicks
                with_mouse_move.on_left_click(pos)
            }
            MouseButtonChange{pos, button: MouseButton::Right, pressed: true} => {
                with_mouse_move.on_right_click(pos)
            }
            _ => (with_mouse_move, vec!())
        }
    }

    pub fn on_left_click(self, pos: Vec2) -> (Self, Vec<Cmd>) {
        if !self.is_on_context_menu(pos) {
            let state = State::Default;
            (Self{state, .. self}, vec!())
        }
        else {
            (self, vec!())
        }
    }


    pub fn on_right_click(self, pos: Vec2) -> (Self, Vec<Cmd>) {
        if !self.is_on_context_menu(pos) {
            let state = State::ContextMenuVisible{
                position: pos,
                highlighted: None
            };
            (Self{state, .. self}, vec!())
        }
        else {
            (self, vec!())
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
    pub fn context_menu_size(&self) -> Vec2 {
        return vec2(200., 50.);
    }

    /// Returns true if the specified position is over the inventory 
    pub fn is_on_inventory(&self, pos: Vec2) -> bool {
        self.position.x < pos.x && self.position.x + self.size().x > pos.x &&
            self.position.y < pos.y && self.position.y + self.size().y > pos.y
    }

    pub fn is_on_context_menu(&self, pos: Vec2) -> bool {
        match self.state {
            State::ContextMenuVisible{position, ..} => {
                let size = self.context_menu_size();
                position.x < pos.x && position.x + size.x > pos.x &&
                    position.y < pos.y && position.y + size.y > pos.y
            }
            _ => {false}
        }
    }

    pub fn context_menu_entries(&self) -> Vec<(String, Cmd)> {
        if let State::ContextMenuVisible{position, ..} = self.state {
            unimplemented!()
        }
        else {
            vec![]
        }
    }

    /**
      Returns the item that is stored at the specified position if such an item
      exists. The position should be relative to the top left corner of the
      inventory
    */
    pub fn item_at_pos(&self, rel_pos: Vec2) -> &Option<InventoryItem> {
        let x_index = (rel_pos.x / self.item_size.x).floor() as usize;
        if x_index <= self.columns {
            let y_index = (rel_pos.y / self.item_size.y).floor() as usize;
            let index = x_index + y_index * self.columns;
            if index < self.inventory.capacity() {
                self.inventory.peek_item(index).unwrap_or(&None)
            }
            else {
                &None
            }
        }
        else {
            &None
        }
    }
}


impl InventoryUi {
    pub fn draw<T>(&self, canvas: &mut Canvas<T>) -> Result<(), String>
        where T: RenderTarget
    {
        let pos = self.position;
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

        if let State::ContextMenuVisible{position, highlighted} = self.state {
            let size = self.context_menu_size();
            canvas.fill_rect(
                Rect::new(
                    position.x as i32,
                    position.y as i32,
                    size.x as u32,
                    size.y as u32
                )
            )?;
        }
        Ok(())
    }
}
