pub mod hash;
pub mod array;
pub mod slice;
pub mod collections;
pub mod convert;
pub mod option;
pub mod string;
pub mod cmp;
pub mod ops;
pub mod default;
pub mod prelude;
pub mod runtime;
pub mod meta;
pub mod append;
pub mod mem;
pub mod panic;
pub mod hint;
mod compiler_tests;

enum Baz {
    FooBar,
    BarFoo
}

pub fn tttest() {
    let i = Baz::FooBar;
    match i {
        Baz::FooBar => (),
        _ => (),
    };
}

// Oracle calls are required to be wrapped in an unconstrained function
// Thus, the only argument to the `println` oracle is expected to always be an ident
#[oracle(print)]
unconstrained fn print_oracle<T>(with_newline: bool, input: T) {}

unconstrained fn print_unconstrained<T>(with_newline: bool, input: T) {
    print_oracle(with_newline, input);
}

pub fn println<T>(input: T) {
    // Safety: a print statement cannot be constrained
    unsafe {
        print_unconstrained(true, input);
    }
}

pub fn print<T>(input: T) {
    // Safety: a print statement cannot be constrained
    unsafe {
        print_unconstrained(false, input);
    }
}

// Asserts that the given value is known at compile-time.
// Useful for debugging for-loop bounds.
#[builtin(assert_constant)]
pub fn assert_constant<T>(x: T) {}

// Asserts that the given value is both true and known at compile-time.
// The message can be a string, a format string, or any value, as long as it is known at compile-time
#[builtin(static_assert)]
pub fn static_assert<T>(predicate: bool, message: T) {}

mod tests {
    use super::ops::arith::WrappingMul;

    #[test(should_fail_with = "custom message")]
    fn test_static_assert_custom_message() {
        super::static_assert(1 == 2, "custom message");
    }

    #[test]
    fn test_wrapping_mul() {
        let zero: u128 = 0;
        let one: u128 = 1;
        let two_pow_64: u128 = 0x10000000000000000;
        // let u128_max: u128 = 0xffffffffffffffffffffffffffffffff; / TODO

        // 1*0==0
        assert_eq(zero, zero.wrapping_mul(one));

        // 0*1==0
        assert_eq(zero, one.wrapping_mul(zero));

        // 1*1==1
        assert_eq(one, one.wrapping_mul(one));

        // 0 * ( 1 << 64 ) ==  0
        assert_eq(zero, zero.wrapping_mul(two_pow_64));

        // ( 1 << 64 ) * 0 == 0
        assert_eq(zero, two_pow_64.wrapping_mul(zero));

        // 1 * ( 1 << 64 ) == 1 << 64
        assert_eq(two_pow_64, two_pow_64.wrapping_mul(one));

        // ( 1 << 64 ) * 1 == 1 << 64
        assert_eq(two_pow_64, one.wrapping_mul(two_pow_64));

        // ( 1 << 64 ) * ( 1 << 64 ) == 1 << 64
        assert_eq(zero, two_pow_64.wrapping_mul(two_pow_64));
        // -1 * -1 == 1
        // assert_eq(one, u128_max.wrapping_mul(u128_max)); // TODO: make this test pass
    }
}
