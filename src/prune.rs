use crate::constants::{FNUM, PNUM, SMAX};

include!(concat!(env!("OUT_DIR"), "/prime.rs"));

pub struct Factors {
    p: Vec<i64>,
    n: Vec<i64>,
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
    isn: i64,
    factors: Factors,
    smin: i64,
    z: Vec<i64>,
    s: i64,
    i: usize,
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
            i: 0,
            fmax: 0,
        }
    }

    fn free(&mut self, k: i64, l: i64) -> bool {
        let n: i64 = l / k;
        let lmin: i64 = (k + 1) * n + 2;
        let lmax: i64 = (k - 1) * (n + 1) - 2;
        (lmin <= l) && (l <= lmax)
    }

    fn sigma(&mut self) -> i64 {
        let mut r = self.factors.n[0];
        let fmax = self.fmax;
        for n in self.factors.n[1..=fmax].iter_mut() {
            r *= *n + 1;
        }
        r
    }

    fn t(&mut self) -> i64 {
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
                r += self.free(k, l) as i64;
            }
        }
        r
    }

    fn work(&mut self) {
        let s = self.s;
        let mut r = self.sigma();
        if r >= self.isn {
            r = self.t();
            if (r == self.isn) && (s < self.smin) {
                self.smin = s;
            }
        }
        let i = self.i;
        let mut fmax = self.fmax;
        let pmax = self.smin / s + 1;
        let mut p = PR[i];
        if p <= pmax {
            self.factors.n[fmax] += 1;
            self.s = s * p;
            self.work();
            self.factors.n[fmax] -= 1;
        }
        fmax += 1;
        self.factors.n[fmax] = 1;

        // Clippy said do this, which is not only ugly but much slower.
        // for (j, p) in PR.iter().enumerate().take(PNUM).skip(i + 1) {
        // for (j, p) in PR.iter().enumerate().skip(i + 1) {
        for j in i + 1..PNUM {
            p = PR[j];
            if p > pmax {
                break;
            }
            self.factors.p[fmax] = p;
            self.s = s * p;
            self.i = j;
            self.fmax = fmax;
            self.work();
        }
        self.factors.n[fmax] = 0;
    }

    pub fn inv(&mut self, n: i64) -> i64 {
        self.isn = n;
        self.factors = Factors::new(FNUM);
        self.factors.p[0] = PR[0];
        self.factors.n[0] = 1;
        self.work();
        if self.smin < SMAX {
            self.smin
        } else {
            -1
        }
    }
}
