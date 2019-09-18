/// Cat Module for runtime

use codec::{Encode, Decode};
use support::{
  ensure,
  decl_module, decl_storage, decl_event,
  StorageMap, StorageValue,
  dispatch::Result
};
use sr_primitives::{
  traits::{
    Zero, Hash
  }
};
use system::{ensure_signed};

// for Module test
// mod mock;
// mod tests;

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
  dna: u128,
  gen: u64,
  price: Balance,
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as KittyStorage {
    // All kitties
    pub Kitties get(kitties): map T::Hash => Kitty<T::Hash, T::Balance>;
    pub KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

    // List of kitties
    AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
    AllKittiesCount get(kitties_amount): u64;
    AllKittiesIndex: map T::Hash => u64;

    // List of kitties owners
    OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
    OwnedKittiesCount get(owned_kitty_amount): map T::AccountId => u64;
    OwnedKittiesIndex: map T::Hash => u64;

    // Nonce
    Nonce: u128;
	}
}

decl_event!(
	pub enum Event<T> where
    AccountId = <T as system::Trait>::AccountId,
    Hash = <T as system::Trait>::Hash
  {
		Created(AccountId, Hash),
	}
);

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
    fn deposit_event() = default;

    fn create_kitty(origin) -> Result {
      let sender = ensure_signed(origin)?;

      // create new kitty
      let new_id = Self::_gen_random_hash(&sender);
      let new_dna = Self::_gen_dna_by_random_hash(&new_id);
      let new_kitty = Self::_create_gen_zero_kitty(&new_id, new_dna);

      // mint kitty
      Self::_mint_kitty(sender, new_id, new_kitty)?;

      Ok(())
    }
	}
}

// Main Kitty implementation

impl<T: Trait> Module<T> {
  fn _gen_random_hash (sender: &T::AccountId) -> T::Hash {
    let random_seed = <system::Module<T>>::random_seed();
    let nonce = <Self as Store>::Nonce::get();

    (random_seed, sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash)
  }

  fn _gen_dna_by_random_hash<M: AsRef<[u8]>> (random_hash: &M) -> u128 {
    let rand = random_hash.as_ref();
    let mut flag = false;
    let mut ret: u128 = 0;
    for element in rand.iter() {
      if !flag {
        ret += *element as u128;
      } else {
        ret <<= 8;
      }
      flag = !flag;
    }
    ret
  }

  // create gen zero kitty
  fn _create_gen_zero_kitty (id: &T::Hash, dna_data: u128) -> Kitty<T::Hash, T::Balance> {
    Kitty {
      id: id.clone(),
      dna: dna_data,
      price: <T as balances::Trait>::Balance::zero(),
      gen: 0,
    }
  }

  // mint a new Kitty
  fn _mint_kitty (kitty_owner: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {
    ensure!(!<Kitties<T>>::exists(kitty_id), "This kitty id already exists");
    // calc index
    let curr_count_index = Self::kitties_amount();
    let new_count = curr_count_index.checked_add(1).ok_or("Overflow adding a new kitty")?;

    let curr_owner_amount = Self::owned_kitty_amount(&kitty_owner);
    let new_owner_amount = curr_owner_amount.checked_add(1).ok_or("Overflow adding a new kitty of owner")?;

    // Save kitty
    <Kitties<T>>::insert(kitty_id, new_kitty);
    <KittyOwner<T>>::insert(kitty_id, &kitty_owner);
    // add kitty to all list
    <AllKittiesArray<T>>::insert(curr_count_index, kitty_id);
    <AllKittiesIndex<T>>::insert(kitty_id, curr_count_index);
    <Self as Store>::AllKittiesCount::put(new_count);
    // add kitty to owner
    <OwnedKittiesArray<T>>::insert((kitty_owner.clone(), curr_owner_amount), kitty_id);
    <OwnedKittiesCount<T>>::insert(&kitty_owner, new_owner_amount);
    <OwnedKittiesIndex<T>>::insert(kitty_id, curr_owner_amount);

    // inc nonce
    <Self as Store>::Nonce::mutate(|n| *n += 1);
    
    // dispatch event
    Self::deposit_event(RawEvent::Created(kitty_owner, kitty_id));

    Ok(())
  }
}
