fn main() {
    println!("Hello, world!");
    get_result();
}

fn get_result() {
    let mut numbers: Vec<u32> = Vec::new();
    let result: Vec<&str> = include_str!("input.txt").lines().map(|line| line).collect();

    for line in result {
        let mut pair: Vec<u32> = Vec::new();
        for char in line.chars() {
            match char.to_digit(10) {
                None => continue,
                Some(digit) => {
                    pair.push(digit);
                    break;
                }
            }
        }

        for char in line.chars().rev() {
            match char.to_digit(10) {
                None => continue,
                Some(digit) => {
                    pair.push(digit);
                    break;
                }
            }
        }
        numbers.push(concat(&pair));
    }

    println!("{:?}", numbers.iter().sum::<u32>());
}

fn concat(vec: &[u32]) -> u32 {
    let mut acc = 0;
    for elem in vec {
        acc *= 10;
        acc += elem;
    }
    acc
}
