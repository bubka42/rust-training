#![feature(test)]
extern crate test;

use p22::calc::celsius2farenheit;
use p22::calc::farenheit2celsius;
use p22::calc::fibonacci_loop;
use p22::calc::fibonacci_rec;
use test::{black_box, Bencher};

#[bench]
fn bench_celsius2farenheit(b: &mut Bencher) {
    b.iter(|| {
        let x = black_box(-40);
        let y = celsius2farenheit(x);
        black_box(y);
    })
}

#[bench]
fn bench_farenheit2celsius(b: &mut Bencher) {
    b.iter(|| {
        let x = black_box(-40);
        let y = farenheit2celsius(x);
        black_box(y);
    })
}

#[bench]
fn bench_fibonacci_loop(b: &mut Bencher) {
    b.iter(|| {
        let x = black_box(15);
        let y = fibonacci_loop(x);
        black_box(y);
    })
}

#[bench]
fn bench_fibonacci_rec(b: &mut Bencher) {
    b.iter(|| {
        let x = black_box(15);
        let y = fibonacci_rec(x);
        black_box(y);
    })
}
