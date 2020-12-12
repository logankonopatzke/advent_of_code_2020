use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("inputs/day_1_part_2.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let my_vec: Vec<i32> = content.lines().map(|s| s.parse().unwrap()).collect();

    for x in 0..my_vec.len() {
        if x == my_vec.len() - 1 {
            break;
        }
        let my_slice = &my_vec[x + 1..my_vec.len()];

        for y in 0..my_slice.len() {
            if y == my_slice.len() - 1 {
                break;
            }
            let my_slice_2 = &my_slice[y + 1..my_slice.len()];

            for z in 0..my_slice_2.len() {
                let sum = my_vec[x] + my_slice[y] + my_slice_2[z];
                if sum == 2020 {
                    println!("{}", my_vec[x] * my_slice[y] * my_slice_2[z]);
                }
            }
        }
    }
}
