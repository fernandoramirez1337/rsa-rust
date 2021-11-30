#![allow(clippy::many_single_char_names)]

extern crate num;
extern crate num_bigint as bigint;
extern crate primal;
extern crate rand;
extern crate rustc_serialize;

use bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num::{Integer, One, Zero};

fn small_primes(bound: usize) -> Vec<usize> {
    primal::Primes::all().take(bound).collect::<Vec<usize>>()
}

pub fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    let mut result = One::one();
    let mut b = base.to_owned();
    let mut exp = exponent.to_owned();

    while exp > Zero::zero() {
        if (&exp & 1.to_biguint().unwrap()) == One::one() {
            result *= &b;
            result %= modulus;
        }
        b = &b * &b;
        b = &b % modulus;

        exp >>= 1;
    }
    result
}

fn rewrite(n: &BigUint) -> (BigUint, BigUint) {
    let mut d = n.clone();
    let mut s: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two = 2.to_biguint().unwrap();

    while d.is_even() {
        d /= &two;
        s += &one;
    }
    (s, d)
}

fn rabin_miller(candidate: &BigUint) -> bool {
    const K: usize = 128usize;

    let one: BigUint = One::one();
    let two = 2.to_biguint().unwrap();
    let three = 3.to_biguint().unwrap();

    if candidate == &two {
        return true;
    }
    if candidate == &three {
        return true;
    }
    if candidate.is_even() {
        return false;
    }

    let (mut s, d) = rewrite(&(candidate - &one));
    'witness_loop: for _ in 0..K {
        let mut rng = rand::thread_rng();
        let basis = rng.gen_biguint_range(&two, &(candidate - &one));
        let mut x = mod_exp(&basis, &d, candidate);

        if x == one || x == (candidate - &one) {
            continue 'witness_loop;
        }

        while s > one {

            x = (&x * &x) % candidate;
            if x == one {

                return false;
            }

            if x == candidate - &one {
                continue 'witness_loop;
            }
            s -= &one;
        }

        return false;
    }

    true
}

pub fn is_prime(candidate: &BigUint) -> bool {
    for p in small_primes(100).iter() {
        let bigp = p.to_biguint().unwrap();
        if *candidate == bigp {
            return true;
        } else if bigp.divides(candidate) {
            return false;
        }
    }
    rabin_miller(candidate)
}

pub fn big_prime(bitsize: usize) -> BigUint {
    let one: BigUint = One::one();
    let two = 2.to_biguint().unwrap();

    let mut rng = rand::thread_rng();
    let mut candidate = rng.gen_biguint(bitsize);
    if candidate.is_even() {
        candidate = &candidate + &one;
    }
    while !is_prime(&candidate) {
        candidate = &candidate + &two;
    }
    candidate
}

pub fn rsa_prime(size: usize, e: &BigUint) -> BigUint {
    loop {
        let p = big_prime(size);
        if &p % e != One::one() {
            return p;
        }
    }
}

pub fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigInt, BigInt, BigInt) {

    let (mut s, mut old_s, mut t, mut old_t): (BigInt, BigInt, BigInt, BigInt) =
        (Zero::zero(), One::one(), One::one(), Zero::zero());

    let (mut r, mut old_r) = (b.to_bigint().unwrap(), a.to_bigint().unwrap());

    while r != Zero::zero() {
        let quotient = &old_r / &r;

        let mut tmp = &old_r - &quotient * &r;
        old_r = r;
        r = tmp;

        tmp = &old_s - &quotient * &s;
        old_s = s;
        s = tmp;

        tmp = &old_t - &quotient * &t;
        old_t = t;
        t = tmp;

    }
    let gcd = old_r;
    (gcd, s, t)
}

pub fn invmod(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    let (mut t, mut new_t): (BigInt, BigInt) = (Zero::zero(), One::one());

    let (mut r, mut new_r) = (n.to_bigint().unwrap(), a.to_bigint().unwrap());

    while new_r != Zero::zero() {
        let quotient = &r / &new_r;

        let mut tmp = &t - &quotient * &new_t;
        t = new_t;
        new_t = tmp;

        tmp = &r - &quotient * &new_r;
        r = new_r;
        new_r = tmp;
    }
    if r > One::one() {
        return None;
    };
    if t < Zero::zero() {
        t = &t + &n.to_bigint().unwrap()
    };

    Some(t.to_biguint().unwrap())
}