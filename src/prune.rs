use crate::constants::{FNUM, PNUM, SMAX};

include!(concat!(env!("OUT_DIR"), "/prime.rs"));

pub struct Factors {
    s: i64,
    fmax: usize,
    i: usize,
    p: Vec<i64>,
    n: Vec<i64>,
}

impl Factors {
    fn new(size: usize) -> Factors {
        Factors {
            s: 2,
            fmax: 0,
            i: 0,
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
}

impl Tatami {
    pub fn new() -> Tatami {
        Tatami {
            isn: 0,
            factors: Factors::new(FNUM),
            smin: SMAX,
            z: vec![0; FNUM],
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
        for i in 1..=self.factors.fmax {
            r *= self.factors.n[i] + 1;
        }
        r
    }

    fn t(&mut self) -> i64 {
        let mut r = 0;
        loop {
            let mut found: bool = false;
            for i in 0..=self.factors.fmax {
                if self.z[i] < self.factors.n[i] {
                    self.z[i] += 1;
                    found = true;
                    break;
                }
                self.z[i] = 0;
            }
            if !found {
                break;
            }
            let mut k = 1;
            let mut l = 1;
/*            
            for i in 0..=self.factors.fmax {
                k *= self.factors.p[i].pow(self.z[i] as u32);
                l *= self.factors.p[i].pow(self.factors.n[i] as u32 - self.z[i] as u32);
            }
*/
            let fmax = self.factors.fmax;
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
        let s = self.factors.s;
        let mut r = self.sigma();
        if r >= self.isn {
            r = self.t();
            if (r == self.isn) && (s < self.smin) {
                self.smin = s;
            }
        }
        let i = self.factors.i;
        let mut fmax = self.factors.fmax;
        let pmax = self.smin / s + 1;
        let mut p = PR[i];
        if p <= pmax {
            self.factors.n[fmax] += 1;
            self.factors.s = s * p;
            self.work();
            self.factors.n[fmax] -= 1;
        }
        fmax += 1;
        self.factors.n[fmax] = 1;
        for j in i + 1..PNUM {
            p = PR[j];
            if p > pmax {
                break;
            }
            self.factors.p[fmax] = p;
            self.factors.s = s * p;
            self.factors.i = j;
            self.factors.fmax = fmax;
            self.work();
        }
        self.factors.n[fmax] = 0;
    }

    pub fn inv(&mut self, n: i64) -> i64 {
        self.isn = n;
        self.factors = Factors::new(FNUM);
        self.factors.p[0] = PR[0];
        self.factors.n[0] = 1;
        self.factors.i = 0;
        self.factors.s = 2;
        self.factors.fmax = 0;
        self.work();
        if self.smin < SMAX {
            self.smin
        } else {
            -1
        }
    }
}
