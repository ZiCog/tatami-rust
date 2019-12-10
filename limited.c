/*  limited.c -- Compute T(s) from Project Euler Problem 256
    Written November 9, 2019 by Eric Olson in K&R C for PDP-11

    Less copying, more multiplication less division.
    Avoid computing T(s) when s doesn't have enough factors.  */

#include <stdio.h>
#include <stdlib.h>

/*
#define smax 100000000l
#define Pnum 1300
*/
#define smax 100000000l
#define Pnum 1300
#define fnum 10

typedef struct { long s; int fmax,i; long p[fnum]; char n[fnum]; } factors;

static factors x;
static int Pn,Tisn;
static long P[Pnum],smin;
static char z[fnum];

//#define void int

static int tfree(k,l) long k,l; {
    long n=l/k;
    long lmin=(k+1)*n+2;
    long lmax=(k-1)*(n+1)-2;
    return lmin<=l && l<=lmax;
}
static long isprime(p) long p;{
    int i;
    for(i=0;i<Pn;i++){
        if(!(p%P[i])) return 0;
    }
    return 1;
}
static void doinit(){
    int i;
    long p,r;
    smin=smax+2;
    P[0]=2; Pn=1;
    for(p=3;Pn<Pnum;p++){
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
static long sigma(){
    int i;
    long r=x.n[0];
    for(i=1;i<=x.fmax;i++) r*=x.n[i]+1;
    return r;        
}
static long T(){
    int r,w;
    for(w=0;w<fnum;w++) z[w]=0;
    r=0;
    for(;;){
        int i;
        long k,l;
        for(i=0;i<=x.fmax;i++){
            if(z[i]<x.n[i]){
                z[i]++; break;
            }
            z[i]=0;
        }
        if(i>x.fmax) break;
        k=1; l=1;
        for(i=0;i<=x.fmax;i++){
            k*=ppow(x.p[i],z[i]);
            l*=ppow(x.p[i],x.n[i]-z[i]);
        }
        if(k<=l) r+=tfree(k,l);
    }
    return r;
}
static void Twork(){
    int i,r;
    long s,fmax,pmax,p;
    s=x.s;
    r=sigma();
    if(r>=Tisn){
        r=T();
        if(r==Tisn&&s<smin) smin=s;
    }
    i=x.i;
    fmax=x.fmax;
    pmax=smax/s+1;
    p=P[i];
    if(p<=pmax){
        x.n[fmax]++; x.s=s*p;
        Twork();
        x.n[fmax]--;
    }
    fmax++;
    x.n[fmax]=1;
    for(i++;i<Pnum;i++){
        p=P[i];
        if(p>pmax) break;
        x.p[fmax]=p; x.s=s*p;
        x.i=i; x.fmax=fmax;
        Twork();
    }
    x.n[fmax]=0;
}
static long Tinv(n) int n; {
    Tisn=n;
    x.p[0]=P[0]; x.n[0]=1; x.i=0; x.s=2; x.fmax=0;
    Twork();
    return smin<smax?smin:-1;
}
int main(){
    int n=200;
    doinit();
    printf("T(%ld)=%d\n",Tinv(n),n);
    return 0;
}