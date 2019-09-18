//! tests for this module
#![cfg(test)]

use super::*;

use runtime_io::with_externalities;
use support::{assert_ok};

use mock::{Origin, new_test_ext};

#[test]
fn it_works_for_default_value() {
  with_externalities(&mut new_test_ext(), || {
    // // calling the `do_something` function with a value 42
    // assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
    // // asserting that the stored value is equal to what we stored
    // assert_eq!(TemplateModule::something(), Some(42));
  });
}