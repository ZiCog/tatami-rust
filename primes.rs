// Prime mumber generator.
//
// Derived from E.J.Olsen's wheel.c
//
use crate::defs::defs;

pub struct Primes {
    pub primes: Vec<defs::PrimeType>,
}

impl Primes {
    pub fn new(n: usize) -> Primes {
        let mut primes = Primes { primes: vec![0; n] };

        let mut p: defs::PrimeType;
        primes.primes[0] = 2;
        primes.primes[1] = 3;
        let mut pn: usize = 2;
        let mut in_: usize = 1;

        fn isprime(p: defs::PrimeType, in_: &mut usize, primes: &[defs::PrimeType]) -> bool {
            for prime in primes.iter().take(*in_).skip(1) {
                if p % prime == 0 {
                    return false;
                }
            }
            let mut i = *in_;
            while primes[i] * primes[i] <= p {
                if p % primes[i] == 0 {
                    return false;
                }
                i += 1;
            }
            *in_ = i - 1;
            true
        }

        p = 5;
        while pn < primes.primes.len() {
            if isprime(p, &mut in_, &primes.primes) {
                primes.primes[pn] = p;
                pn += 1;
            }
            p += 2;
        }

        primes
    }
}
