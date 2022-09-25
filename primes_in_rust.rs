fn main() {
    let mut primes  : Vec<u64> = Vec::with_capacity(1000000);  
    primes.push(2);
    println!("{}",primes[0]);

    for n in 2..1000000 {
        let mut is_prime = true;
        for prime in &primes{

            if n % prime  == 0{
                is_prime = false;
                break;
            }

        } 
        if is_prime{
            primes.push(n);
            println!("{}",n);
        }
    }
// println!("{}",array);
}