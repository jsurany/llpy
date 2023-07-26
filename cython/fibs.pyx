# distutils: language=c++

from libcpp.vector cimport vector

def fib(unsigned int n):
    # allocate the memory
    cdef vector[int] f
    f.reserve(n)

    if n == 0:
        return f
    f.push_back(0)
    if n == 1:
        return f
    f.push_back(1)
    if n == 2:
        return f
    for i in range(2, n):
        f.push_back(f[i-2] + f[i-1])
    return f
