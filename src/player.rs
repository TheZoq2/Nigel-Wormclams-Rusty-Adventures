use crate::constants;

#[derive(Clone)]
pub struct Player {
	pub name: String,
	pub position: Vec2,
	pub speed: i16,
	pub strength: u16,
	pub intellect: u16,
	pub charisma: u16,
	pub spirit: u16,
	pub agility: u16,
}

impl Player {
	pub fn new(name: String, position: Vec2, speed: i16,
				strength: u16, intellect: u16, charisma: u16,
				intellect: u16, charisma: u16, spirit: u16,
				agility: u16) -> Player {
			
		Player {
			name: name,
			position: position,
			speed: speed,
			intellect: intellect,
			charisma: charisma,
			spirit: spirit,
			agility: agility,
		}
	}
}