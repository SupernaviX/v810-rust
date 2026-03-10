// Checks that these functions are branchless.
//
//@ compile-flags: -Copt-level=3
//@ known-bug: #153504

#![crate_type = "lib"]

// CHECK-LABEL: @is_ascii_alphanumeric_char
#[no_mangle]
pub fn is_ascii_alphanumeric_char(x: char) -> bool {
    // CHECK-NOT: br {{.*}}
    x.is_ascii_alphanumeric()
}
