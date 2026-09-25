// Append within composite.rs module at handoff boundary.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_support_norm_composes_interval_and_arithmetic_error() {
        for values in [[0.0,0.0,0.0],[3.0,4.0,0.0],[-3.0,0.0,-4.0],[1e300,1e-300,0.0],[1e-200,0.0,0.0]] {
            let result=norm(values,values.map(|v|[v,v])).unwrap();
            assert!(result.interval[0]<=result.value && result.value<=result.interval[1]);
            assert!(result.relative<=1e-9);
            if values==[3.0,4.0,0.0] { assert_eq!(result.value,5.0);assert!(result.interval[0]<=5.0&&result.interval[1]>=5.0); }
        }
        assert!(norm([3.0,4.0,0.0],[[2.9,3.1],[4.0,4.0],[0.0,0.0]]).is_err());
        assert!(norm([f64::from_bits(1),0.0,0.0],[[f64::from_bits(1);2],[0.0;2],[0.0;2]]).is_err());
        assert!(norm([1.7e308,1.7e308,0.0],[[1.7e308;2],[1.7e308;2],[0.0;2]]).is_err());
    }
}
