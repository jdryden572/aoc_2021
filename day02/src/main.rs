fn main() {
    let answer_one = part1("input.txt");
    println!("Answer one: {}", answer_one);
}

fn part1(file_name: &str) -> i32 {
    let initial_position = Position {
        horizontal: 0,
        depth: 0,
    };

    let final_position = helpers::read_lines_panicky(file_name)
        .map(Command::parse)
        .fold(initial_position, |pos, command| pos.handle(command));
    
    final_position.horizontal * final_position.depth
}

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    pub fn handle(self, command: Command) -> Self {
        match command {
            Command::Forward(dist) => Self { horizontal: self.horizontal + dist, depth: self.depth },
            Command::Down(dist) => Self { horizontal: self.horizontal, depth: self.depth + dist },
            Command::Up(dist) => Self { horizontal: self.horizontal, depth: self.depth - dist },
        }
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
        assert_eq!(150, part1("test_input_1.txt"));
    }

    #[test]
    fn final_part1() {
        assert_eq!(1989265, part1("input.txt"));
    }
}