use regex::Regex;

pub enum Direction {
    Left,
    Right,
}

pub struct Rotation(Direction, u32);

impl TryFrom<String> for Rotation {
    type Error = &'static str;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^(?<direction>L|R)(?<distance>\d+)$").unwrap();

        let Some(captures) = re.captures(&input) else {
            return Err("invalid input");
        };

        let distance: u32 = captures["distance"].parse().unwrap();
        let direction = match &captures["direction"] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };

        Ok(Self(direction, distance))
    }
}

pub struct StartingPosition(pub u32);
pub struct MaxPosition(pub u32);

pub struct Dial {
    position: u32,
    total_positions: i32,
}

impl Dial {
    pub fn new(position: StartingPosition, max: MaxPosition) -> Self {
        Self {
            position: position.0,
            total_positions: (max.0 + 1).try_into().unwrap(),
        }
    }

    pub fn position(&self) -> u32 {
        self.position
    }
}

impl Dial {
    pub fn rotate(&mut self, rotation: Rotation) {
        let current_position: i32 = self.position.try_into().unwrap();
        let distance: i32 = rotation.1.try_into().unwrap();

        self.position = match rotation {
            Rotation(Direction::Left, _) => {
                (current_position - distance).rem_euclid(self.total_positions)
            }
            Rotation(Direction::Right, _) => {
                (current_position + distance).rem_euclid(self.total_positions)
            }
        }
        .try_into()
        .unwrap()
    }
}
