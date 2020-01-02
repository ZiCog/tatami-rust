/* Derived from ejolson's C OMP implementation */

#include <math.h>
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sysexits.h>
#include <unistd.h>

#define TDEBUG 0

#if !defined(T_S)
#define T_S 1000
#endif

#if T_S == 200
typedef uint32_t prime_t;

#define MAX_SQUARES 100000000
#define NUM_PRIMES 1300
#define NUM_FACTORS 10
#define MAX_FREE_FACTORS 4000
#define FIDO 15

#define P_FMT "u"

#elif T_S == 1000
typedef uint64_t prime_t;

#define MAX_SQUARES 100000000000
#define NUM_PRIMES 40000
#define NUM_FACTORS 20
#define MAX_FREE_FACTORS 40000
#define FIDO 15

#if _LP64
#define P_FMT "lu"
#else
#define P_FMT "llu"
#endif

#else
#error "T_S unsupported"
#endif

static uint32_t num_procs;

typedef struct factors_s
{
    struct factors_s* next;
    struct factors_s* prev;
    prime_t s, p[NUM_FACTORS];
    uint32_t fmax, i;
    uint8_t n[NUM_FACTORS];
} factors_t;

typedef struct
{
    factors_t* next;
    factors_t* prev;
    pthread_mutex_t mtx;
} queue_t;

static pthread_t* workers;
static uint8_t stop;

static queue_t task_q;
static pthread_cond_t cond;

static queue_t free_q;
static factors_t free_factors[MAX_FREE_FACTORS];

static inline void q_add_last(factors_t* f, queue_t* q)
{
    f->next = (void*)q;
    f->prev = q->prev;
    q->prev->next = f;
    q->prev = f;
}

static inline int q_empty(queue_t* q)
{
    return q->next == (void*)q;
}

static inline factors_t* q_remove_first(queue_t* q)
{
#if TDEBUG
    if (q_empty(q))
    {
        printf("queue empty\n");
        exit(EX_SOFTWARE);
    }
#endif
    factors_t* f = q->next;
    f->next->prev = (void*)q;
    q->next = f->next;
    return f;
}

static void q_init(queue_t* q)
{
    q->next = q->prev = (void*)q;
    pthread_mutex_init(&q->mtx, NULL);
}

static void free_q_init(void)
{
    q_init(&free_q);
    for (uint32_t i = 0; i < MAX_FREE_FACTORS; i++)
        q_add_last(free_factors + i, &free_q);
}

static void t_threaded_work(factors_t f);

static void* worker(void* arg)
{
    for (;;)
    {
        pthread_mutex_lock(&task_q.mtx);
        if (q_empty(&task_q))
        {
            if (stop)
                break;
            else
                pthread_cond_wait(&cond, &task_q.mtx);
        }
        factors_t* f = q_remove_first(&task_q);
        pthread_mutex_unlock(&task_q.mtx);
        t_threaded_work(*f);
        pthread_mutex_lock(&free_q.mtx);
        q_add_last(f, &free_q);
        pthread_mutex_unlock(&free_q.mtx);
    }
    pthread_mutex_unlock(&task_q.mtx);
    return NULL;
}

static void work_start(void)
{
    num_procs = sysconf(_SC_NPROCESSORS_ONLN);
    workers = (pthread_t*)malloc(sizeof(pthread_t) * num_procs);
    stop = 0;
    free_q_init();
    q_init(&task_q);
    pthread_cond_init(&cond, NULL);
    for (uint32_t i = 0; i < num_procs; i++)
        pthread_create(workers + i, NULL, worker, NULL);
}

static void work_end(void)
{
    pthread_mutex_lock(&task_q.mtx);
    stop = 1;
    pthread_mutex_unlock(&task_q.mtx);
    pthread_cond_broadcast(&cond);
    void* retval;
    for (uint32_t i = 0; i < num_procs; i++)
        pthread_join(workers[i], &retval);
}

static void work_enqueue(factors_t* f)
{
    pthread_mutex_lock(&task_q.mtx);
    q_add_last(f, &task_q);
    pthread_mutex_unlock(&task_q.mtx);
    pthread_cond_signal(&cond);
}

static prime_t P[NUM_PRIMES], gMin;
static uint32_t num_primes;

static uint32_t is_prime(prime_t p)
{
    uint32_t i;
    static uint32_t i_limit = 1;
    for (i = 1; i < i_limit; i++)
        if (!(p % P[i]))
            return 0;
    for (i = i_limit; P[i] * P[i] <= p; i++)
        if (!(p % P[i]))
            return 0;
    i_limit = i - 1;
    return 1;
}

static void calc_primes()
{
    prime_t p;

    P[0] = 2;
    P[1] = 3;
    num_primes = 2;
    for (p = 5; num_primes < NUM_PRIMES; p += 2)
        if (is_prime(p))
            P[num_primes++] = p;
#if TDEBUG
    prime_t i;
    prime_t r;
    if (p <= MAX_SQUARES / p + 1)
    {
        printf("The maximum prime %" P_FMT " is too small\n", p);
        exit(EX_SOFTWARE);
    }
    r = 1;
    for (i = 0; i < NUM_FACTORS - 1; i++)
    {
        if (P[i] > MAX_SQUARES / r + 1)
            return;
        r *= P[i];
    }
    printf("Distinct Primes %" P_FMT " in factorisation too few!",
        (prime_t)NUM_FACTORS);
    exit(EX_SOFTWARE);
#endif
}

static prime_t p_pow(prime_t p, uint8_t n)
{
    prime_t r = 1;
    for (; n; p *= p)
    {
        if (n & 1)
            r *= p;
        n >>= 1;
    }
    return r;
}

static prime_t sigma_0_div_2(factors_t* f)
{
    prime_t r = f->n[0];
    for (uint32_t i = 1; i <= f->fmax; i++)
        r *= f->n[i] + 1;
    return r;
}

static uint32_t t_free(prime_t k, prime_t l)
{
    prime_t n = l / k;
    prime_t lmin = (k + 1) * n + 2;
    prime_t lmax = (k - 1) * (n + 1) - 2;
    return lmin <= l && l <= lmax;
}

static uint32_t t(factors_t* f)
{
    uint8_t z[NUM_FACTORS];
    for (uint32_t i = 0; i < NUM_FACTORS; i++)
        z[i] = 0;
    uint32_t r = 0;
    for (;;)
    {
        uint32_t i;
        for (i = 0; i <= f->fmax; i++)
        {
            if (z[i] < f->n[i])
            {
                z[i]++;
                break;
            }
            z[i] = 0;
        }
        if (i > f->fmax)
            break;
        prime_t k = 1;
        for (i = 0; i <= f->fmax; i++)
        {
            k *= p_pow(f->p[i], z[i]);
        }
        prime_t l = f->s / k;
        if ((k <= l) && t_free(k, l))
            r++;
    }
    return r;
}

static void t_threaded_work(factors_t f)
{
    prime_t p = P[f.i];
    prime_t s = f.s;
    prime_t pMax = gMin / s + 1;
    if (p <= pMax)
    {
        uint32_t fmax = f.fmax;
        f.n[fmax]++;
        f.s = s * p;
        if ((sigma_0_div_2(&f) >= T_S && (t(&f) == T_S)))
            for (;;)
            {
                prime_t sMin = gMin;
                if (f.s >= sMin)
                    break;
                if (__atomic_compare_exchange_n(&gMin, &sMin, f.s, 0,
                        __ATOMIC_RELAXED, __ATOMIC_RELAXED))
                    break;
            }
        t_threaded_work(f);
        f.s = s;
        f.n[fmax]--;
        if (f.i >= NUM_PRIMES - 1)
            return;
        f.i++;
        if (f.n[fmax])
            f.fmax++;
        f.p[f.fmax] = P[f.i];
        f.n[f.fmax] = 0;
        t_threaded_work(f);
    }
}

static void t_work(factors_t f)
{
    prime_t s = f.s;
    prime_t pMax = gMin / s + 1;
    prime_t p = P[f.i];
    if (p <= pMax)
    {
        uint32_t fmax = f.fmax;
        if ((pow(log(pMax), sqrt(2)) / log(p)) < FIDO)
        {
            pthread_mutex_lock(&free_q.mtx);
            factors_t* factor = q_remove_first(&free_q);
            pthread_mutex_unlock(&free_q.mtx);
            *factor = f;
            work_enqueue(factor);
            return;
        }
        f.n[fmax]++;
        f.s = s * p;
        if ((sigma_0_div_2(&f) >= T_S && (t(&f) == T_S)))
            for (;;)
            {
                prime_t sMin = gMin;
                if (f.s >= sMin)
                    break;
                if (__atomic_compare_exchange_n(&gMin, &sMin, f.s, 0,
                        __ATOMIC_RELAXED, __ATOMIC_RELAXED))
                    break;
            }
        t_work(f);
        f.s = s;
        f.n[fmax]--;
        if (f.i >= NUM_PRIMES - 1)
            return;
        f.i++;
        if (f.n[fmax])
            f.fmax++;
        f.p[f.fmax] = P[f.i];
        f.n[f.fmax] = 0;
        t_work(f);
    }
}

int main()
{
    calc_primes();

    factors_t f;
    f.s = 2;
    f.p[0] = P[0];
    f.fmax = 0;
    f.i = 0;
    f.n[0] = 1;
    gMin = MAX_SQUARES;
    work_start();
    t_work(f);
    work_end();
    printf("T(%" P_FMT ")=%u\n", gMin, T_S);
    return EX_OK;
}
