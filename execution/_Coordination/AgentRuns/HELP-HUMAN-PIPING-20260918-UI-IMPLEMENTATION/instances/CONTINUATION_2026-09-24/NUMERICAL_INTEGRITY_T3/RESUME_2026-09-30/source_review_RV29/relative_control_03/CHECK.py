from fractions import Fraction as Q
import json

def pow2(e):
    return Q(1 << e) if e >= 0 else Q(1, 1 << -e)

def decode(bits):
    exponent = (bits >> 52) & 2047
    fraction = bits & ((1 << 52) - 1)
    assert bits >> 63 == 0 and exponent != 2047
    return (fraction if exponent == 0 else (1 << 52) | fraction) * pow2(-1074 if exponent == 0 else exponent - 1075)

def rn(q):
    assert q > 0
    e = q.numerator.bit_length() - q.denominator.bit_length()
    if q < pow2(e):
        e -= 1
    quantum = max(e - 52, -1074)
    scaled = q / pow2(quantum)
    integer, remainder = divmod(scaled.numerator, scaled.denominator)
    if 2 * remainder > scaled.denominator or (2 * remainder == scaled.denominator and integer % 2):
        integer += 1
    if integer < 1 << 52:
        return integer
    if integer == 1 << 53:
        integer >>= 1
        quantum += 1
    return ((quantum + 52 + 1023) << 52) | (integer - (1 << 52))

x = decode(0x3ff0000000000401)
h = pow2(-1074)
H = x * (pow2(-64) + pow2(-85) + pow2(-53))
a0 = decode(rn(x * pow2(-64)))
a1 = decode(rn(a0 * (1 + pow2(-21))))
u0 = decode(rn(x * pow2(-53)))
u1 = decode(rn(u0 + h))
a2 = decode(rn(a1 + u1))
nearest = rn(H)
up = nearest + (decode(nearest) < H)
exact_allowance = x * pow2(-64) * (1 + pow2(-21)) + x * pow2(-53) + h
assert H.numerator == 19352257851307810204156929 and H.denominator == 1 << 137
assert H.numerator.bit_length() == 85
assert decode(up - 1) < H < exact_allowance < a2 == decode(up)
assert up == nearest == 0x3ca0020000100402
assert 10**9 * H <= x
assert x >= pow2(-988) and x >= decode(rn(x * pow2(-34)))
# A consistent displacement magnitude companion at the same x, with W_plus=0,
# adds only its required x*2^(1-256) term and has a different positive radius.
companion = x * pow2(-255)
assert decode(0x3000000000000401) == companion
assert companion < exact_allowance and 10**9 * companion < x
print(json.dumps({"five_step_bits": [f"{rn(v):016x}" for v in [a0, a1, u0, u1, a2]], "H_significant_bits": H.numerator.bit_length(), "RN64_H_bits": f"{nearest:016x}", "RU64_H_bits": f"{up:016x}", "ordering": "prev < H < H+h < A_f64 = RU64(H)", "relative_and_public": "pass", "optional_consistent_magnitude_radius_bits": "3000000000000401", "production_execution": False}, indent=2))
