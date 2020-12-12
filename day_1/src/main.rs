use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("inputs/day_1.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let my_vec: Vec<i32> = content.lines().map(|s| s.parse().unwrap()).collect();

    for (i, k) in my_vec.iter().enumerate() {
        if i == my_vec.len() - 1 {
            // Bounds checking
            break;
        }
        let my_slice = &my_vec[i + 1..my_vec.len()];

        for v in my_slice {
            if k + v == 2020 {
                println!("{}", k * v);
            }
        }
    }
}
