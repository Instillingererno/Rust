use std::thread;
use std::cmp::min;

const START : u32 = 0;
const END : u32 = 100_000_001;
const PRINT : bool = false;
const NUMBER_OF_THREADS : u32 = 64; // 3 for 10k -> 64 for 100 000k
const BUFFER : usize = 500;

// --- HELPER FUNCTIONS ---

fn is_prime(n : u32) -> bool {
	if n == 2 || n == 3{
		return true;
	}
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
		return false;
	}
	let mut m = 5;
    let mut w = 2;
	while m <= (n as f32).sqrt() as u32 + 1 {
		if n % m == 0 {
			return false;
		}
		m += w;
        w = 6 - w;
	}
	true
}

// --- HELPER FUNCTION HELPER FUNCTION ---

fn estimate_number_of_primes(n : u32) -> usize {
    (n as f32 / ((n as f32).ln() + 1.0)) as usize
}

// ------


fn find_primes(from : u32, to : u32) -> Vec<u32> {
	let mut v = Vec::with_capacity(estimate_number_of_primes(to) + BUFFER - estimate_number_of_primes(from));
	for n in from..to {
		if is_prime(n) {
			v.push(n);
		}
	}
	v
}

// ------

fn primes(from : u32, to : u32) -> Vec<u32> {
    let mut children = vec![];

	let per = ((to - from) as f32 / NUMBER_OF_THREADS as f32).ceil() as u32; // Numbers assigned to every thread
	// Could be logarythmic to spread work better -> solution now is to use 32 threads.
	for n in 0..NUMBER_OF_THREADS {
    	children.push(thread::spawn(move || {
        	return find_primes(per * n + from, min(per * (n + 1) + from, to));
    	}));
    }

    let mut primes = Vec::new();
    primes.reserve(estimate_number_of_primes(to) + 10000 - estimate_number_of_primes(from));


    for child in children {
        for d in child.join() { // Get result from threads
            //println!("{:?}", d.len());
			for n in d {
				primes.push(n);
			}
		}
    }

    primes
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let p = primes(START, END);

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Seconds: {}", sec);

    println!("Primes from {0} to {1}:", START, END - 1);

    if PRINT {
        for n in p {
            print!("{:?} ", n);
        }
    }

    println!("[END]");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn found_all_primes() {
        let p = primes(0, 10000);
        assert_eq!(1229, p.len());
    }

    #[test]
    fn is_sorted() {
        let mut l = 0;
        for p in primes(0, 1000) {
            assert_eq!(false, p < l);
            l = p;
        }
    }

    #[test]
    fn check_if_prime() {
        assert_eq!(true, is_prime(2));
        assert_eq!(false, is_prime(4));
        assert_eq!(true, is_prime(5));
        assert_eq!(false, is_prime(9));
        assert_eq!(true, is_prime(13));
        assert_eq!(false, is_prime(33));
        assert_eq!(true, is_prime(17));
        assert_eq!(false, is_prime(12331));
        assert_eq!(true, is_prime(15485867));
        assert_eq!(false, is_prime(45689));
    }

    #[test]
    fn find_all_primes() {
        let v = find_primes(0, 13);
        assert_eq!(2, v[0]);
        assert_eq!(3, v[1]);
        assert_eq!(5, v[2]);
        assert_eq!(7, v[3]);
        assert_eq!(11, v[4]);
    }

}
