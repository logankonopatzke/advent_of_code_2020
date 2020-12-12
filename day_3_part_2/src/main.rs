use std::io::Read;

const NODES_PER_LINE: i32 = 31;

#[allow(dead_code)]
enum Movement {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }
}

// Note: x position will need to be wrapped around
// Interface for managing our movement through the terrain
struct Terrain {
    parsed_content: Vec<String>,
}

impl Terrain {
    fn new(content: &str) -> Terrain {
        let parse_content =
            |data: &str| -> Vec<String> { data.lines().map(|ts: &str| String::from(ts)).collect() };

        Terrain {
            parsed_content: parse_content(content),
        }
    }

    // Due to the nature of the input, the x value will eventually extend out of the horizontal range
    // We have to deal with this by wrapping it around to the other side
    fn wrap_x_value(&self, x: i32) -> i32 {
        let wrap_factor: i32 = x / NODES_PER_LINE;
        let k = NODES_PER_LINE * wrap_factor;
        let t = x - k;

        return t;
    }

    fn at(&self, position: &Position) -> Option<char> {
        match self.parsed_content.iter().nth(position.y as usize) {
            Some(y_value) => match y_value.chars().nth(position.x as usize) {
                Some(x_value) => return Some(x_value),
                None => {
                    return self.at(&Position {
                        x: self.wrap_x_value(position.x),
                        y: position.y,
                    })
                }
            },
            None => None, // End of list
        }
    }
}

struct Player {
    position: Position,
}

impl Player {
    fn new(position: Position) -> Player {
        Player { position: position }
    }
}

struct World {
    player: Player,
    terrain: Terrain,
}

impl World {
    fn new(player: Player, terrain: Terrain) -> World {
        World {
            player: player,
            terrain: terrain,
        }
    }

    fn trees_to_encounter(&mut self, movements: &Vec<Movement>) -> i32 {
        let mut movement_offset = Position::new(0, 0);
        for i in movements {
            match i {
                Movement::Left(value) => movement_offset.x -= value,
                Movement::Right(value) => movement_offset.x += value,
                Movement::Up(value) => movement_offset.y -= value,
                Movement::Down(value) => movement_offset.y += value,
            }
        }

        let mut sim_trees_hit: i32 = 0;
        loop {
            let rel_pos = Position {
                x: self.player.position.x + movement_offset.x,
                y: self.player.position.y + movement_offset.y,
            };
            self.player.position = rel_pos;

            let node = self.terrain.at(&self.player.position);

            match node {
                Some(value) => {
                    if value == '#' {
                        sim_trees_hit += 1;
                    }
                }
                None => break,
            }
        }

        self.player.position = Position { x: 0, y: 0 };
        // Reset the player's position

        return sim_trees_hit;
    }
}

fn main() {
    let mut file = std::fs::File::open("inputs/day_3.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut world = World::new(Player::new(Position::new(0, 0)), Terrain::new(&content));

    let trees_1 = world.trees_to_encounter(&vec![Movement::Right(1), Movement::Down(1)]);
    let trees_2 = world.trees_to_encounter(&vec![Movement::Right(3), Movement::Down(1)]);
    let trees_3 = world.trees_to_encounter(&vec![Movement::Right(5), Movement::Down(1)]);
    let trees_4 = world.trees_to_encounter(&vec![Movement::Right(7), Movement::Down(1)]);
    let trees_5 = world.trees_to_encounter(&vec![Movement::Right(1), Movement::Down(2)]);

    let total_tree_count =
        trees_1 as i64 * trees_2 as i64 * trees_3 as i64 * trees_4 as i64 * trees_5 as i64;
    println!("{}", total_tree_count);
}
