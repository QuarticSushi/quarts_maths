pub fn factors(n: usize) -> Vec<usize> {
    let mut num: usize = n;
    let mut divisor: usize = 2;
    let mut facts: Vec<usize> = [].to_vec();
    
    while num > 1 {
        if num % divisor == 0 {
            facts.push(divisor);
            num /= divisor;
        } else {
            divisor += 1;
        }
    }
    facts
}