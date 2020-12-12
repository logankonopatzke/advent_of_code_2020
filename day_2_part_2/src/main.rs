use std::io::Read;

#[derive(Debug)]
struct Password {
    character: char,
    minimum: i32,
    maximum: i32,
    content: String,
}

impl Password {
    fn new(character: char, minimum: i32, maximum: i32, content: &str) -> Password {
        Password {
            character: character,
            minimum: minimum,
            maximum: maximum,
            content: String::from(content),
        }
    }

    fn valid(&self) -> bool {
        let cont = |num: i32| -> bool {
            return self.content.chars().nth(num as usize - 1).unwrap() == self.character;
        };

        if cont(self.minimum) && !cont(self.maximum) {
            return true;
        } else if cont(self.maximum) && !cont(self.minimum) {
            return true;
        } else {
            return false;
        }
    }
}

fn parse_passwords(content: &str) -> Vec<Password> {
    content
        .lines()
        .map(|line: &str| {
            let col_pos = line.find(":").unwrap();
            let dash_pos = line.find("-").unwrap();
            let first_white = line.find(" ").unwrap();

            let character = line.chars().nth(col_pos - 1).unwrap();
            let minimum: i32 = (&line[0..dash_pos]).parse().unwrap();
            let maximum: i32 = (&line[(dash_pos + 1)..first_white]).parse().unwrap();
            let password_content = &line[(col_pos + 2)..line.len()];
            Password::new(character, minimum, maximum, password_content)
        })
        .collect()
}

fn count_valid(password_list: &Vec<Password>) -> i32 {
    let mut count: i32 = 0;
    for i in password_list {
        if i.valid() {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut file = std::fs::File::open("inputs/day_2.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let password_list = parse_passwords(&content);
    let valid_count = count_valid(&password_list);

    println!("{}", valid_count);
}
