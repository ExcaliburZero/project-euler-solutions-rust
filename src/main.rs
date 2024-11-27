use std::collections::BTreeMap;
use std::env;

use project_euler::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let solvers = get_solvers();

    println!("{}", solvers[&args[1].parse::<u64>().unwrap()]());
}

fn get_solvers() -> BTreeMap<u64, Box<dyn Fn() -> String>> {
    let mut solvers: BTreeMap<u64, Box<dyn Fn() -> String>> = BTreeMap::new();

    solvers.insert(1, Box::new(prob_0001::solve));
    solvers.insert(2, Box::new(prob_0002::solve));
    solvers.insert(3, Box::new(prob_0003::solve));

    solvers
}
