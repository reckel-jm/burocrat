mod bpmn;

pub fn add_calculation(a: i64, b:i64) -> i64 {
    a+b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_calculation() {
        for a in 1..10000 {
            let b: i64 = a*a;
            assert_eq!(add_calculation(a, a*b), a+a*b);
        }
    }
}