// queue.cpp -- Compute T(s) from Project Euler Problem 256
// Written December 7, 2019 by Eric Olson
// Translated to C++, 2019 by Jean M. Cyr

#include <condition_variable>
#include <functional>
#include <future>
#include <iostream>
#include <mutex>
#include <queue>
#include <sstream>
#include <thread>
#include <vector>

#include <math.h>

using namespace std;

const uint32_t sMax = 100000000UL;
const uint32_t pNum = 1300;
const uint32_t fNum = 10;
const float fifteen = 15.0;
const float sqrtOf2 = 1.4142135623730951;

typedef struct
{
    uint32_t s, fmax, i, p[fNum];
    uint8_t n[fNum];
} factors;

uint32_t Pn, Tisn, iLim;
uint32_t P[pNum];
uint32_t gMin;
uint32_t nThreads, Lc;

class Threads
{
public:
    Threads(size_t);
    template <class F, class... Args>
    auto enqueue(F&& f, Args&&... args)
        -> future<typename result_of<F(Args...)>::type>;
    ~Threads();

private:
    vector<thread> workers;
    queue<function<void()>> tasks;

    mutex queue_mutex;
    condition_variable condition;
    bool stop;
};

Threads* pool;

inline Threads::Threads(size_t threads) : stop(false)
{
    for (size_t i = 0; i < threads; ++i)
        workers.emplace_back([this] {
            for (;;)
            {
                function<void()> task;
                {
                    unique_lock<mutex> lock(this->queue_mutex);
                    this->condition.wait(lock,
                        [this] { return this->stop || !this->tasks.empty(); });
                    if (this->stop && this->tasks.empty())
                        return;
                    task = move(this->tasks.front());
                    this->tasks.pop();
                }
                task();
            }
        });
}

template <class F, class... Args>
auto Threads::enqueue(F&& f, Args&&... args)
    -> future<typename result_of<F(Args...)>::type>
{
    using return_type = typename result_of<F(Args...)>::type;
    auto task = make_shared<packaged_task<return_type()> >(
        bind(forward<F>(f), forward<Args>(args)...));
    future<return_type> res = task->get_future();
    {
        unique_lock<mutex> lock(queue_mutex);
        if (stop)
            throw runtime_error("enqueue on stopped Threads");
        tasks.emplace([task]() { (*task)(); });
    }
    condition.notify_one();
    return res;
}

inline Threads::~Threads()
{
    {
        unique_lock<mutex> lock(queue_mutex);
        stop = true;
    }
    condition.notify_all();
    for (thread& worker : workers)
        worker.join();
}

bool tfree(uint32_t k, uint32_t l)
{
    uint32_t n = l / k;
    uint32_t lmin = (k + 1) * n + 2;
    uint32_t lmax = (k - 1) * (n + 1) - 2;
    return lmin <= l && l <= lmax;
}

uint32_t isprime(uint32_t p)
{
    uint32_t i;
    for (i = 1; i < iLim; i++)
        if (!(p % P[i]))
            return 0;
    for (i = iLim; P[i] * P[i] <= p; i++)
        if (!(p % P[i]))
            return 0;
    iLim = i - 1;
    return 1;
}

void doinit()
{
    uint32_t i;
    uint32_t p, r;
    gMin = sMax;
    P[0] = 2;
    P[1] = 3;
    Pn = 2;
    iLim = 1;
    for (p = 5; Pn < pNum; p += 2)
        if (isprime(p))
            P[Pn++] = p;
    if (p <= sMax / p + 1)
    {
        stringstream ss;
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
    stringstream ss;
    ss << "Distinct primes " << fNum << " in factorisation too few!";
    throw runtime_error(ss.str());
}

uint32_t ppow(uint32_t p, uint8_t n)
{
    uint32_t r;
    if (!n)
        return 1;
    r = 1;
    for (;;)
    {
        if (n & 1)
            r *= p;
        n >>= 1;
        if (!n)
            return r;
        p *= p;
    }
}

uint32_t sigma(factors& xp)
{
    uint32_t i;
    uint32_t r = xp.n[0];
    for (i = 1; i <= xp.fmax; i++)
        r *= xp.n[i] + 1;
    return r;
}

uint32_t T(factors& xp)
{
    uint8_t z[fNum];
    uint32_t r, w;
    for (w = 0; w < fNum; w++)
        z[w] = 0;
    r = 0;
    for (;;)
    {
        uint32_t i;
        uint32_t k, l;
        for (i = 0; i <= xp.fmax; i++)
        {
            if (z[i] < xp.n[i])
            {
                z[i]++;
                break;
            }
            z[i] = 0;
        }
        if (i > xp.fmax)
            break;
        k = 1;
        l = 1;
        for (i = 0; i <= xp.fmax; i++)
        {
            k *= ppow(xp.p[i], z[i]);
            l *= ppow(xp.p[i], xp.n[i] - z[i]);
        }
        if (k <= l)
            if (tfree(k, l))
                r++;
    }
    return r;
}

void Twork(factors& xp)
{
    uint32_t fmax;
    uint32_t smin = gMin;
    uint32_t s, pMax, p;
    fmax = xp.fmax;
    s = xp.s;
    pMax = smin / s + 1;
    p = P[xp.i];
    if (p <= pMax)
    {
        uint32_t r;
        xp.n[fmax]++;
        xp.s = s * p;
        r = sigma(xp);
        if (r >= Tisn)
        {
            r = T(xp);
            if (r == Tisn) {
                while (xp.s < smin) {
                    __atomic_compare_exchange_n(&gMin, &smin, xp.s, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED);
                }
            }
        }
        Twork(xp);
        xp.s = s;
        xp.n[fmax]--;
        if (xp.i >= pNum - 1)
            return;
        xp.i++;
        if (xp.n[fmax])
            xp.fmax++;
        xp.p[xp.fmax] = P[xp.i];
        xp.n[xp.fmax] = 0;
        Twork(xp);
        xp.fmax = fmax;
        xp.i--;
    }
}

void Tqueue(factors& xp)
{
    uint32_t fmax;
    uint32_t smin = gMin;
    uint32_t s, pMax, p;
    fmax = xp.fmax;
    s = xp.s;
    pMax = smin / s + 1;
    p = P[xp.i];
    if (p <= pMax)
    {
        uint32_t r;
        if ((pow(log(pMax), sqrtOf2) / log(p)) < fifteen)
        {
            factors* yp = new factors;
            *yp = xp;
            Lc++;
            pool->enqueue([yp] {
                Twork(*yp);
                delete yp;
            });
            return;
        }
        xp.n[fmax]++;
        xp.s = s * p;
        r = sigma(xp);
        if (r >= Tisn)
        {
            r = T(xp);
            if (r == Tisn) {
                while (xp.s < smin) {
                    __atomic_compare_exchange_n(&gMin, &smin, xp.s, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED);
                }
            }
        }
        Tqueue(xp);
        xp.s = s;
        xp.n[fmax]--;
        if (xp.i >= pNum - 1)
            return;
        xp.i++;
        if (xp.n[fmax])
            xp.fmax++;
        xp.p[xp.fmax] = P[xp.i];
        xp.n[xp.fmax] = 0;
        Tqueue(xp);
        xp.fmax = fmax;
        xp.i--;
    }
}

uint32_t Tinv(uint32_t n)
{
    factors x;
    pool = new Threads(thread::hardware_concurrency());
    Tisn = n;
    x.p[0] = P[0];
    x.n[0] = 1;
    x.i = 0;
    x.s = 2;
    x.fmax = 0;
    Tqueue(x);
    delete pool;
    return gMin;
}

int main()
{
    uint32_t n = 200;
    doinit();
    cout << "T(" << Tinv(n) << ")=" << n << "\n";
    return 0;
}
