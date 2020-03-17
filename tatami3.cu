// tatami.cu

#include <cuda.h>
#include

const unsigned nMax(100000000);
const unsigned nMaxSqrt(sqrt(nMax));

global void odd(unsigned* v, unsigned base)
{
unsigned i = (blockIdx.x * blockDim.x + threadIdx.x + base) * 2 + 7;
unsigned k2 = i + 3;
unsigned k3 = i + i - 4;
while ((k2 <= k3) && ((i * k2) < nMax))
{
unsigned k4 = (nMax - 1) / i;
if (k3 < k4)
k4 = k3;
__syncthreads();
for (unsigned j = k2 / 2; j <= k4 / 2; j++)
atomicAdd(&v[i * j], 1);
__syncthreads();
k2 += i + 1;
k3 += i - 1;
}
__syncthreads();
}

global void even(unsigned* v, unsigned base)
{
unsigned i = (blockIdx.x * blockDim.x + threadIdx.x + base) * 2 + 8;
unsigned k2 = i + 3;
unsigned k3 = i + i - 4;
while ((k2 <= k3) && ((i * k2) < nMax))
{
unsigned k4 = (nMax - 1) / i;
if (k3 < k4)
k4 = k3;
__syncthreads();
for (unsigned j = k2; j <= k4; ++j)
atomicAdd(&v[i * j / 2], 1);
__syncthreads();
k2 += i + 1;
k3 += i - 1;
}
__syncthreads();
}

int Tatami(int s)
{
unsigned* v;

cudaMalloc(&v, sizeof(unsigned) * nMax);
cudaMemset(v, 0, sizeof(unsigned) * nMax);
const unsigned group_size = 1024;
{
    // for (int i = 8; i < nMaxSqrt; i += 2)
    const unsigned iterations = (nMaxSqrt - 8) / 2;
    const unsigned groups = iterations / group_size;
    const unsigned trailing_group_size = iterations - group_size * groups;
    even<<<groups, group_size>>>(v, 0);
    if (trailing_group_size)
        even<<<1, trailing_group_size>>>(v, groups * group_size);
}
{
    // for (int i = 7; i < nMaxSqrt; i += 2)
    const unsigned iterations = (nMaxSqrt - 7) / 2;
    const unsigned groups = iterations / group_size;
    const unsigned trailing_group_size = iterations - group_size * groups;
    odd<<<groups, group_size>>>(v, 0);
    if (trailing_group_size)
        odd<<<1, trailing_group_size>>>(v, groups * group_size);
}

unsigned* vh = (unsigned*)malloc(sizeof(unsigned) * nMax);
cudaMemcpy(vh, v, sizeof(unsigned) * nMax, cudaMemcpyDeviceToHost);
cudaDeviceSynchronize();

for (unsigned i = 0; i < nMax; ++i)
    if (vh[i] == s)
        return i + i;
return 0;  // shouldn't happen
}

int main()
{
int s = 200;
std::cout << "T(" << Tatami(s) << ")=" << s << std::endl;
}
