use strum_macros::EnumIter;

// Direction needs to do two things:
// * First, it needs to allow easy conversion from one direc to a neighbor
// * Second, it should contain the values that can be added to the lf's location in the MoveForward
// case.
// I do not like this implementation. I think it should be able to happen with an enum.
// Direction should be an enum that simply knows how to increment and decrement itself.
static DIRECTIONS: &'static [(i8, i8)] = &[
    (0, 1), // 0 = North
    (1, 1), // 1 = NorthEast
    (1, 0), // 2 = East
    (1, -1), // 3 = SouthEast
    (0, -1), // 4 = South
    (-1, -1), // 5 = SouthWest
    (-1, 0), // 6 = West
    (-1, 1), // 7 = NorthWest
];

#[derive(Debug, Clone)]
pub struct Direction {
    direction: u8,
}

impl Direction {
    pub fn new() -> Self {
        Self { direction: 0 }
    }

    pub fn turn_left(&mut self) {
        self.direction = ((self.direction + 8) - 1) % 8;
    }

    pub fn turn_right(&mut self) {
        self.direction = (self.direction + 1) % 8;
    }

    pub fn get_forward_modifier(&self) -> (i8, i8) {
        DIRECTIONS[self.direction as usize]
    }

    pub fn name(&self) -> DirectionName {
        match self.direction {
            0 => DirectionName::North,
            1 => DirectionName::NorthEast,
            2 => DirectionName::East,
            3 => DirectionName::SouthEast,
            4 => DirectionName::South,
            5 => DirectionName::SouthWest,
            6 => DirectionName::West,
            7 => DirectionName::NorthWest,
            _ => panic!("Not facing a direction"),
        }
    }
}

// TODO Still I'm feeling like with the numbers at least we should be able to switch.
// So basically I want to say, from turn left, "return a Me minus one". Then get_forward_modifier
// could be basically the same.
#[derive(Debug, EnumIter, Clone, PartialEq)]
pub enum DirectionName {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}
