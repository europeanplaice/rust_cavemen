mod dp_module;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(args[1].clone()).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut a: Vec<i32> = Vec::new();
    for line in lines{
        a.push(line.unwrap().parse::<i32>().unwrap());
    }
    let result = dp_module::dp::find_subset(&a, args[2].parse::<usize>().unwrap());
    println!("{:?}", result);
}
