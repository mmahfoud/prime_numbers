use stopwatch::Stopwatch;

fn eliminate_multiples(a: &mut Vec<usize>, n: usize) -> &mut Vec<usize> {
    a.retain(|&x| x <= n || x % n != 0);
    a
}

fn main() {
    const SIZE: usize = 10_000_000;
    // all numbers (before sifting)
    let mut numbers: Vec<usize> = (2..SIZE).into_iter().collect();
    // start sifting from 2
    let mut i = 0;
    // start the timer
    let sw = Stopwatch::start_new();
    while i < numbers.len() {
        let n = numbers[i];
        if (n as f64) > (SIZE as f64).sqrt() {
            break;
        }
        numbers = eliminate_multiples(&mut numbers, n).to_vec();
        //print!("{:?}\n====\n{:?}\n===============\n", n, numbers);
        i = i + 1;
    }
    // stop the timer
    println!("{:?}", sw.elapsed());
    println!("{:?}", numbers);
}
