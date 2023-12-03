pub struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl Position {
    pub fn new() -> Position {
        Position {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }

    pub fn move_simple(&mut self, command: &str, value: i32) -> () {
        match command {
            "forward" => self.horizontal += value,
            "down" => self.depth += value,
            "up" => self.depth -= value,
            _ => panic!("Invalid command found"),
        }
    }

    pub fn move_with_aim(&mut self, command: &str, value: i32) -> () {
        match command {
            "forward" => {
                self.horizontal += value;
                self.depth += self.aim * value
            }
            "down" => self.aim += value,
            "up" => self.aim -= value,
            _ => panic!("Invalid command found"),
        }
    }

    pub fn distance(&self) -> i32 {
        self.depth * self.horizontal
    }
}
