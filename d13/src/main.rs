use std::fs::File;
use std::io::{Read, Result};
use regex::Regex;

#[derive(Debug)]
struct Game {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize)
}

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.split('\n').collect::<Vec<&str>>();
    let mut line_no = 0usize;
    let button_regex = Regex::new(r".+X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r".+X=(\d+), Y=(\d+)").unwrap();
    let mut cost = 0;
    while line_no < lines.len() {
        let button_a = button_regex.captures(&lines[line_no]).unwrap();
        let button_b = button_regex.captures(&lines[line_no + 1]).unwrap();
        let prize = prize_regex.captures(&lines[line_no + 2]).unwrap();
        let game = Game {
            button_a: (button_a.get(1).unwrap().as_str().parse::<usize>().unwrap(), button_a.get(2).unwrap().as_str().parse::<usize>().unwrap()),
            button_b: (button_b.get(1).unwrap().as_str().parse::<usize>().unwrap(), button_b.get(2).unwrap().as_str().parse::<usize>().unwrap()),
            prize: (prize.get(1).unwrap().as_str().parse::<usize>().unwrap(), prize.get(2).unwrap().as_str().parse::<usize>().unwrap())
        };
        cost += get_cost(&game);
        line_no += 4;
    }

    println!("Total cost: {}", cost);

    Ok(())
}

fn get_cost(game: &Game) -> usize {
    // println!("{:?}", game);
    let mut cost = 0;
    let (mut x, mut y) = (game.prize.0, game.prize.1);
    let mut exceeded = false;
    // let mut b_pressed = 0;

    // if game.button_a.0 as f32 / game.button_b.0 as f32 == game.button_a.1 as f32 / game.button_b.1 as f32 {
    //     panic!("Collinear buttons");
    // }

    while x % game.button_a.0 != 0 || y % game.button_a.1 != 0 || x / game.button_a.0 != y / game.button_a.1 {
        // println!("{} {}", x, y);
        if x < game.button_b.0 || y < game.button_b.1 {
            exceeded = true;
            break;
        }
        x -= game.button_b.0;
        y -= game.button_b.1;
        cost += 1;
        // b_pressed += 1;
    }

    if exceeded {
        // println!("0");
        return 0;
    }

    // println!("A pressed {}", x / game.button_a.0);
    // println!("B pressed {}", b_pressed);
    // println!("{}", cost + 3 * x / game.button_a.0);
    cost + 3 * x / game.button_a.0
}
