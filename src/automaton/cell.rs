#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub status: CellStatus,
    pub postion: Coordinates,
}
#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinates { x, y }
    }
}

impl Coordinates {
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellStatus {
    Alive,
    Dead,
}

impl Cell {
    pub fn new(postion: Coordinates) -> Self {
        Cell {
            postion,
            status: CellStatus::Dead,
        }
    }

    pub fn new_from_status(postion: Coordinates, status: CellStatus) -> Self {
        Cell { postion, status }
    }

    pub fn change_state(&mut self, status: CellStatus) {
        self.status = status;
    }
    pub fn get_coordinates(&self) -> &Coordinates {
        &self.postion
    }
    pub fn get_status(&self) -> &CellStatus {
        &self.status
    }
}
