use crate::error::TatamiError;
use std::slice::Iter;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub struct Factors {
    p: Vec<PrimeType>,
    n: Vec<PrimeType>,
}

impl Factors {
    fn new(size: usize) -> Factors {
        Factors {
            p: vec![0; size],
            n: vec![0; size],
        }
    }
}

pub struct Tatami {
    isn: PrimeType,
    factors: Factors,
    smin: PrimeType,
    z: Vec<PrimeType>,
    s: PrimeType,
    fmax: usize,
}

impl Tatami {
    pub fn new() -> Tatami {
        Tatami {
            isn: 0,
            factors: Factors::new(FNUM),
            smin: SMAX,
            z: vec![0; FNUM],
            s: 2,
            fmax: 0,
        }
    }

    fn free(&mut self, k: PrimeType, l: PrimeType) -> bool {
        let n = l / k;
        let lmin = (k + 1) * n + 2;
        let lmax = (k - 1) * (n + 1) - 2;
        (lmin <= l) && (l <= lmax)
    }

    fn sigma(&mut self) -> PrimeType {
        let mut r = self.factors.n[0];
        let fmax = self.fmax;
        for n in self.factors.n[1..=fmax].iter_mut() {
            r *= *n + 1;
        }
        r
    }

    fn t(&mut self) -> PrimeType {
        let mut r = 0;
        loop {
            let fmax = self.fmax;
            let mut found: bool = false;
            for (z, n) in self.z[0..=fmax].iter_mut().zip(&self.factors.n[0..=fmax]) {
                if *z < *n {
                    *z += 1;
                    found = true;
                    break;
                }
                *z = 0;
            }

            if !found {
                break;
            }
            let mut k = 1;
            let mut l = 1;

            for ((p, n), z) in self.factors.p[0..=fmax]
                .iter()
                .zip(&self.factors.n[0..=fmax])
                .zip(&self.z[0..=fmax])
            {
                k *= p.pow(*z as u32);
                l *= p.pow(*n as u32 - *z as u32);
            }

            if k <= l {
                r += self.free(k, l) as PrimeType;
            }
        }
        r
    }

    fn work(&mut self, p: PrimeType, mut pr: Iter<PrimeType>) {
        let s = self.s;
        let mut r = self.sigma();
        if r >= self.isn {
            r = self.t();
            if (r == self.isn) && (s < self.smin) {
                self.smin = s;
            }
        }
        let mut fmax = self.fmax;
        let pmax = self.smin / s + 1;
        if p <= pmax {
            self.factors.n[fmax] += 1;
            self.s = s * p;
            self.work(p, pr.clone());
            self.factors.n[fmax] -= 1;
        }
        fmax += 1;
        self.factors.n[fmax] = 1;
        while let Some(&p) = pr.next() {
            if p > pmax {
                break;
            }
            self.factors.p[fmax] = p;
            self.s = s * p;
            self.fmax = fmax;
            self.work(p, pr.clone());
        }
        self.factors.n[fmax] = 0;
    }

    pub fn inv(&mut self, n: PrimeType) -> Result<PrimeType, TatamiError> {
        self.isn = n;
        self.factors = Factors::new(FNUM);
        self.factors.p[0] = PR[0];
        self.factors.n[0] = 1;
        self.work(PR[0], PR[1..].iter());
        if self.smin < SMAX {
            Ok(self.smin)
        } else {
            Err(TatamiError::new("borked"))
        }
    }
}
