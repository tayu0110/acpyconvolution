# for https://judge.yosupo.jp/problem/convolution_mod
from acpyconvolution import convolution, M998244353

_ = input()
a = list(map(int, input().split()))
b = list(map(int, input().split()))

c = convolution(a, b, M998244353)

print(*c, sep=" ")
