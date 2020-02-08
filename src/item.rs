pub trait Item {
    fn use_item(&mut self);
}

#[derive(Clone)]
pub struct Helmet {
}

#[derive(Clone)]
pub struct ChestArmor {
}

#[derive(Clone)]
pub struct Trousers {
}

#[derive(Clone)]
pub struct Boots {
}

#[derive(Clone)]
pub struct Weapon {
}
