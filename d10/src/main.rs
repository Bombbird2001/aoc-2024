use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let map = contents.lines().map(|line|
                                       line.chars().map(|c|
                                           c.to_string().parse::<i8>().unwrap()
                                       ).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    println!("Score: {}", score(&map));

    println!("Rating: {}", rating(&map));

    Ok(())
}

fn score(map: &Vec<Vec<i8>>) -> usize {
    let mut reachable_peaks: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            search_peaks(row, col, &map, &mut reachable_peaks);
        }
    }

    reachable_peaks.iter().filter(|(key, _)|
        map[key.0][key.1] == 0).fold(0, |acc, ((_, _), peaks)| acc + peaks.len())
}

fn search_peaks(row: usize, col: usize, map: &Vec<Vec<i8>>, reachable_peaks: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>) {
    if reachable_peaks.contains_key(&(row, col)) {
        return;
    }

    if map[row][col] == 9 {
        let mut peaks = HashSet::new();
        peaks.insert((row, col));
        reachable_peaks.insert((row, col), peaks);
        return;
    }

    let mut peak_union: HashSet<(usize, usize)> = HashSet::new();
    if row >= 1 && map[row - 1][col] - map[row][col] == 1 {
        search_peaks(row - 1, col, map, reachable_peaks);
        peak_union = peak_union.union(reachable_peaks.get(&(row - 1, col)).unwrap()).map(|peak| *peak).collect();
    }
    if row + 1 < map.len() && map[row + 1][col] - map[row][col] == 1 {
        search_peaks(row + 1, col, map, reachable_peaks);
        peak_union = peak_union.union(reachable_peaks.get(&(row + 1, col)).unwrap()).map(|peak| *peak).collect();
    }
    if col >= 1 && map[row][col - 1] - map[row][col] == 1 {
        search_peaks(row, col - 1, map, reachable_peaks);
        peak_union = peak_union.union(reachable_peaks.get(&(row, col - 1)).unwrap()).map(|peak| *peak).collect();
    }
    if col + 1 < map[row].len() && map[row][col + 1] - map[row][col] == 1 {
        search_peaks(row, col + 1, map, reachable_peaks);
        peak_union = peak_union.union(reachable_peaks.get(&(row, col + 1)).unwrap()).map(|peak| *peak).collect();
    }

    reachable_peaks.insert((row, col), peak_union);
}

fn rating(map: &Vec<Vec<i8>>) -> usize {
    let mut reachable_paths: HashMap<(usize, usize), usize> = HashMap::new();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            search_paths(row, col, &map, &mut reachable_paths);
        }
    }

    reachable_paths.iter().filter(|(key, _)|
        map[key.0][key.1] == 0).fold(0, |acc, ((_, _), peaks)| acc + peaks)
}

fn search_paths(row: usize, col: usize, map: &Vec<Vec<i8>>, reachable_paths: &mut HashMap<(usize, usize), usize>) -> usize {
    if reachable_paths.contains_key(&(row, col)) {
        return *reachable_paths.get(&(row, col)).unwrap();
    }

    if map[row][col] == 9 {
        return 1;
    }

    let mut paths = 0usize;
    if row >= 1 && map[row - 1][col] - map[row][col] == 1 {
        paths += search_paths(row - 1, col, map, reachable_paths);
    }
    if row + 1 < map.len() && map[row + 1][col] - map[row][col] == 1 {
        paths += search_paths(row + 1, col, map, reachable_paths);
    }
    if col >= 1 && map[row][col - 1] - map[row][col] == 1 {
        paths += search_paths(row, col - 1, map, reachable_paths);
    }
    if col + 1 < map[row].len() && map[row][col + 1] - map[row][col] == 1 {
        paths += search_paths(row, col + 1, map, reachable_paths);
    }

    reachable_paths.insert((row, col), paths);

    paths
}