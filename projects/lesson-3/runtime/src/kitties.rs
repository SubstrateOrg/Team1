use support::{decl_module, decl_storage, StorageValue, StorageMap, decl_event, dispatch::Result};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;
use sr_primitives::traits::Hash;

// pub trait Trait: system::Trait {
// }

pub trait Trait: balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash, Balance> {
	id: Hash,
	dna: [u8; 16],
	price: Balance,
}

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {

		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map u32 => Kitty<T::Hash, T::Balance>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): u32;

		Nonce: u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();
			// homework 3-1 fix overflow of KittiesCount, Done
			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let payload = (<system::Module<T>>::random_seed(), &sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let random_dna = payload.using_encoded(blake2_128);
			let nonce = Nonce::get();
			let id_payload = (<system::Module<T>>::random_seed(), &sender, nonce);
			let random_id = id_payload.using_encoded(<T as system::Trait>::Hashing::hash);


			let kitty = Kitty {
				id: random_id,
				dna: random_dna,
				price: 0.into(),
				// price: <T::Balance as Into<u64>>::sa(0.into()),
			};
			<Kitties<T>>::insert(count, kitty);
			KittiesCount::put(count + 1);
			Nonce::mutate(|n| *n += 1);
		}

		/// Breed a new kitty
		pub fn breed_kitty(origin, kitty_idx_1: u32, kitty_idx_2: u32) -> Result {
			let sender = ensure_signed(origin)?;
			if <Kitties<T>>::exists(kitty_idx_1) {
				return Err("Kitty 1 does NOT exists.");
			}
			if <Kitties<T>>::exists(kitty_idx_2) {
				return Err("Kitty 2 does NOT exists.");
			}
			let kitty_1 = Self::kitty(kitty_idx_1);
			let kitty_2 = Self::kitty(kitty_idx_2);

			let payload = (<system::Module<T>>::random_seed(), &sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let random_dna = payload.using_encoded(blake2_128);
			let mut final_dna = kitty_1.dna;
            for (i, (dna_2_element, r)) in kitty_2.dna.as_ref().iter().zip(random_dna.as_ref().iter()).enumerate() {
                if r % 2 == 0 {
                    final_dna.as_mut()[i] = *dna_2_element;
                }
            }
			// todo: refactor these mint process to a private common func.
			let count = Self::kitties_count();
			// homework 3-1 fix overflow of KittiesCount, Done
			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let nonce = Nonce::get();
			let id_payload = (<system::Module<T>>::random_seed(), &sender, nonce);
			let random_id = id_payload.using_encoded(<T as system::Trait>::Hashing::hash);

			let kitty = Kitty {
				id: random_id,
				dna: final_dna,
				price: 0.into(),
				// price: <T::Balance as Into<u64>>::sa(0.into()),
			};
			<Kitties<T>>::insert(count, kitty);
			KittiesCount::put(count + 1);
			Nonce::mutate(|n| *n += 1);

			Ok(())
		}
	}

}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),
	}
);
