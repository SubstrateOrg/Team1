//! tests for this module
#![cfg(test)]

use super::*;

use runtime_io::with_externalities;
use support::{assert_ok};

use self::mock::{Test, Origin, new_test_ext};

type KittyModule = Module<Test>;

#[test]
fn it_works_for_default_value() {
  with_externalities(&mut new_test_ext(), || {
    assert_ok!(KittyModule::create_kitty(Origin::signed(1)));
    assert_eq!(KittyModule::kitties_amount(), 1);
    let hash = KittyModule::kitty_by_index(0);
    assert_eq!(<KittyModule as Store>::AllKittiesIndex::get(hash), 0);
    if let Some(o) = KittyModule::owner_of(hash) {
      assert_eq!(KittyModule::kitty_of_owner_by_index((o, 0)), hash);
    }
    let kitty = KittyModule::kitties(hash);
    assert_eq!(kitty.id, hash);
    assert_eq!(kitty.gen, 0);
    assert_eq!(kitty.price, 0);
  });
}