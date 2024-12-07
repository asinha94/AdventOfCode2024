use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn check_page_ordering(page_ordering: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut should_not_see: HashSet<i32> = HashSet::new();
    
    for page in page_ordering {
        if should_not_see.contains(&page) {
            return 0
        }

        /* Insert all the dependencies of out product into the set */
        match rules.get(&page) {
            Some(dependencies) => {
                for dependency in dependencies {
                    should_not_see.insert(*dependency);
                }
            }
            None => ()
        }
    }

    let mid = page_ordering.len() / 2;
    return page_ordering[mid];

}

pub fn part1() {
    let input = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day05.txt")
        .unwrap();

    let rules_list: Vec<_> = input
        .lines()
        .filter_map(|line|
            line.contains('|')
            .then(||
                line.split('|')
                .map(|w| 
                    w.parse::<i32>().unwrap())
                .collect::<Vec<_>>()))
        .collect();

    /* Rules are keyed by the dependency */
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules_list.iter() {
        rules.entry(rule[1])
            .and_modify(|r| r.push(rule[0]))
            .or_insert(vec![rule[0]]);
    }

    let produce_lists: Vec<_> = input
        .lines()
        .filter_map(|line|
            line.contains(',')
            .then(||
                line.split(',')
                .map(|w| 
                    w.parse::<i32>().unwrap())
                .collect::<Vec<_>>()))
        .collect();


    /* Check if lists are valid  */
    let sum: i32 =  produce_lists.iter()
        .map(|produce| check_page_ordering(produce, &rules))
        .sum();

    println!("Got midpoint sum: {sum}");
    
}


pub fn part2() {

    let input = fs::read_to_string("/Users/anu/Documents/programming/AoC/2024/input/day05.txt")
        .unwrap();

    /* Parse out rules list */
    let rules_list: Vec<_> = input
        .lines()
        .filter_map(|line|
            line.contains('|')
            .then(||
                line.split('|')
                .map(|w| 
                    w.parse::<i32>().unwrap())
                .collect::<Vec<_>>()))
        .collect();

    /* Rules are keyed by the dependency */
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules_list.iter() {
        rules.entry(rule[1])
            .and_modify(|r| r.push(rule[0]))
            .or_insert(vec![rule[0]]);
    }

    /* Parse the page order lists */
    let produce_lists: Vec<_> = input
        .lines()
        .filter_map(|line|
            line.contains(',')
            .then(||
                line.split(',')
                .map(|w| 
                    w.parse::<i32>().unwrap())
                .collect::<Vec<_>>()))
        .collect();

     /* Get invalid lists  */
     let invalid_produce_page_lists: Vec<_> =  produce_lists.iter()
        .filter(|produce| check_page_ordering(produce, &rules) == 0)
        .collect();

    let mut sum = 0;
    for page_order in invalid_produce_page_lists {

        let mut remaining_dependencies = HashSet::new();
        for page in page_order {
            remaining_dependencies.insert(*page);
        }

        let mut final_order = vec![];
        for page in page_order {

            /* BFS: Create a queue of pages we need to see */
            let mut dependencies_to_check = VecDeque::new();
            let mut dependencies_to_add = vec![];
            dependencies_to_check.push_back(*page);
            while dependencies_to_check.len() != 0 {
                
                let dependency = dependencies_to_check.pop_front().unwrap();
                if remaining_dependencies.contains(&dependency) {
                    match rules.get(&dependency) {
                        Some(deps) => {
                            for dep in deps {
                                dependencies_to_check.push_back(*dep);
                            }
                        },
                        None => ()
                    }
                };

                dependencies_to_add.push(dependency);
            }

            /* Insert into final list, in the order that we should see it */
            for dependency in dependencies_to_add.iter().rev() {
                if remaining_dependencies.contains(dependency) {
                    remaining_dependencies.remove(dependency);
                    final_order.push(*dependency);
                }
            }
        }

        let mid = final_order.len() / 2;
        sum += final_order[mid];
    }

    println!("Got sum: {sum}");

}
