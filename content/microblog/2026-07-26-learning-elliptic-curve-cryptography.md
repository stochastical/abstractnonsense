---
title: Learning Elliptic Curve Cryptography with Curve25519
tags: ['cryptography', 'study']
link: 'https://martin.kleppmann.com/papers/curve25519.pdf'
date: 2026-07-26
---

As of late, I've been trying to learn about cryptography more rigorously. I quite enjoyed [Real-World Cryptography](/microblog/2026-03-30-real-world-cryptography-by-david-wong) by David Wong, but there's a lot of group theory and low-level machinery to continue learning.

Martin Kleppmann has an exemplary tutorial on [Implementing Curve25519/X25519: A Tutorial on Elliptic Curve Cryptography](https://martin.kleppmann.com/papers/curve25519.pdf) that I worked through.

I think I mostly understood the theory as I worked through it; but here's some thoughts:
1. [`TweetNaCl`](https://tweetnacl.cr.yp.to) is really cool! I love the idea of tweet-sized algorithms for educative purposes.
2. There's a lot of different ways to implement finite-field arithmetic (efficiently) -- something I want to dig into more.
3. I don't feel comfortable with the [point at infinity](https://en.wikipedia.org/wiki/Point_at_infinity). I understand that we're augmenting the group with an identity element that satisfies criteria that are a pre-requisite for the elliptic curve structure to hold. But I feel like I need a more rigorous perspective to fully grok how it functions. I suspect that some projective geometry and an understanding of quotient spaces would help here.
4. I appreciated the usage of the distinct notation ($\bullet$) for the group operation, it helped disambiguate *field*-element arithmetic from applying the group operation on *group*-elements without parsing between the different syntactical conventions.

There is a *lot* of literature distilled down here. There is much to learn: from algorithms ([Pollard's rho algorithm](https://en.wikipedia.org/wiki/Pollard%27s_rho_algorithm), the [Montgomery ladder](https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication#Montgomery_ladder)) to theory ([Fermat's little theorem](https://en.wikipedia.org/wiki/Fermat%27s_little_theorem), security properties of groups) to various bit-twiddling hacks and subtleties for efficient finite-field arithmetic. The eventual goal is to have a better understanding of homomorphic encryption, but I think it's equally useful to learn the foundations. Plus, I've missed learning new maths - and I love how practical it is!

I transliterated the provided C implementation of the [Curve25519/X25519](https://en.wikipedia.org/wiki/Curve25519) elliptic curve into a pure-Rust implementation of the constant-time algorithm. It's a pretty faithful mapping of the C code, with some minor amendments. Notably, I've re-ordered the `out` buffer to come last in the function signatures, so watch out with your function calls! To be more idiomatic, you could `impl` methods on a `FieldElem` struct, if desired.

I've popped the code into a repo [`x25519-rs`](https://github.com/stochastical/x25519-rs), but since it's so wonderfully concise, I've reproduced it below too.

It should go without saying, but please don't use this for real-world applications. Note also that the code was handwritten (no LLMs were harmed in the making of).

```rust
use rand::{
    Rng, SeedableRng,
    rngs::{StdRng, SysRng},
};

type FieldElem = [i64; 16];

const _121665: FieldElem = {
    let mut c = [0; 16];
    [c[0], c[1]] = [0xDB41, 1];
    c
};

const _9: [u8; 32] = {
    let mut c = [0; 32];
    c[0] = 9;
    c
};

fn unpack25519(input: &[u8], out: &mut FieldElem) {
    for i in 0..16 {
        out[i] = input[2 * i] as i64 + ((input[2 * i + 1] as i64) << 8);
    }
    out[15] &= 0x7fff;
}

fn carry25519(elem: &mut FieldElem) {
    for i in 0..16 {
        let carry = elem[i] >> 16;
        elem[i] -= carry << 16;
        if i < 15 {
            elem[i + 1] += carry;
        } else {
            elem[0] += 38 * carry;
        }
    }
}

fn fadd(a: &FieldElem, b: &FieldElem, out: &mut FieldElem) {
    for i in 0..16 {
        out[i] = a[i] + b[i];
    }
}

fn fsub(a: &FieldElem, b: &FieldElem, out: &mut FieldElem) {
    for i in 0..16 {
        out[i] = a[i] - b[i];
    }
}

fn fmul(a: &FieldElem, b: &FieldElem, out: &mut FieldElem) {
    let mut product = [0; 31];

    for i in 0..16 {
        for j in 0..16 {
            product[i + j] += a[i] * b[j];
        }
    }
    for i in 0..15 {
        product[i] += 38 * product[i + 16];
    }

    for i in 0..16 {
        out[i] = product[i];
    }
    carry25519(out);
    carry25519(out);
}

fn finverse(input: &FieldElem, out: &mut FieldElem) {
    let mut c: FieldElem = *input;
    for i in (0..=253).rev() {
        fmul(&c.clone(), &c.clone(), &mut c);
        if i != 2 && i != 4 {
            fmul(&c.clone(), input, &mut c);
        }
    }
    *out = c;
}

fn swap25519(p: &mut FieldElem, q: &mut FieldElem, bit: i64) {
    let mut t;
    let c = !(bit - 1);
    for i in 0..16 {
        t = c & (p[i] ^ q[i]);
        p[i] ^= t;
        q[i] ^= t;
    }
}

fn pack25519(input: &FieldElem, out: &mut [u8; 32]) {
    let mut carry;
    let mut m: FieldElem = [0; 16];
    let mut t: FieldElem = *input;

    carry25519(&mut t);
    carry25519(&mut t);
    carry25519(&mut t);

    for _ in 0..2 {
        m[0] = t[0] - 0xffed;
        for i in 1..15 {
            m[i] = t[i] - 0xffff - ((m[i - 1] >> 16) & 1);
            m[i - 1] &= 0xffff;
        }
        m[15] = t[15] - 0x7fff - ((m[14] >> 16) & 1);
        carry = (m[15] >> 16) & 1;
        m[14] &= 0xffff;
        swap25519(&mut t, &mut m, 1 - carry);
    }
    for i in 0..16 {
        out[2 * i] = (t[i] & 0xff) as u8;
        out[2 * i + 1] = (t[i] >> 8) as u8;
    }
}

fn scalarmult(scalar: &[u8; 32], point: &[u8; 32], out: &mut [u8; 32]) {
    let mut x: FieldElem = [0; 16];

    let mut clamped = *scalar;
    clamped[0] &= 0xf8;
    clamped[31] = (clamped[31] & 0x7f) | 0x40;
    unpack25519(point, &mut x);
    
    let mut a: FieldElem = [0; 16];
    let mut b: FieldElem = x;
    let mut c: FieldElem = [0; 16];
    let mut d: FieldElem = [0; 16];
    let mut e: FieldElem = [0; 16];
    let mut f: FieldElem = [0; 16];
    (a[0], d[0]) = (1, 1);

    for i in (0..=254).rev() {
        let bit = ((clamped[i >> 3] >> (i & 7)) & 1) as i64;
        swap25519(&mut a, &mut b, bit);
        swap25519(&mut c, &mut d, bit);
        fadd(&a, &c, &mut e);
        fsub(&a.clone(), &c, &mut a);
        fadd(&b, &d, &mut c);
        fsub(&b.clone(), &d, &mut b);
        fmul(&e, &e, &mut d);
        fmul(&a, &a, &mut f);
        fmul(&c, &a.clone(), &mut a);
        fmul(&b, &e, &mut c);
        fadd(&a, &c, &mut e);
        fsub(&a.clone(), &c, &mut a);
        fmul(&a, &a, &mut b);
        fsub(&d, &f, &mut c);
        fmul(&c, &_121665, &mut a);
        fadd(&a.clone(), &d, &mut a);
        fmul(&c.clone(), &a, &mut c);
        fmul(&d, &f, &mut a);
        fmul(&b, &x, &mut d);
        fmul(&e, &e, &mut b);
        swap25519(&mut a, &mut b, bit);
        swap25519(&mut c, &mut d, bit);
    }
    finverse(&c.clone(), &mut c);
    fmul(&a.clone(), &c, &mut a);
    pack25519(&a, out);
}

fn scalarmult_base(scalar: &[u8; 32], out: &mut [u8; 32]) {
    scalarmult(scalar, &_9, out);
}

fn generate_keypair(pk: &mut [u8; 32], sk: &mut [u8; 32]) {
    let mut rng = StdRng::try_from_rng(&mut SysRng).unwrap();
    rng.fill_bytes(sk);
    scalarmult_base(sk, pk);
}

fn x25519(pk: &[u8; 32], sk: &[u8; 32], out: &mut [u8; 32]) {
    scalarmult(sk, pk, out);
}

/// This test case comes from https://cr.yp.to/highspeed/naclcrypto-20090310.pdf
#[test]
fn vec1() {
    #[rustfmt::skip]
    const SK: [u8; 32] = [
        0x77, 0x07, 0x6d, 0x0a, 0x73, 0x18, 0xa5, 0x7d,
        0x3c, 0x16, 0xc1, 0x72, 0x51, 0xb2, 0x66, 0x45,
        0xdf, 0x4c, 0x2f, 0x87, 0xeb, 0xc0, 0x99, 0x2a,
        0xb1, 0x77, 0xfb, 0xa5, 0x1d, 0xb9, 0x2c, 0x2a,
    ];

    #[rustfmt::skip]
    const PK: [u8; 32] = [
        0x85, 0x20, 0xf0, 0x09, 0x89, 0x30, 0xa7, 0x54,
        0x74, 0x8b, 0x7d, 0xdc, 0xb4, 0x3e, 0xf7, 0x5a,
        0x0d, 0xbf, 0x3a, 0x0d, 0x26, 0x38, 0x1a, 0xf4,
        0xeb, 0xa4, 0xa9, 0x8e, 0xaa, 0x9b, 0x4e, 0x6a,
    ];

    let mut out: [u8; 32] = [0; 32];
    scalarmult_base(&SK, &mut out);
    assert_eq!(out, PK);
}

/// This test case comes from https://cr.yp.to/highspeed/naclcrypto-20090310.pdf
#[test]
fn vec2() {
    #[rustfmt::skip]
    const SK: [u8; 32] = [
        0x5d, 0xab, 0x08, 0x7e, 0x62, 0x4a, 0x8a, 0x4b,
        0x79, 0xe1, 0x7f, 0x8b, 0x83, 0x80, 0x0e, 0xe6, 
        0x6f, 0x3b, 0xb1, 0x29, 0x26, 0x18, 0xb6, 0xfd,
        0x1c, 0x2f, 0x8b, 0x27, 0xff, 0x88, 0xe0, 0xeb,
    ];

    #[rustfmt::skip]
    const PK: [u8; 32]  = [
        0xde, 0x9e, 0xdb, 0x7d, 0x7b, 0x7d, 0xc1, 0xb4,
        0xd3, 0x5b, 0x61, 0xc2, 0xec, 0xe4, 0x35, 0x37,
        0x3f, 0x83, 0x43, 0xc8, 0x5b, 0x78, 0x67, 0x4d,
        0xad, 0xfc, 0x7e, 0x14, 0x6f, 0x88, 0x2b, 0x4f,
    ];

    let mut out: [u8; 32] = [0; 32];
    scalarmult_base(&SK, &mut out);
    assert_eq!(out, PK);
}
```
