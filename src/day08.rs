use std::{cmp, collections::{HashMap, HashSet}, fs};


fn is_within_map(x: i32, y: i32, xmax: usize, ymax: usize) ->bool {
    let x_in_bounds = x >= 0 && x <= (xmax as i32);
    let y_in_bounds = y >= 0 && y <= (ymax as i32);
    x_in_bounds && y_in_bounds
}

fn get_antinodes_count(antennas: &Vec<(i32, i32)>, positions: &mut HashSet<(i32, i32)>, xmax: usize, ymax: usize) {
    if antennas.len() == 1 {
        return;
    }

    antennas.iter()
        .enumerate()
        .for_each(|(i, (ax, ay))| {
            antennas.iter()
                .enumerate()
                .for_each(|(j, (bx ,by))| {
                    if i == j {
                        return;
                    }

                    /* Vector AB i.e A -> B = B - A */
                    let delta_x = bx - ax;
                    let delta_y = by - ay;

                    // antinode near a
                    let (aa_x, aa_y) = (ax - delta_x, ay - delta_y);
                    if is_within_map(aa_x, aa_y, xmax, ymax) {
                        positions.insert((aa_x, aa_y));
                    }

                    // antinode near b
                    let (ab_x, ab_y) = (bx + delta_x, by + delta_y);
                    if is_within_map(ab_x, ab_y, xmax, ymax) {
                        positions.insert((ab_x, ab_y));
                    }

                });
        });
}

pub fn part1() {
    let mut antennas = HashMap::new();
    let (mut xmax, mut ymax) = (0, 0);
    fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day08.txt").unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, row)| {
            ymax = cmp::max(ymax, y);
            row.chars()
            .enumerate()
            .for_each(|(x, c)| {
                xmax = cmp::max(xmax, x);
                if c.is_ascii_alphanumeric() {
                    antennas.entry(c)
                        .and_modify(|v: &mut Vec<(i32, i32)>| v.push((x as i32,y as i32)))
                        .or_insert_with(|| vec![(x as i32,y as i32)]) ;
                }
            });
        });

    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    antennas.iter()
        .for_each(|(_, v)| {
            get_antinodes_count(v, &mut positions, xmax, ymax);
        });

    println!("Got sum: {}", positions.len());

}