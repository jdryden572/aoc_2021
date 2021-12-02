fn main() {
    let answer_one = part1("input.txt");
    println!("Answer one: {}", answer_one);

    let answer_two = part2("input.txt");
    println!("Answer two: {}", answer_two);
}

fn parse_commands(file_name: &'static str) -> impl Iterator<Item = Command> {
    helpers::read_lines_panicky(file_name)
        .map(Command::parse)
}

fn part1(file_name: &'static str) -> i32 {
    let final_position = Position::calculate_final(parse_commands(file_name));
    final_position.horizontal * final_position.depth
}

#[derive(Debug, Default)]
struct Position {
    horizontal: i32,
    depth: i32,
}

impl CommandHandler for Position {
    fn handle(self, command: Command) -> Self {
        match command {
            Command::Forward(dist) => Self { horizontal: self.horizontal + dist, depth: self.depth },
            Command::Down(dist) => Self { horizontal: self.horizontal, depth: self.depth + dist },
            Command::Up(dist) => Self { horizontal: self.horizontal, depth: self.depth - dist },
        }
    }
}

fn part2(file_name: &'static str) -> i32 {
    let final_position = PositionWithAim::calculate_final(parse_commands(file_name));
    final_position.horizontal * final_position.depth
}

#[derive(Default)]
struct PositionWithAim {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl PositionWithAim {
    fn handle_forward(self, dist: i32) -> Self {
        let depth_change = self.aim * dist;
        Self { 
            horizontal: self.horizontal + dist, 
            depth: self.depth + depth_change,
            aim: self.aim,
        }
    }

    fn change_aim(self, change: i32) -> Self {
        Self { 
            horizontal: self.horizontal, 
            depth: self.depth, 
            aim: self.aim + change 
        }
    }
}

impl CommandHandler for PositionWithAim {
    fn handle(self, command: Command) -> Self {
        match command {
            Command::Forward(dist) => self.handle_forward(dist),
            Command::Down(change) => self.change_aim(change),
            Command::Up(change) => self.change_aim(-change),
        }
    }
}

trait CommandHandler where Self: Default {
    fn handle(self, command: Command) -> Self;

    fn calculate_final<I>(commands: I) -> Self where I: Iterator<Item = Command> {
        let initial = Self::default();
        commands.fold(initial, |pos, command| pos.handle(command))
    }
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn parse<S: AsRef<str>>(line: S) -> Self {
        let mut parts = line.as_ref().split(" ");
        let direction = parts.next().expect("Direction").to_lowercase();
        let distance: i32 = parts.next().expect("Distance").parse().expect("Distance integer");
        match direction.as_ref() {
            "forward" => Command::Forward(distance),
            "down" => Command::Down(distance),
            "up" => Command::Up(distance),
            _ => panic!("Unrecognized command direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(150, part1("test_input.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(1989265, part1("input.txt"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(900, part2("test_input.txt"));
    }

    #[test]
    fn final_part2() {
        assert_eq!(2089174012, part2("input.txt"));
    }
}