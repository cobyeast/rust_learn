// Fibonacii function generates first 100 numbers in squence
pub fn fibonacii() -> Vec<u128> {
    // u128 holds digits until 340282366920938463463374607431768211455
    let mut fib: Vec<u128> = vec![0, 1];

    for i in 2..100 {
        fib.push(fib[i - 1] + fib[i - 2]);
    }

    return fib;
}
