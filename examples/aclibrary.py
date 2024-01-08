# for https://judge.yosupo.jp/problem/convolution_mod
from atcoder.convolution import convolution

_ = input()
a = list(map(int, input().split()))
b = list(map(int, input().split()))

c = convolution(998244353, a, b)

print(*c, sep=" ")
