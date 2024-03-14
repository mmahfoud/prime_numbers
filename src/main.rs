use stopwatch::Stopwatch;

fn get_prime_numbers_less_than(max: u64) -> Vec<u64> {
    // all numbers (before sifting) from 2 to max
    let mut numbers: Vec<u64> = (2..=max).collect();

    // caculate where to stop
    let square_root_of_max = (max as f64).sqrt();

    // start sifting from 2 (i.e at index 0)
    let mut i = 0;
    while i < numbers.len() {
        // this number is prime since it is not a multiple of any of the lesser primes
        let &n = numbers.get(i).unwrap();

        // break the loop when reaching a number that is bigger than the root of the upper limit,
        // because all its multiples were already removed.
        if (n as f64) > square_root_of_max {
            break;
        }

        // eliminate all multiples of n: retain other elements
        numbers.retain(|&x| x <= n || x % n != 0);

        // move to the next prime
        i += 1;
    }
    numbers
}

fn main() {
    let sw = Stopwatch::start_new();
    let primes = get_prime_numbers_less_than(100);
    println!("{:?}", sw.elapsed());
    println!("{:?}", primes);
}
