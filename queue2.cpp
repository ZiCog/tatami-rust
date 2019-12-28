
// queue.cpp -- Compute T(s) from Project Euler Problem 256
// Written December 7, 2019 by Eric Olson
// Translated to C++, 2019 by Jean M. Cyr

#if !defined(TDEBUG)
#define TDEBUG 0
#endif

#include <atomic>
#include <cmath>
#include <condition_variable>
#include <iostream>
#include <mutex>
#include <queue>
#include <thread>
#include <vector>
#if TDEBUG
#include <sstream>
#endif

using namespace std;

#if !defined(Ts)
#define Ts 200
#endif

#if Ts == 200
typedef uint32_t uintprime_t;
const uintprime_t sMax(100000000);
const uint32_t pNum(1300);
const uint32_t fNum(10);
#elif Ts == 1000
typedef uint64_t uintprime_t;
const uintprime_t sMax(100000000000ULL);
const uint32_t pNum(40000);
const uint32_t fNum(20);
#else
#error "Only Ts=200 and Ts=1000 are supported"
#endif

typedef struct
{
    uintprime_t s, p[fNum];
    uint32_t fmax, i;
    uint8_t n[fNum];
} Factors;

class Threads
{
public:
    Threads(uint32_t);
    void Enqueue(Factors f);
    ~Threads();

private:
    vector<thread> workers;
    queue<Factors> tasks;
    mutex tMutex;
    condition_variable condition;
    bool stop;
};

void TWork(Factors f);

inline Threads::Threads(uint32_t threads) : stop(false)
{
    for (uint32_t i = 0; i < threads; ++i)
        workers.emplace_back([this] {
            for (;;)
            {
                Factors f;
                {
                    unique_lock<mutex> lock(tMutex);
                    if (tasks.empty())
                    {
                        if (stop)
                            break;
                        else
                            condition.wait(lock);
                    }
                    f = tasks.front();
                    tasks.pop();
                }
                TWork(f);
            }
        });
}

void Threads::Enqueue(Factors f)
{
    {
        unique_lock<mutex> lock(tMutex);
        tasks.emplace(f);
    }
    condition.notify_one();
}

inline Threads::~Threads()
{
    {
        unique_lock<mutex> lock(tMutex);
        stop = true;
    }
    condition.notify_all();
    for (thread& worker : workers)
        worker.join();
}

uint32_t numPrimes, iLim;
uintprime_t P[pNum];
atomic<uintprime_t> gMin(sMax);
Threads* pool;

inline bool IsPrime(uintprime_t p)
{
    uint32_t i;
    for (i = 1; i < iLim; i++)
        if (!(p % P[i]))
            return false;
    for (i = iLim; P[i] * P[i] <= p; i++)
        if (!(p % P[i]))
            return false;
    iLim = i - 1;
    return true;
}

void Primes()
{
    uint32_t p;

    P[0] = 2;
    P[1] = 3;
    numPrimes = 2;
    iLim = 1;
    for (p = 5; numPrimes < pNum; p += 2)
        if (IsPrime(p))
            P[numPrimes++] = p;
#if TDEBUG
    uint32_t i;
    uintprime_t r;
    stringstream ss;
    if (p <= sMax / p + 1)
    {
        ss << "The maximum prime " << p << " is too small!";
        throw runtime_error(ss.str());
    }
    r = 1;
    for (i = 0; i < fNum - 1; i++)
    {
        if (P[i] > sMax / r + 1)
            return;
        r *= P[i];
    }
    ss << "Distinct Primes " << fNum << " in factorisation too few!";
    throw runtime_error(ss.str());
#endif
}

inline uintprime_t ppow(uintprime_t p, uint8_t n)
{
    uintprime_t r(1);
    for (; n; p *= p)
    {
        if (n & 1)
            r *= p;
        n >>= 1;
    }
    return r;
}

inline uintprime_t Sigma0Div2(Factors& f)
{
    uintprime_t r(f.n[0]);
    for (uint32_t i = 1; i <= f.fmax; i++)
        r *= f.n[i] + 1;
    return r;
}

inline bool TFree(uintprime_t k, uintprime_t l)
{
    uintprime_t n(l / k);
    uintprime_t lmin((k + 1) * n + 2);
    uintprime_t lmax((k - 1) * (n + 1) - 2);
    return lmin <= l && l <= lmax;
}

uint32_t T(Factors& f)
{
    uint8_t z[fNum] = {};
    uint32_t r(0);
    for (;;)
    {
        uint32_t i;
        for (i = 0; i <= f.fmax; i++)
        {
            if (z[i] < f.n[i])
            {
                z[i]++;
                break;
            }
            z[i] = 0;
        }
        if (i > f.fmax)
            break;
        uintprime_t k(1);
        uintprime_t l(1);
        for (i = 0; i <= f.fmax; i++)
        {
            k *= ppow(f.p[i], z[i]);
            l *= ppow(f.p[i], f.n[i] - z[i]);
        }
        if ((k <= l) && TFree(k, l))
            r++;
    }
    return r;
}

void TWork(Factors f)
{
    uintprime_t p(P[f.i]);
    uintprime_t sMin(gMin.load());
    uintprime_t s(f.s);
    uintprime_t pMax(sMin / s + 1);
    if (p <= pMax)
    {
        uint32_t fmax(f.fmax);
        f.n[fmax]++;
        f.s = s * p;
        if ((Sigma0Div2(f) >= Ts) && (T(f) == Ts))
            while ((f.s < sMin) && !(gMin.compare_exchange_weak(sMin, f.s)))
                ;
        TWork(f);
        f.s = s;
        f.n[fmax]--;
        if (f.i >= pNum - 1)
            return;
        f.i++;
        if (f.n[fmax])
            f.fmax++;
        f.p[f.fmax] = P[f.i];
        f.n[f.fmax] = 0;
        TWork(f);
        // f.fmax = fmax;
        // f.i--;
    }
}

void TQueue(Factors& f)
{
    uintprime_t sMin(gMin.load());
    uintprime_t s(f.s);
    uintprime_t pMax(sMin / s + 1);
    uintprime_t p(P[f.i]);
    if (p <= pMax)
    {
        uint32_t fmax(f.fmax);
        if ((pow(log(pMax), sqrt(2)) / log(p)) < 10)
        {
            pool->Enqueue(f);
            return;
        }
        f.n[fmax]++;
        f.s = s * p;
        if ((Sigma0Div2(f) >= Ts) && (T(f) == Ts))
            while ((f.s < sMin) && !(gMin.compare_exchange_weak(sMin, f.s)))
                ;
        TQueue(f);
        f.s = s;
        f.n[fmax]--;
        if (f.i >= pNum - 1)
            return;
        f.i++;
        if (f.n[fmax])
            f.fmax++;
        f.p[f.fmax] = P[f.i];
        f.n[f.fmax] = 0;
        TQueue(f);
        f.fmax = fmax;
        f.i--;
    }
}

int main()
{
    Primes();

    Factors f = {.s = 2, .p = {P[0]}, .fmax = 0, .i = 0, .n = {1}};
    pool = new Threads(thread::hardware_concurrency());
    TQueue(f);
    delete pool;
    cout << "T(" << gMin.load() << ")=" << Ts << endl;
    return 0;
}

