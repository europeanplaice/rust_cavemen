/// Returns the mean of the vector.
pub fn calc_mean(xs: &Vec<f32>) -> f32 {
    let mut sum: f32 = 0.0;
    let mut counter: f32 = 0.;
    for x in xs.iter(){
        sum += x;
        counter += 1.;
    };
    sum / counter
}

/// Returns the variance of the vector.
pub fn calc_variance(xs: &Vec<f32>) -> f32 {
    let xs_p2: Vec<f32> = xs.iter().map(|z| z * z).collect();
    let exp_of_power:f32 = calc_mean(&xs_p2);
    let power_of_exp: f32 = calc_mean(&xs).powf(2.);
    exp_of_power - power_of_exp
}

/// Returns the standard deviation of the vector.
pub fn calc_std(xs: &Vec<f32>) -> f32 {
    let var: f32 = calc_variance(xs);
    let std: f32 = var.sqrt();
    std
}

/// Returns the pdf of Normal Distribution.
pub fn norm_dist_pdf(x: &f32) -> f32 {
    let a: f32 = std::f32::consts::PI * 2.0;
    let b: f32 = a.sqrt();
    let c: f32 = 1. / b;
    let d: f32 = (- (x * x) / 2.).exp();
    c * d
}


pub mod counting {
    //! this is a module for counting.
    use crate::math::factrial;

    /// Counting with ordered and without replacement.  
    /// Returns the number of possible arrangements of size r from n objects.
    pub fn ordered_without_replacement(n: &u32, r: &u32) -> Result<u32, String> {
        if n < r {
            return Err("n must be larger then r.".to_string());
        } else {
            return Ok(factrial(n) / factrial(&(n - r)))
        }
    }

    /// Counting with ordered and with replacement.  
    /// Returns the number of possible arrangements of size r from n objects.
    pub fn count_ordered_with_replacement(n: &u32, r: &u32) -> Result<u32, String> {
        if n < r {
            return Err("n must be larger then r.".to_string());
        } else {
            return Ok(n.pow(*r))
        }
    }

    /// Counting with unordered and without replacement.  
    /// Returns the number of possible arrangements of size r from n objects.
    pub fn count_unordered_without_replacement(n: &u32, r: &u32) -> Result<u32, String> {
        if n < r {
            return Err("n must be larger then r.".to_string());
        } else {
            return Ok(factrial(n) / (factrial(&(n - r)) * factrial(r)))
        }
    }

    /// Counting with unordered and with replacement.  
    /// Returns the number of possible arrangements of size r from n objects.
    pub fn count_unordered_with_replacement(n: &u32, r: &u32) -> Result<u32, String> {
        if n < r {
            return Err("n must be larger then r.".to_string());
        } else {
            return Ok(factrial(&(n + r - 1)) / (factrial(&(n + r - 1 - r)) * factrial(r)))
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
        assert_eq!(norm_dist_pdf(&1.), 0.24197073);
        assert_eq!(norm_dist_pdf(&-1.), 0.24197073);
    }

    #[test]
    fn test_ordered_without_replacement(){
        let result = counting::ordered_without_replacement(&4, &6);
        match result {
            Ok(_v) => panic!("this test shuld fail."),
            Err(v) => assert_eq!(v, "n must be larger then r.".to_string()),
        }  
        assert_eq!(counting::ordered_without_replacement(&4, &2).unwrap(), 12);
    }

    #[test]
    fn test_count_ordered_with_replacement(){
        let result = counting::count_ordered_with_replacement(&4, &6);
        match result {
            Ok(_v) => panic!("this test shuld fail."),
            Err(v) => assert_eq!(v, "n must be larger then r.".to_string()),
        }  
        assert_eq!(counting::count_ordered_with_replacement(&10, &3).unwrap(), 1000);
    }

    #[test]
    fn test_count_unordered_without_replacement(){
        let result = counting::count_ordered_with_replacement(&4, &6);
        match result {
            Ok(_v) => panic!("this test shuld fail."),
            Err(v) => assert_eq!(v, "n must be larger then r.".to_string()),
        }  
        assert_eq!(counting::count_unordered_without_replacement(&10, &4).unwrap(), 210);
    }

    #[test]
    fn test_count_unordered_with_replacement(){
        let result = counting::count_ordered_with_replacement(&4, &6);
        match result {
            Ok(_v) => panic!("this test shuld fail."),
            Err(v) => assert_eq!(v, "n must be larger then r.".to_string()),
        }  
        assert_eq!(counting::count_unordered_with_replacement(&10, &3).unwrap(), 220);
    }
}

