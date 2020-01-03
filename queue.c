/*  queue.c -- Compute T(s) from Project Euler Problem 256
    Written December 7, 2019 by Eric Olson in K&R C for PDP-11

    Less copying, more multiplication less division.
    Avoid computing T(s) when s doesn't have enough factors.
    More sensible computation of prime table at startup.
    Don't check larger values of s than already found.
    OpenMP version with atomic, powerdog and a Twrap.  */

#include <stdio.h>

#ifdef _NFILE
# define void int
#else
# include <stdlib.h>
# include <unistd.h>
# include <math.h>
# include <sys/time.h>
# include <sys/resource.h>
#endif

#ifdef _OPENMP
# include <omp.h>
# define do_pragma(Z) _Pragma(#Z)
# define spawn(Y) do_pragma(omp task default(shared) Y)
# define forall _Pragma("omp taskloop default(shared)") for
# define sync _Pragma("omp taskwait")
# define get_ncpu(X) omp_get_num_procs(X)
# define workers(X) \
    omp_set_dynamic(0); \
    if(X) omp_set_num_threads(X); \
    else omp_set_num_threads(1); \
    _Pragma("omp parallel default(shared)") \
    _Pragma("omp single")
# define load(X) __atomic_load_n(&X,__ATOMIC_RELAXED)
# define cmpstore(X,Y,Z) __atomic_compare_exchange_n(&X,&Y,Z, \
    0,__ATOMIC_RELAXED,__ATOMIC_RELAXED)
#else
# define spawn(Y)
# define forall for
# define sync
# define get_ncpu(X) 1
# define workers(X)
# define load(X) (X)
# define cmpstore(X,Y,Z) (X==Y?(X=Z,1):(Y=X,0))
#endif


#define smax 100000000000l
#define Pnum 40000
#define fnum 20
/*
#define smax 100000000l
#define Pnum 1300
#define fnum 10
*/
#define fido 15.0
#define sqr2 1.4142135623730951

typedef struct { long s; int fmax,i; long p[fnum]; char n[fnum]; } factors;

static int Pn,Tisn,ilim;
static long P[Pnum],gmin;
static int nwork,Lc;

static int tfree(k,l) long k,l; {
    long n=l/k;
    long lmin=(k+1)*n+2;
    long lmax=(k-1)*(n+1)-2;
    return lmin<=l && l<=lmax;
}
static long isprime(p) long p;{
    int i;
    for(i=1;i<ilim;i++){
        if(!(p%P[i])) return 0;
    }
    for(i=ilim;P[i]*P[i]<=p;i++){
        if(!(p%P[i])) return 0;
    }
    ilim=i-1;
    return 1;
}
static void doinit(){
    int i;
    long p,r;
    gmin=smax;
    P[0]=2; P[1]=3; Pn=2, ilim=1;
    for(p=5;Pn<Pnum;p+=2){
        if(isprime(p)) P[Pn++]=p;
    }
    if(p<=smax/p+1){
        printf("The maximum prime %ld is too small!\n",p);
        exit(1);
    }
    r=1;
    for(i=0;i<fnum-1;i++) {
        if(P[i]>smax/r+1) return;
        r*=P[i];
    }
    printf("Distinct primes %d in factorisation too few!\n",fnum);
    exit(2);
}
static long ppow(p,n) long p; char n; {
    long r;
    if(!n) return 1;
    r=1;
    for(;;){
        if(n&1) r*=p;
        n>>=1;
        if(!n) return r;
        p*=p;
    }
}
static long sigma(xp) factors *xp; {
    int i;
    long r=xp->n[0];
    for(i=1;i<=xp->fmax;i++) r*=xp->n[i]+1;
    return r;        
}
static long T(xp) factors *xp; {
    char z[fnum];
    int r,w;
    for(w=0;w<fnum;w++) z[w]=0;
    r=0;
    for(;;){
        int i;
        long k,l;
        for(i=0;i<=xp->fmax;i++){
            if(z[i]<xp->n[i]){
                z[i]++; break;
            }
            z[i]=0;
        }
        if(i>xp->fmax) break;
        k=1; l=1;
        for(i=0;i<=xp->fmax;i++){
            k*=ppow(xp->p[i],z[i]);
            l*=ppow(xp->p[i],xp->n[i]-z[i]);
        }
        if(k<=l) r+=tfree(k,l);
    }
    return r;
}
static void Twork(xp) factors *xp; {
    int fmax;
    long smin=load(gmin);
    long s,pmax,p;
    fmax=xp->fmax;
    s=xp->s;
    pmax=smin/s+1;
    p=P[xp->i];
    if(p<=pmax){
        int r;
        xp->n[fmax]++; xp->s=s*p;
        r=sigma(xp);
        if(r>=Tisn){
            r=T(xp);
            if(r==Tisn){
                while(xp->s<smin){
                    if(cmpstore(gmin,smin,xp->s)) break;
                }
            }
        }
        Twork(xp);
        xp->s=s; xp->n[fmax]--;
        if(xp->i>=Pnum-1) return;
        xp->i++; if(xp->n[fmax]) xp->fmax++;
        xp->p[xp->fmax]=P[xp->i];
        xp->n[xp->fmax]=0;
        Twork(xp);
        xp->fmax=fmax; xp->i--; 
    }
    return;
}
static void Twrap(xp) factors *xp; {
    Twork(xp);
    free(xp);
}
static double powerdog(x,p) double x,p; {
    return pow(log(x),sqr2)/log(p);
}
static void Tqueue(xp) factors *xp; {
    int fmax;
    long smin=load(gmin);
    long s,pmax,p;
    fmax=xp->fmax;
    s=xp->s;
    pmax=smin/s+1;
    p=P[xp->i];
    if(p<=pmax){
        int r;
        if(powerdog((double)pmax,(double)p)<fido){
            factors *yp=malloc(sizeof(factors));
            *yp=(*xp);
            Lc++;
            spawn(firstprivate(yp)) Twrap(yp);
            return;
        }
        xp->n[fmax]++; xp->s=s*p;
        r=sigma(xp);
        if(r>=Tisn){
            r=T(xp);
            if(r==Tisn){
                while(xp->s<smin){
                    if(cmpstore(gmin,smin,xp->s)) break;
                }
            }
        }
        Tqueue(xp);
        xp->s=s; xp->n[fmax]--;
        if(xp->i>=Pnum-1) return;
        xp->i++; if(xp->n[fmax]) xp->fmax++;
        xp->p[xp->fmax]=P[xp->i];
        xp->n[xp->fmax]=0;
        Tqueue(xp);
        xp->fmax=fmax; xp->i--; 
    }
}
static long Tinv(n) int n; {
    factors x;
    Tisn=n;
    x.p[0]=P[0]; x.n[0]=1; x.i=0; x.s=2; x.fmax=0;
    Tqueue(&x);
    sync;
    return gmin<smax?gmin:-1;
}
int main(){
    //int n=200;
    int n=1000;
    nwork=2*get_ncpu();
    printf("Using %d threads.\n", nwork);
    doinit();
    printf("Pr(%d)=%ld\n",Pnum,P[Pnum-1]);
    workers(nwork){
        printf("T(%ld)=%d\n",Tinv(n),n);
    }
    printf("Lc=%d\n",Lc);
    return 0;
}
