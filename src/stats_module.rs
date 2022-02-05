
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

pub fn norm_dist_pdf(x: &f64) -> f64 {
    use std;
    let a: f64 = std::f64::consts::PI * 2.0;
    let b: f64 = a.sqrt();
    let c: f64 = 1. / b;
    let d: f64 = (- (x * x) / 2.).exp();
    c * d
}


pub mod count {
    use crate::math::factrial;
    pub fn ordered_without_replacement(n: &u64, r: &u64) -> Result<u64, String> {
        if n < r {
            return Err("n must be larger then r.".to_string());
        } else {
            return Ok(factrial(n) / factrial(&(n - r)))
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_mean() {
        assert_eq!(calc_mean(&vec![1., 2., 3.,]), 2.);
        assert_eq!(calc_mean(&vec![1., 1., 1.,]), 1.);
    }
    #[test]
    fn test_var() {
        assert!(calc_variance(&vec![3., 4., 5.,]) - 0.666666 < 0.0001);
        assert!(calc_variance(&vec![0., 0., 0.,]) == 0.);
    }
    #[test]
    fn test_std() {
        assert!(calc_std(&vec![3., 4., 5.,]) - 0.8164 < 0.0001);
        assert!(calc_std(&vec![0., 0., 0.,]) == 0.);
    }

    #[test]
    fn test_standard_norm_dist(){
        assert_eq!(norm_dist_pdf(&0.), 0.3989422804014327);
        assert_eq!(norm_dist_pdf(&1.), 0.24197072451914337);
        assert_eq!(norm_dist_pdf(&-1.), 0.24197072451914337);
    }

    #[test]
    fn test_ordered_without_replacement(){
        let result = count::ordered_without_replacement(&4, &6);
        match result {
            Ok(v) => panic!("this test shuld fail."),
            Err(v) => assert_eq!(v, "n must be larger then r.".to_string()),
        }  
        assert_eq!(count::ordered_without_replacement(&4, &2).unwrap(), 12);
    }
}

