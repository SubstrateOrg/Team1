/// Cat Module for runtime

use rstd::prelude::*;
use codec::{Encode, Decode};
use support::{decl_module, decl_storage, decl_event, StorageMap, StorageValue, dispatch::Result};
use sr_primitives::{
  traits::{
    Zero, Hash
  }
};
use system::ensure_signed;

// for Module test
mod mock;
mod tests;

/// The module's configuration trait.
pub trait Trait: system::Trait + balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// Struct for a Kitty
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
  id: Hash,
  dna: Hash,
  price: Balance,
  gen: u64,
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as KittyStorage {
    Owners get(owners): map T::AccountId => Vec<T::Hash>;
    Kitties get(kitties): map T::Hash => Kitty<T::Hash, T::Balance>;
    KittiesOwner get(kitties_owner): map T::Hash => T::AccountId;
	}
}

decl_event!(
	pub enum Event<T> where
    AccountId = <T as system::Trait>::AccountId,
    Hash = <T as system::Trait>::Hash
  {
		NewKitty(Hash, AccountId),
	}
);

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		// fn deposit_event() = default;

    fn create_kitty(origin, id: T::Hash) -> Result {
        let sender = ensure_signed(origin)?;

        let new_kitty = Self::do_create_gen_zero_kitty(&sender, &id);

        <Kitties<T>>::insert(id, new_kitty);
        Ok(())
    }
	}
}

// Main Kitty implementation

impl<T: Trait> Module<T> {
  // create gen zero kitty
  fn do_create_gen_zero_kitty (owner: &T::AccountId, id: &T::Hash) -> Kitty<T::Hash, T::Balance> {
    let hash_of_zero = <<T as system::Trait>::Hashing as Hash>::hash_of(&0);
    let zero_price = <T as balances::Trait>::Balance::zero();

    Kitty {
      id: id.clone(),
      dna: hash_of_zero,
      price: zero_price,
      gen: 0,
    }
  }
}
