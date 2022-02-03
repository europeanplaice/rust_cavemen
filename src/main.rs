mod lib;

fn main() {
    let res: f64 = lib::calc_variance(&vec![1., 2., 3.]);
    println!("result is {}", res);
}
