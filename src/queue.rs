// queue.rs -- Compute T(s) from Project Euler Problem 256

// Written December 7, 2019 by Eric Olson
// Translated to C++, 2019 by Jean M. Cyr
// Translated to Rust. 25th Dec 2019 by Heater.

// We want to keep orignal C names for now. 
#![allow(bad_style)]

use std::sync::atomic::{AtomicU32, Ordering};

use rayon::Scope;

const fifteen: f64 = 15.0;
const sqrtOf2: f64 = std::f64::consts::SQRT_2;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

#[derive(Debug, Clone, Copy)]
pub struct Factors {
    s: u32,
    fmax: usize,
    i: usize,
    p: [u32; FNUM],
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

fn tfree(k: u32, l: u32) -> bool {
    let n: u32 = l / k;
    let lmin: u32 = (k + 1) * n + 2;
    let lmax: u32 = (k - 1) * (n + 1) - 2;
    lmin <= l && l <= lmax
}

fn sigma(xp: &Factors) -> u32 {
    let mut r: u32 = xp.n[0].into();
    for i in 1..=xp.fmax {
        r *= xp.n[i] as u32 + 1;
    }
    r
}

fn T(xp: &mut Factors) -> u32 {
    let mut z: Vec<u8> = vec![0; FNUM];
    let mut r: u32 = 0;
    'outer: loop {
        let mut k: u32;
        let mut l: u32;
        for (i, z) in z.iter_mut().enumerate().take(xp.fmax + 1) {
            if *z < xp.n[i] {
                *z += 1;
                if i > xp.fmax {
                    break 'outer; // FIXME: Check this loop carefullly.!
                }
            }
            *z = 0;
        }
        k = 1;
        l = 1;
        for (i, z) in z.iter().enumerate().take(xp.fmax + 1) {
            k *= xp.p[i].pow(*z as u32);
            l *= xp.p[i].pow(xp.n[i] as u32 - *z as u32);
        }
        if k <= l && tfree(k, l) {
            r += 1;
        }
    }
    r
}

// From GCC atomic built-in.
// bool __atomic_compare_exchange_n (type *ptr, type *expected, type desired, bool weak, int success_memmodel, int failure_memmodel)
// This built-in function implements an atomic compare and exchange operation.
// This compares the contents of *ptr with the contents of *expected and...
//     if equal, writes desired into *ptr.
//     if they are not equal, the current contents of *ptr is written into *expected.
// True is returned if desired is written into *ptr
// False is returned otherwise,
// https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicU32.html
// https://gcc.gnu.org/onlinedocs/gcc-4.9.2/gcc/_005f_005fatomic-Builtins.html
fn Twork<'scope>(xp: &mut Factors, Tisn: u32, gMin: &'scope AtomicU32) {
    let fmax = xp.fmax;
    let mut smin: u32 = gMin.load(Ordering::Relaxed);
    let s: u32;
    let pMax: u32;
    let p: u32;
    s = xp.s;
    pMax = smin / s + 1;
    p = PR[xp.i];
    if p <= pMax {
        let mut r: u32;
        xp.n[fmax] += 1;
        xp.s = s * p;
        r = sigma(xp);
        if r >= Tisn {
            r = T(xp);
            if r == Tisn {
                //while (xp.s < smin) {
                //    __atomic_compare_exchange_n(&gMin, &smin, xp.s, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED);
                //}

                while xp.s < smin {
                    smin = gMin.swap(xp.s, Ordering::Relaxed);
                }
            }
        }
        Twork(xp, Tisn, gMin);
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
        Twork(xp, Tisn, gMin);
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

fn Tqueue<'scope>(xp: &mut Factors, Tisn: u32, gMin: &'scope AtomicU32, scope: &Scope<'scope>) {
    let fmax = xp.fmax;
    let mut smin: u32 = gMin.load(Ordering::Relaxed);
    let s: u32 = xp.s;
    let pMax: u32 = smin / s + 1;
    let p: u32 = PR[xp.i];
    if p <= pMax {
        let mut r: u32;
        if (pow(log(pMax.into()), sqrtOf2) / log(p.into())) < fifteen {
            let mut yp: Factors = *xp;

            scope.spawn(move |_scope| {
                Twork(&mut yp, Tisn, gMin);
            });            

            return;
        }
        xp.n[fmax] += 1;
        xp.s = s * p;
        r = sigma(xp);
        if r >= Tisn {
            r = T(xp);
            if r == Tisn {
                //while (xp.s < smin) {
                //    __atomic_compare_exchange_n(&gMin, &smin, xp.s, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED);
                //}

                while xp.s < smin {
                    smin = gMin.swap(xp.s, Ordering::Relaxed);
                }
            }
        }
        Tqueue(xp, Tisn, gMin, scope);
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
        Tqueue(xp, Tisn, gMin, scope);
        xp.fmax = fmax;
        xp.i -= 1;
    }
}

pub fn Tinv(n: u32) -> u32 {
    let mut x = Factors::new();
    let gMin = AtomicU32::new(SMAX);

    // Using rayon scope. See suggestion by alice here:
    // https://users.rust-lang.org/t/yes-at-last-my-rust-is-faster-than-c/36100/21
    rayon::scope(|scope| {
        Tqueue(&mut x, n, &gMin, scope);
    });
    
    gMin.into_inner()
}
