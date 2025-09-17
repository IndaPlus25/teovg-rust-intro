use std::io;

fn disToEdge(r: usize, k: usize, x: usize, y: usize) -> usize {
    let top = y;
    let left = x;
    let right = k - 1 - x;
    let bottom = r - 1 - y;

    [top, left, right, bottom].iter().copied().min().unwrap()
}

fn main() {
    // hantera kattis input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let r = nums[0];
    let k = nums[1];

    // skapa 2d vec för att kunna räkna avstånd
    let mut matris: Vec<Vec<char>> = vec![vec![' '; k]; r];

    //^ fick tips för detta, annars gjorde jag en for loop som fyllde varje slot, mycket mer effektivt att göra så här

    for y in 0..r {
        for x in 0..k {
            let distance = disToEdge(r, k, x, y);
            let steps = distance + 1;
            matris[y][x] = if steps <= 9 {
                std::char::from_digit(steps as u32, 10).unwrap()
            } else {
                '.'
            };
        }
    }

    // skriv ut matrisen
    for row in matris {
        for character in row {
            print!("{}", character);
        }
        println!();
    }
}
