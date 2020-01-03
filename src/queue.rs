// queue.rs -- Compute t(s) from Project Euler Problem 256

// Written December 7, 2019 by Eric Olson
// Translated to C++, 2019 by Jean M. Cyr
// Translated to Rust. 25th Dec 2019 by Heater.

use std::sync::atomic::Ordering;

use rayon::Scope;

const FIFTEEN: f64 = 15.0;
const SQRT_OF2: f64 = std::f64::consts::SQRT_2;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

#[derive(Debug, Clone, Copy)]
pub struct Factors {
    s: PrimeType,
    fmax: usize,
    i: usize,
    p: [PrimeType; FNUM],
    n: [u8; FNUM],
}

impl Factors {
    fn new() -> Factors {
        let mut x = Factors {
            s: 2,
            fmax: 0,
            i: 0,
            p: [0; FNUM],
            n: [0; FNUM],
        };
        x.p[0] = PR[0];
        x.n[0] = 1;
        x
    }
}

fn tfree(k: PrimeType, l: PrimeType) -> bool {
    let n: PrimeType = l / k;
    let lmin: PrimeType = (k + 1) * n + 2;
    let lmax: PrimeType = (k - 1) * (n + 1) - 2;
    lmin <= l && l <= lmax
}

fn sigma(xp: &Factors) -> u32 {
    let mut r: u32 = xp.n[0].into();
    for i in 1..=xp.fmax {
        r *= xp.n[i] as u32 + 1;
    }
    r
}

fn t(xp: &mut Factors) -> u32 {
    let mut z: Vec<u8> = vec![0; FNUM];
    let mut r: u32 = 0;
    loop {
        let mut k: PrimeType;
        let l: PrimeType;
        let mut found: bool = false;
        for (i, z) in z.iter_mut().enumerate().take(xp.fmax + 1) {
            if *z < xp.n[i] {
                *z += 1;
                found = true;
                break;
            }
            *z = 0;
        }
        if !found {
            break;
        }

        k = 1;
        for (i, z) in z.iter().enumerate().take(xp.fmax + 1) {
            k *= xp.p[i].pow(*z as u32);
        }
        l = xp.s / k;
        if k <= l && tfree(k, l) {
            r += 1;
        }
    }
    r
}

fn twork<'scope>(xp: &mut Factors, tisn: u32, g_min: &'scope AtomicType) {
    let fmax = xp.fmax;
    let mut smin = g_min.load(Ordering::Relaxed);
    let s = xp.s;
    let p_max = smin / s + 1;
    let p = PR[xp.i];
    if p <= p_max {
        xp.n[fmax] += 1;
        xp.s = s * p;
        if sigma(xp) >= tisn && t(xp) == tisn {
            while xp.s < smin {
                g_min.compare_and_swap(smin, xp.s, Ordering::Relaxed);
                smin = g_min.load(Ordering::Relaxed);
            }
        }
        twork(xp, tisn, g_min);
        xp.s = s;
        xp.n[fmax] -= 1;
        if xp.i >= PR.len() - 1 {
            return;
        }
        xp.i += 1;
        if xp.n[fmax] != 0 {
            xp.fmax += 1;
        }
        xp.p[xp.fmax] = PR[xp.i];
        xp.n[xp.fmax] = 0;
        twork(xp, tisn, g_min);
        xp.fmax = fmax;
        xp.i -= 1;
    }
}

// From C std lib: The log function computes the value of the natural logarithm of argument x.
fn log(x: f64) -> f64 {
    x.ln()
}

// From C std lib: Returns base raised to the power exponent.
fn pow(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

fn tqueue<'scope>(xp: &mut Factors, tisn: u32, g_min: &'scope AtomicType, scope: &Scope<'scope>) {
    let fmax = xp.fmax;
    let mut smin = g_min.load(Ordering::Relaxed);
    let s = xp.s;
    let p_max = smin / s + 1;
    let p = PR[xp.i];
    if p <= p_max {
        let mut r: u32;
        if (pow(log(p_max as f64), SQRT_OF2) / log(p as f64)) < FIFTEEN {
            let mut yp: Factors = *xp;

            scope.spawn(move |_scope| {
                twork(&mut yp, tisn, g_min);
            });

            return;
        }
        xp.n[fmax] += 1;
        xp.s = s * p;
        if sigma(xp) >= tisn && t(xp) == tisn {
            while xp.s < smin {
                g_min.compare_and_swap(smin, xp.s, Ordering::Relaxed);
                smin = g_min.load(Ordering::Relaxed);
            }
        }
        tqueue(xp, tisn, g_min, scope);
        xp.s = s;
        xp.n[fmax] -= 1;
        if xp.i >= PR.len() - 1 {
            return;
        }
        xp.i += 1;
        if xp.n[fmax] != 0 {
            xp.fmax += 1;
        }
        xp.p[xp.fmax] = PR[xp.i];
        xp.n[xp.fmax] = 0;
        tqueue(xp, tisn, g_min, scope);
        xp.fmax = fmax;
        xp.i -= 1;
    }
}

pub fn tinv(n: u32) -> PrimeType {
    let mut x = Factors::new();
    let g_min = AtomicType::new(SMAX);

    let cpus = num_cpus::get();
    println!("Uing {} cores.", cpus);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(cpus)
        .stack_size(4 * 1000_000)
        .build()
        .unwrap();

    pool.scope(|scope| {
        tqueue(&mut x, n, &g_min, scope);
    });

    g_min.into_inner()
}
