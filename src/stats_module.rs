pub fn calc_mean(xs: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    let mut counter: f64 = 0.;
    for x in xs.iter(){
        sum += x;
        counter += 1.;
    };
    sum / counter
}

pub fn calc_variance(xs: &Vec<f64>) -> f64 {
    let xs_p2: Vec<f64> = xs.iter().map(|z| z * z).collect();
    let exp_of_power:f64 = calc_mean(&xs_p2);
    let power_of_exp: f64 = calc_mean(&xs).powf(2.);
    exp_of_power - power_of_exp
}

pub fn calc_std(xs: &Vec<f64>) -> f64 {
    let var: f64 = calc_variance(xs);
    let std: f64 = var.sqrt();
    std
}

