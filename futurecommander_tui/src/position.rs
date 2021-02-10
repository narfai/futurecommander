
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

impl Position {
    pub fn new (x: u16, y: u16) -> Self {
        Position {
            x,
            y
        }
    }
    pub fn is_contained_by(&self, area: &Area) -> bool {
        self.x >= area.left_top.x && self.x <= area.right_bottom.x && self.y >= area.left_top.y && self.y <= area.right_bottom.y
    }

    pub fn to_tuple(&self) -> (u16, u16){
        (self.x, self.y)
    }
}

impl Eq for Position {}

#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub left_top: Position,
    pub right_bottom: Position
}

impl Area {    
    pub fn new(left_top: Position, right_bottom: Position) -> Self {
        Area {
            left_top,
            right_bottom
        }
    }

    pub fn inside(&self, cell_number: u16) -> Area {
        fn sub_if_positive(n: u16, cn: u16) -> u16{ if n > 0 { n - cn } else { 0 } }
        Area {
            left_top: Position { x: self.left_top.x + cell_number, y: self.left_top.y + cell_number},
            right_bottom: Position { x: sub_if_positive(self.right_bottom.x, cell_number), y: sub_if_positive(self.right_bottom.y, cell_number)},
        }
    }    

    //left percent
    //right percent    
}