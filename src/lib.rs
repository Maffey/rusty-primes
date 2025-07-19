use pyo3::prelude::*;


#[pyfunction]
fn is_prime(number: u128) -> bool {
    if number <= 1 {
        return false;
    }
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }

    let number_squared = number.isqrt();
    for i in (3..=number_squared).step_by(2) {
        if number % i == 0 {
            return false;
        }
    }

    true
}

#[pyfunction]
fn get_primes(stop: u128, should_print: bool) -> Vec<u128> {
    let mut primes: Vec<u128> = Vec::new();
    let mut current_number: u128 = 1;

    loop {
        if is_prime(current_number) {
            primes.push(current_number);
            if should_print { println!("Found prime: {current_number}"); }
        }

        current_number += 1;
        if current_number >= stop { break }
    }

    primes
}



#[pymodule]
fn rusty_primes(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(is_prime, module)?)?;
    module.add_function(wrap_pyfunction!(get_primes, module)?)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_small_numbers() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
    }

    #[test]
    fn test_is_prime_composites() {
        assert!(!is_prime(9));
        assert!(!is_prime(15));
        assert!(!is_prime(21));
        assert!(!is_prime(25));
        assert!(!is_prime(121));
        assert!(!is_prime(99));
        assert!(!is_prime(100));
    }

    #[test]
    fn test_is_prime_primes() {
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(is_prime(19));
        assert!(is_prime(23));
        assert!(is_prime(97));
        assert!(is_prime(101));
        assert!(is_prime(127));
    }

    #[test]
    fn test_get_primes_empty_range() {
        assert_eq!(get_primes(0, false), Vec::<u128>::new());
        assert_eq!(get_primes(1, false), Vec::<u128>::new());
        assert_eq!(get_primes(2, false), Vec::<u128>::new());
    }

    #[test]
    fn test_get_primes_small_range() {
        assert_eq!(get_primes(3, false), vec![2]);
        assert_eq!(get_primes(4, false), vec![2, 3]);
        assert_eq!(get_primes(5, false), vec![2, 3]);
        assert_eq!(get_primes(6, false), vec![2, 3, 5]);
    }

    #[test]
    fn test_get_primes_medium_range() {
        assert_eq!(get_primes(12, false), vec![2, 3, 5, 7, 11]);
        assert_eq!(get_primes(20, false), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_get_primes_with_printing() {
        let result = get_primes(8, true);
        assert_eq!(result, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_get_primes_stop_is_prime() {
        assert_eq!(get_primes(3, false), vec![2]);
        assert_eq!(get_primes(5, false), vec![2, 3]);
    }

    #[test]
    fn test_get_primes_large_stop() {
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        assert_eq!(get_primes(50, false), expected);
    }
}