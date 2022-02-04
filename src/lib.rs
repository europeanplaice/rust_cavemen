mod stats_module;

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_mean() {
        assert_eq!(stats_module::calc_mean(&vec![1., 2., 3.,]), 2.);
        assert_eq!(stats_module::calc_mean(&vec![1., 1., 1.,]), 1.);
    }
    #[test]
    fn test_var() {
        assert!(stats_module::calc_variance(&vec![3., 4., 5.,]) - 0.666666 < 0.0001);
        assert!(stats_module::calc_variance(&vec![0., 0., 0.,]) == 0.);
    }
    #[test]
    fn test_std() {
        assert!(stats_module::calc_std(&vec![3., 4., 5.,]) - 0.8164 < 0.0001);
        assert!(stats_module::calc_std(&vec![0., 0., 0.,]) == 0.);
    }

    #[test]
    fn test_standard_norm_dist(){
        assert_eq!(stats_module::norm_dist_pdf(&0.), 0.3989422804014327);
        assert_eq!(stats_module::norm_dist_pdf(&1.), 0.24197072451914337);
        assert_eq!(stats_module::norm_dist_pdf(&-1.), 0.24197072451914337);
    }
}

