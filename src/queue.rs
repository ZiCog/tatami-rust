// queue.rs -- Compute T(s) from Project Euler Problem 256

// Written December 7, 2019 by Eric Olson
// Translated to C++, 2019 by Jean M. Cyr

use std::sync::atomic::{AtomicPtr, Ordering};

const sMax: u32 = 100_000_000;
const pNum: usize = 1300;
const fNum: usize = 10;
const fifteen: f64 = 15.0;
const sqrtOf2: f64 = 1.414_213_562_373_095_1;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub struct Factors {
    s: u32,
    fmax: usize,
    i: usize,
    p: Vec<u32>,
    n: Vec<u8>,
}

impl Factors {
    fn new(size: usize) -> Factors {
        Factors {
            s: 0,
            fmax: 0,
            i: 0,
            p: vec![0; size],
            n: vec![0; size],
        }
    }
}

/* Globals !!!
u32 Tisn;
u32 P[pNum];
u32 gMin;
*/

fn tfree(k: u32, l: u32) -> bool {
    let n: u32 = l / k;
    let lmin: u32 = (k + 1) * n + 2;
    let lmax: u32 = (k - 1) * (n + 1) - 2;
    lmin <= l && l <= lmax
}

/* Globals !!!
void doinit()
{
    u32 i;
    u32 p, r;
    gMin = sMax;
    P[0] = 2;
    P[1] = 3;
    iLim = 1;
}
*/

fn sigma(xp: &Factors) -> u32 {
    let r: u32 = xp.n[0].into();
    for i in 1..=xp.fmax {
        r *= xp.n[i] as u32 + 1;
    }
    r
}

fn T(xp: &Factors) -> u32 {
    let z: Vec<u8> = vec![0; fNum];
    let r: u32 = 0;
    loop {
        let k: u32;
        let l: u32;
        for i in 0..=xp.fmax {
            if z[i] < xp.n[i] {
                z[i] += 1;
                break;
            }
            z[i] = 0;
        }
        if i > xp.fmax {
            break;
        }
        k = 1;
        l = 1;
        for i in 0..=xp.fmax {
            k *= xp.p[i].pow(z[i] as u32);
            l *= xp.p[i].pow(xp.n[i] as u32 - z[i] as u32);
        }
        if k <= l && tfree(k, l) {
            r += 1;
        }
    }
    r
}

// From GCC atomic built-in.
// This built-in function implements an atomic compare and exchange operation.
// This compares the contents of *ptr with the contents of *expected and...
//     if equal, writes desired into *ptr.
//     if they are not equal, the current contents of *ptr is written into *expected.
// True is returned if desired is written into *ptr
// False is returned otherwise,
fn __atomic_compare_exchange_n(ptr: &mut u32, expected: &mut u32, desired: u32) -> bool {
    let some_ptr = AtomicPtr::new(ptr);
    let value =
        some_ptr.compare_exchange(expected, &mut desired, Ordering::Relaxed, Ordering::Relaxed);
    match value {
        Ok(expected) => true,
        Err(expected) => false,
    }
}

fn Twork(xp: &Factors, Tisn: u32, gMin: &mut u32) {
    let fmax = xp.fmax;
    let smin: u32 = *gMin;
    let s: u32;
    let pMax: u32;
    let p: u32;
    s = xp.s;
    pMax = smin / s + 1;
    p = PR[xp.i];
    if p <= pMax {
        let r: u32;
        xp.n[fmax] += 1;
        xp.s = s * p;
        r = sigma(xp);
        if r >= Tisn {
            r = T(xp);
            if r == Tisn {
                while xp.s < smin {
                    if __atomic_compare_exchange_n(gMin, &mut smin, xp.s) {
                        break;
                    }

                    // THREAD STUFF
                    // if __atomic_compare_exchange_n(&gMin, &smin, xp.s, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED) {
                    //    break;
                    // }
                }
            }
        }
        Twork(xp, Tisn, gMin);
        xp.s = s;
        xp.n[fmax] -= 1;
        if xp.i >= pNum - 1 {
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

fn Tqueue(xp: &Factors, Tisn: u32, gMin: &mut u32) {
    let fmax = xp.fmax;
    let smin: u32 = *gMin;
    let s: u32 = xp.s;
    let pMax: u32 = smin / s + 1;
    let p: u32 = PR[xp.i];
    if p <= pMax {
        let r: u32;
        if (pow(log(pMax.into()), sqrtOf2) / log(p.into())) < fifteen {
            let yp: Factors = *xp;
            /* THREAD STUFF
                        pool->enqueue([yp] {
                            Twork(*yp);
                            delete yp;
                        });
            */
            return;
        }
        xp.n[fmax] += 1;
        xp.s = s * p;
        r = sigma(xp);
        if r >= Tisn {
            r = T(xp);
            if r == Tisn {
                while xp.s < smin {
                    if __atomic_compare_exchange_n(gMin, &mut smin, xp.s) {
                        break;
                    }
                    // THREAD STUFF
                    // if __atomic_compare_exchange_n(&gMin, &smin, xp.s, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED) {
                    break;
                    //}
                }
            }
        }
        Tqueue(xp, Tisn, gMin);
        xp.s = s;
        xp.n[fmax] -= 1;
        if xp.i >= pNum - 1 {
            return;
        }
        xp.i += 1;
        if xp.n[fmax] != 0 {
            xp.fmax += 1;
        }
        xp.p[xp.fmax] = PR[xp.i];
        xp.n[xp.fmax] = 0;
        Tqueue(xp, Tisn, gMin);
        xp.fmax = fmax;
        xp.i -= 1;
    }
}

pub fn Tinv(n: u32) -> u32 {
    let x: Factors;

    let gMin: u32 = sMax;

    let ptr = &mut gMin;

    /* THREAD STUFF
        pool = new Threads(thread::hardware_concurrency());
    */
    x.p[0] = PR[0];
    x.n[0] = 1;
    x.i = 0;
    x.s = 2;
    x.fmax = 0;
    Tqueue(&x, n, ptr);
    /* THREAD STUFF
        delete pool;
    */
    gMin
}

/*
int main()
{
    u32 n = 200;
    doinit();
    cout << "T(" << Tinv(n) << ")=" << n << "\n";
    return 0;
}
*/
