#[derive(Debug)]
pub struct Group {
    pub number: u16,
    pub size: u16,
}

impl Group {
    pub fn new(number: u16, size: u16) -> Self {
        Group { number, size }
    }
}
