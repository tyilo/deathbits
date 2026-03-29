from typing import Iterator


def get_all(exp_bits: int, frac_size: int) -> Iterator[int]:
    total_bits = exp_bits + frac_size
    for i in range(0, 2**total_bits):
        a = i & ((1 << frac_size) - 1)
        e = i >> frac_size
        # RSolve[{a[0] == 0, a[n + 1] == (2^f + a[n]) / 2}, a[n], n] // InputForm
        # 2^(f - n)*(-1 + 2^n)
        # bias = 2**frac_size * (2**e - 1)
        # v = bias + 2**e * a
        v = 2**e * (a + 2**frac_size) - 2**frac_size
        print(f"{i:08b}: {e=} {a=} -> {v}")
        yield v


prev_diff = 0
prev = 0
for v in get_all(4, 4):
    diff = v - prev
    assert (
        (prev_diff == 0 and diff == 1) or diff == prev_diff or diff == 2 * prev_diff
    ), (prev, v, prev_diff, diff)

    prev_diff = diff
    prev = v
