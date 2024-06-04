use p22::calc::celsius2farenheit;
use p22::calc::farenheit2celsius;
use p22::calc::fibonacci_loop;
use p22::calc::fibonacci_rec;

#[test]
fn integration_test_celsius2farehnheit() {
    assert_eq!(celsius2farenheit(15), 59);
    assert_eq!(celsius2farenheit(50), 122);
    assert_eq!(celsius2farenheit(-200), -328);
}

#[test]
fn integration_test_farenheit2celsius() {
    assert_eq!(farenheit2celsius(167), 75);
    assert_eq!(farenheit2celsius(-49), -45);
    assert_eq!(farenheit2celsius(-4), -20);
}

#[test]
fn integration_test_fibonacci_loop() {
    assert_eq!(fibonacci_loop(0), 0);
    assert_eq!(fibonacci_loop(3), 2);
    assert_eq!(fibonacci_loop(14), 377);
}

#[test]
fn integration_test_fibonacci_rec() {
    assert_eq!(fibonacci_rec(1), 1);
    assert_eq!(fibonacci_rec(2), 1);
    assert_eq!(fibonacci_rec(15), 610);
}
