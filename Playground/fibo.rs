fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    let num_terms = 10;

    println!("Sequência Fibonacci com {} termos:", num_terms);

    for i in 0..num_terms {
        print!("{} ", fibonacci(i));
    }
}