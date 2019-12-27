// queue.rs -- Compute T(s) from Project Euler Problem 256

// Written December 7, 2019 by Eric Olson
// Translated to C++, 2019 by Jean M. Cyr
// Translated to Rust. 25th Dec 2019 by Heater.

#![allow(bad_style)] 

use std::sync::atomic::{AtomicPtr, Ordering};

const pNum: usize = 1300;
const fifteen: f64 = 15.0;
const sqrtOf2: f64 =  std::f64::consts::SQRT_2;

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
        Factors {
            s: 2,
            fmax: 0,
            i: 0,
            p: [PR[0]; FNUM],
            n: [1; FNUM],
        }
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

fn T(xp: &Factors) -> u32 {
    let mut z: Vec<u8> = vec![0; FNUM];
    let mut r: u32 = 0;
    loop {
        let mut k: u32;
        let mut l: u32;
        for (i, z) in z.iter_mut().enumerate().take(xp.fmax + 1) {
            if *z < xp.n[i] {
                *z += 1;
                break;
            }
            *z = 0;
        }
        // FIXME:
        //        if i > xp.fmax {
        //            break;
        //        }
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
// This built-in function implements an atomic compare and exchange operation.
// This compares the contents of *ptr with the contents of *expected and...
//     if equal, writes desired into *ptr.
//     if they are not equal, the current contents of *ptr is written into *expected.
// True is returned if desired is written into *ptr
// False is returned otherwise,
// https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html#method.compare_exchange
// https://gcc.gnu.org/onlinedocs/gcc-4.9.2/gcc/_005f_005fatomic-Builtins.html
fn __atomic_compare_exchange_n(ptr: &mut u32, expected: &mut u32, mut desired: u32) -> bool {
    let some_ptr = AtomicPtr::new(ptr);
    let value =
        some_ptr.compare_exchange(expected, &mut desired, Ordering::Relaxed, Ordering::Relaxed);
    match value {
        Ok(_expected) => true,
        Err(_expected) => false,
    }
}

fn Twork(mut xp: &mut Factors, Tisn: u32, gMin: &mut u32) {
    let fmax = xp.fmax;
    let mut smin: u32 = *gMin;
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

fn Tqueue(mut xp: &mut Factors, Tisn: u32, gMin: &mut u32, mut pool: &mut rayon::ThreadPool) {
    let fmax = xp.fmax;
    let mut smin: u32 = *gMin;
    let s: u32 = xp.s;
    let pMax: u32 = smin / s + 1;
    let p: u32 = PR[xp.i];
    if p <= pMax {
        let mut r: u32;
        if (pow(log(pMax.into()), sqrtOf2) / log(p.into())) < fifteen {
            let mut yp: Factors = *xp;
            let mut g = *gMin;
            pool.spawn_fifo(move || {
                Twork(&mut yp, Tisn, &mut g);
            });
        
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
        r = sigma(&xp);
        if r >= Tisn {
            r = T(&xp);
            if r == Tisn {
                while xp.s < smin {
                    if __atomic_compare_exchange_n(gMin, &mut smin, xp.s) {
                        break;
                    }
                    // THREAD STUFF
                    // if __atomic_compare_exchange_n(&gMin, &smin, xp.s, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED) {
                    // break;
                    //}
                }
            }
        }
        Tqueue(&mut xp, Tisn, gMin, &mut pool);
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
        Tqueue(&mut xp, Tisn, gMin, &mut pool);
        xp.fmax = fmax;
        xp.i -= 1;
    }
}

pub fn Tinv(n: u32) -> u32 {
    let mut x = Factors::new();

    let mut gMin: u32 = SMAX;

    let ptr = &mut gMin;

    // See: https://docs.rs/rayon/1.3.0/rayon/struct.ThreadPoolBuilder.html
    let mut pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();



    /* THREAD STUFF
        pool = new Threads(thread::hardware_concurrency());
    */
    Tqueue(&mut x, n, ptr, &mut pool);
    /* THREAD STUFF
        delete pool;
    */
    gMin
}
