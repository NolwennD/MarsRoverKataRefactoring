#[derive(Debug, Clone, Copy)]
struct Rover {
    heading: char,
    x: isize,
    y: isize,
}

impl Default for Rover {
    fn default() -> Self {
        Rover {
            heading: 'N',
            x: 0,
            y: 0,
        }
    }
}

impl Rover {
    fn run(&self, command: char, obstacles: &Vec<(usize, usize)>) -> (Self, bool) {
        if command == 'R' && self.heading == 'N' {
            return (Self {
                heading: 'E',
                x: self.x,
                y: self.y,
            }, true);
        }
        if command == 'R' && self.heading == 'E' {
            return (Self {
                heading: 'S',
                x: self.x,
                y: self.y,
            }, true);
        }
        if command == 'R' && self.heading == 'S' {
            return (Self {
                heading: 'W',
                x: self.x,
                y: self.y,
            }, true);
        }
        if command == 'R' && self.heading == 'W' {
            return (Self {
                heading: 'N',
                x: self.x,
                y: self.y,
            }, true);
        }
        if command == 'L' && self.heading == 'N' {
            return (Self {
                heading: 'W',
                x: self.x,
                y: self.y,
            }, true);
        }
        if command == 'L' && self.heading == 'E' {
            return (Self {
                heading: 'N',
                x: self.x,
                y: self.y,
            }, true);
        }
        if command == 'L' && self.heading == 'S' {
            return (Self {
                heading: 'E',
                x: self.x,
                y: self.y,
            }, true);
        }
        if command == 'L' && self.heading == 'W' {
            return (Self {
                heading: 'S',
                x: self.x,
                y: self.y,
            }, true);
        }
        if command == 'M' && self.heading == 'N' {
            let mut next_y = self.y + 1;
            if next_y > 9 {
                next_y = 0
            }
            if obstacles.contains(&(self.x as usize, next_y as usize)) {
                return (*self, false);
            }

            return (Self {
                heading: self.heading,
                x: self.x,
                y: next_y,
            }, true);
        }
        if command == 'M' && self.heading == 'S' {
            let mut next_y = self.y - 1;
            if next_y < 0 {
                next_y = 9
            }

            if obstacles.contains(&(self.x as usize, next_y as usize)) {
                return (*self, false);
            }

            return (Self {
                heading: self.heading,
                x: self.x,
                y: next_y,
            }, true);
        }
        if command == 'M' && self.heading == 'E' {
            let mut next_y = self.x + 1;
            if next_y > 9 {
                next_y = 0
            }

            if obstacles.contains(&(next_y as usize, self.y as usize)) {
                return (*self, false);
            }

            return (Self {
                heading: self.heading,
                x: next_y,
                y: self.y,
            }, true);
        }
        if command == 'M' && self.heading == 'W' {
            let mut next_y = self.x - 1;
            if next_y < 0 {
                next_y = 9
            }

            if obstacles.contains(&(next_y as usize, self.y as usize)) {
                return (*self, false);
            }

            return (Self {
                heading: self.heading,
                x: next_y,
                y: self.y,
            }, true);
        }

        (*self, true)
    }

    fn execute(&self, commands: &str, obstacles: Vec<(usize, usize)>) -> String {
        let mut rover = (self.clone(), true);
        for command in commands.chars() {
            rover = rover.0.run(command, &obstacles);
        }

        if !rover.1 {
            return format!("O:{}:{}:{}", rover.0.x, rover.0.y, rover.0.heading)
        }

        format!("{}:{}:{}", rover.0.x, rover.0.y, rover.0.heading)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_nothing_with_wrong_commands() {
        let rover = Rover {
            heading: 'W',
            x: 0,
            y: 0,
        };

        assert_eq!("0:0:W".to_string(), rover.execute("ABcdef", vec![]))
    }

    #[test]
    fn do_all_rotation() {
        let rover = Rover {
            heading: 'N',
            x: 0,
            y: 0,
        };

        assert_eq!("0:0:N".to_string(), rover.execute("LLLLRRRR", vec![]))
    }

    #[test]
    fn do_wrap_forward_south_and_north() {
        let rover = Rover {
            heading: 'S',
            x: 0,
            y: 0,
        };

        assert_eq!("0:0:N".to_string(), rover.execute("MRRM", vec![]))
    }

    #[test]
    fn do_wrap_forward_east_and_west() {
        let rover = Rover {
            heading: 'W',
            x: 0,
            y: 0,
        };

        assert_eq!("0:0:E".to_string(), rover.execute("MLLM", vec![]))
    }

    #[test]
    fn move_onto_north_obstacle() {
        let rover = Rover::default();

        assert_eq!("O:0:0:N".to_string(), rover.execute("M", vec![(0,1)]))
    }

    #[test]
    fn move_onto_south_obstacle() {
        let rover = Rover{heading: 'S', x: 0, y: 1};

        assert_eq!("O:0:1:S".to_string(), rover.execute("M", vec![(0,0)]))
    }

    #[test]
    fn move_onto_east_obstacle() {
        let rover = Rover{heading: 'E', x: 0, y: 0};

        assert_eq!("O:0:0:E".to_string(), rover.execute("M", vec![(1,0)]))
    }


    #[test]
    fn move_onto_west_obstacle() {
        let rover = Rover{heading: 'W', x: 1, y: 0};

        assert_eq!("O:1:0:W".to_string(), rover.execute("M", vec![(0,0)]))
    }
}
