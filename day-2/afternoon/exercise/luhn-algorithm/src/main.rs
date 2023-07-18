//! The Luhn algorithm is used to validate credit card numbers. The algorithm
//! takes a string as input and does the following to validate the credit card
//! numbers:
//!  - Ignore all spaces. Reject number with less than two digits.
//!  - Moving from right to left, double evey second digit.
//!     example: for the number 1234, we double 3 and 1.
//!  - After doubling a digit, sum the digits. So doubling 7 becomes 14 which
//!    becomes 5.
//!  - Sum all the undoubled and doubled digits.
//!  - The credit card number is valid if the sum ends with 0.

#![allow(unused_variables, dead_code)]

pub fn luhn_mine(cc_number: &str) -> bool {
    // unimplemented!()
    let mut sum = 0;

    // reject number will less than two digits
    if cc_number.trim().len() <= 2 {
        return false;
    }

    // moving from right to left
    let mut idx = 1;
    for c in cc_number.chars().rev() {
        // ignore space
        if c == ' ' {
            continue;
        }

        // not a number
        if (c as i32) < ('0' as i32) || (c as i32) > ('9' as i32) {
            return false;
        }

        let mut num: i32 = (c as i32) - ('0' as i32);

        // double every second digit (check for even, because the first is 1)
        if idx % 2 == 0 {
            num = num * 2;

            // sum the digit if more than 10
            if num >= 10 {
                num = (num - 10) + 1
            }
        }

        idx += 1;
        sum += num;
    }

    // sum ends with 0
    sum % 10 == 0
}

pub fn luhn_other(cc_number: &str) -> bool {
    let mut digits_seen = 0;
    let mut sum = 0;
    for (i, ch) in cc_number.chars().rev().filter(|&ch| ch != ' ').enumerate() {
        match ch.to_digit(10) {
            Some(d) => {
                sum += if i % 2 == 1 {
                    let dd = d * 2;
                    dd / 10 + dd % 10
                } else {
                    d
                };
                digits_seen += 1;
            }
            None => return false,
        }
    }

    if digits_seen < 2 {
        return false;
    }

    sum % 10 == 0
}

pub fn luhn(cc_number: &str) -> bool {
    // luhn_other(cc_number)
    luhn_mine(cc_number)
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    luhn("4263 9826 4026 9299");
}
