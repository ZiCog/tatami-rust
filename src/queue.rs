// queue.rs -- Compute T(s) from Project Euler Problem 256

// Written December 7, 2019 by Eric Olson
// Translated to C++, 2019 by Jean M. Cyr


const sMax: u32 = 100_000_000;
const pNum: u32 = 1300;
const fNum: usize = 10;
const fifteen: f32 = 15.0;
const sqrtOf2: f32 = 1.4142135623730951;


pub struct Factors {
    s: u32,
    fmax: usize,
    i: u32,
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

fn tfree(k: u32, l: u32) -> bool
{
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

fn sigma(xp: &Factors) -> u32
{
    let r: u32 = xp.n[0].into();
    for i in 1..=xp.fmax {
        r *= xp.n[i] as u32 + 1;
    }
    return r;
}

fn T(xp: &Factors) -> u32
{
    let z: Vec<u8> = vec![0; fNum];
    let r: u32 = 0;
    loop {
        let k: u32;
        let l: u32;
        for  i in 0 ..= xp.fmax {
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
        for i in 0 ..= xp.fmax {
            k *= ppow(xp.p[i], z[i]);
            l *= ppow(xp.p[i], xp.n[i] - z[i]);
        }
        if k <= l {
            if tfree(k, l) {
                r += 1;
            }
        }
    }
    r
}

fn Twork(xp: &Factors) {
    let fmax = xp.fmax;
    let smin: u32 = gMin;
    let s: u32;
    let pMax: u32;
    let p: u32;
    s = xp.s;
    pMax = smin / s + 1;
    p = P[xp.i];
    if p <= pMax {
        let r: u32;
        xp.n[fmax] += 1;
        xp.s = s * p;
        r = sigma(xp);
        if r >= Tisn {
            r = T(xp);
            if r == Tisn {
                while xp.s < smin {
                    if __atomic_compare_exchange_n(&gMin, &smin, xp.s, 0,
                            __ATOMIC_RELAXED, __ATOMIC_RELAXED) {
                        break;
                    }
                }
            }
        }
        Twork(xp);
        xp.s = s;
        xp.n[fmax] -= 1;
        if xp.i >= pNum - 1 {
            return;
        }
        xp.i += 1;
        if xp.n[fmax] != 0 {
            xp.fmax += 1;
        }
        xp.p[xp.fmax] = P[xp.i];
        xp.n[xp.fmax] = 0;
        Twork(xp);
        xp.fmax = fmax;
        xp.i -= 1;
    }
}

fn Tqueue(xp: &Factors) {
    let fmax = xp.fmax;
    let smin: u32 = gMin;
    let s: u32 = xp.s;
    let pMax: u32 = smin / s + 1;
    let p: u32 = P[xp.i];
    if p <= pMax {
        let r: u32;
        if (pow(log(pMax), sqrtOf2) / log(p)) < fifteen {
            let yp: Factors = *xp;
            Lc += 1;
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
                    if __atomic_compare_exchange_n(&gMin, &smin, xp.s, 0,
                            __ATOMIC_RELAXED, __ATOMIC_RELAXED) {
                        break;
                    }
                }
            }
        }
        Tqueue(xp);
        xp.s = s;
        xp.n[fmax] -= 1;
        if xp.i >= pNum - 1 {
            return;
        }
        xp.i += 1;
        if xp.n[fmax] != 0 {
            xp.fmax += 1;
        }
        xp.p[xp.fmax] = P[xp.i];
        xp.n[xp.fmax] = 0;
        Tqueue(xp);
        xp.fmax = fmax;
        xp.i -= 1;
    }
}

pub fn Tinv(n: u32) -> u32 {
    let x: Factors;
/* THREAD STUFF    
    pool = new Threads(thread::hardware_concurrency());
*/
    Tisn = n;
    x.p[0] = P[0];
    x.n[0] = 1;
    x.i = 0;
    x.s = 2;
    x.fmax = 0;
    Tqueue(&x);
/* THREAD STUFF    
    delete pool;
*/
    return gMin;
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