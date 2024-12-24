use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut dependencies = HashMap::new();
    let mut values = HashMap::new();
    let mut init_mode = true;

    for line in contents.lines() {
        if init_mode {
            if line.is_empty() {
                init_mode = false;
                continue;
            }

            let parts = line.split(": ").collect::<Vec<&str>>();
            values.insert(parts[0], parts[1].parse::<u64>().unwrap());
            continue;
        }

        let parts = line.split(" -> ").collect::<Vec<&str>>();
        let program = parts[1];
        let children = parts[0].split(" ").collect::<Vec<&str>>();
        dependencies.insert(program, children);
    }

    println!("Final result: {}", get_circuit_result(&mut values, &dependencies));

    find_sus_connections(&dependencies);

    Ok(())
}

fn get_circuit_result<'a>(values: &mut HashMap<&'a str, u64>, dependencies: &HashMap<&'a str, Vec<&'a str>>) -> u64 {
    let mut final_res: u64 = 0;
    for (program, _) in dependencies.iter() {
        if program.chars().nth(0).unwrap() != 'z' {
            continue;
        }

        let exp = program[1..].parse::<u32>().unwrap();

        let value = get_value(*program, values, &dependencies);
        final_res += value << exp;
    }

    final_res
}

fn get_value<'a>(program: &'a str, values: &mut HashMap<&'a str, u64>, dependencies: &HashMap<&'a str, Vec<&'a str>>) -> u64 {
    if let Some(value) = values.get(program) {
        return *value;
    }

    let children = dependencies.get(program).unwrap();
    let value1 = get_value(children[0], values, dependencies);
    let value2 = get_value(children[2], values, dependencies);
    let result = match children[1] {
        "AND" => value1 & value2,
        "OR" => value1 | value2,
        "XOR" => value1 ^ value2,
        _ => unreachable!("Unknown operator: {}", children[1])
    };

    values.insert(program, result);
    result
}

fn find_sus_connections(dependencies: &HashMap<&str, Vec<&str>>) {
    let mut values: HashMap<&str, u64> = HashMap::new();
    let strings = (0..45).map(|i| {
        (format_string('x', i), format_string('y', i))
    }).collect::<Vec<_>>();

    for i in (0..45).rev() {
        values.clear();
        for (j, (x, y)) in strings.iter().enumerate() {
            if i == j {
                values.insert(x, 1);
                values.insert(y, 1);
                continue;
            }
            values.insert(x, 0);
            values.insert(y, 0);
        }

        let result = get_circuit_result(&mut values, dependencies);
        let expected = 1u64 << (i + 1);
        if result != expected {
            println!("2^{} 1+1: {}, expected {}", i, result, expected);
        }

        values.clear();
        for (j, (x, y)) in strings.iter().enumerate() {
            if i == j {
                values.insert(x, 1);
                values.insert(y, 0);
                continue;
            }
            values.insert(x, 0);
            values.insert(y, 0);
        }

        let result = get_circuit_result(&mut values, dependencies);
        let expected = 1u64 << i;
        if result != expected {
            println!("2^{} 1+0: {}, expected {}", i, result, expected);
        }

        values.clear();
        for (j, (x, y)) in strings.iter().enumerate() {
            if i == j {
                values.insert(x, 0);
                values.insert(y, 1);
                continue;
            }
            values.insert(x, 0);
            values.insert(y, 0);
        }

        let result = get_circuit_result(&mut values, dependencies);
        let expected = 1u64 << i;
        if result != expected {
            println!("2^{} 0+1: {}, expected {}", i, result, expected);
        }
    }
}

fn format_string(ltr: char, num: u32) -> String {
    let mut str = String::new();
    str.push(ltr);
    if num < 10 {
        str.push('0');
        str.push_str(num.to_string().as_str());
    } else {
        str.push_str(num.to_string().as_str());
    }
    str
}