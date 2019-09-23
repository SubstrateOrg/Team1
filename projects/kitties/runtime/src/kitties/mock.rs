//! Test utilities
#![cfg(test)]

use primitives::{H256, Blake2Hasher};
use support::{impl_outer_origin, parameter_types};
use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
use sr_primitives::weights::Weight;
use sr_primitives::Perbill;

use super::*;
use self::{Trait};

impl_outer_origin! {
  pub enum Origin for Test {}
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
  pub const BlockHashCount: u64 = 250;
  pub const MaximumBlockWeight: Weight = 1024;
  pub const MaximumBlockLength: u32 = 2 * 1024;
  pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
  pub const ExistentialDeposit: u64 = 0;
  pub const TransferFee: u64 = 0;
  pub const CreationFee: u64 = 0;
  pub const TransactionBaseFee: u64 = 0;
  pub const TransactionByteFee: u64 = 0;
}

impl system::Trait for Test {
  type Origin = Origin;
  type Call = ();
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type AccountId = u64;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type WeightMultiplierUpdate = ();
  type Event = ();
  type BlockHashCount = BlockHashCount;
  type MaximumBlockWeight = MaximumBlockWeight;
  type MaximumBlockLength = MaximumBlockLength;
  type AvailableBlockRatio = AvailableBlockRatio;
  type Version = ();
}
impl balances::Trait for Test {
  type Balance = u64;
  type OnNewAccount = ();
  type OnFreeBalanceZero = ();
  type Event = ();
  type TransactionPayment = ();
  type TransferPayment = ();
  type DustRemoval = ();
  type ExistentialDeposit = ExistentialDeposit;
  type TransferFee = TransferFee;
  type CreationFee = CreationFee;
  type TransactionBaseFee = TransactionBaseFee;
  type TransactionByteFee = TransactionByteFee;
  type WeightToFee = ();
}
impl Trait for Test {
  type Event = ();
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
  system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
