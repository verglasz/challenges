#!/usr/bin/env rust-script


fn hilight(map: &[&str], coords: &[(usize, usize)]) -> () {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if coords.contains(&(x, y)) {
                print!("\x1b[1;31m{}\x1b[0m", c);
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!();
}

fn main() {
    const MAX: usize = 118;
    const SQ: usize = 5850;
    let map = "#########\n#E+++C#+#\n###-#G#+#\n##+++@#+#\n#++++#M+#\n#M+++|+|#\n#+-M+|++#\n##+++##|#\n#########".split("\n").collect::<Vec<&str>>();
    let x = map[0].len();
    let y = map.len();
    for a in 1..MAX {
        for b in a..MAX {
            let c = MAX - a - b;
            if c < b {
                continue;
            }
            if a * a + b * b + c * c == SQ {
                println!("{}, {}, {}", a, b, c);
                let ax = a % x;
                let ay = a / x;
                let bx = b % x;
                let by = b / x;
                let cx = c % x;
                let cy = c / x;
                println!("{:?} {:?} {:?}", (ax, ay), (bx, by), (cx, cy));
                hilight(&map, &[(ax, ay), (bx, by), (cx, cy)]);
            }
        }
    }
}
