use support::{decl_module, decl_storage, ensure, StorageValue, StorageMap, dispatch::Result, Parameter};
use sr_primitives::traits::{One, SimpleArithmetic, Bounded, CheckedAdd, CheckedSub};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;
use rstd::result;

pub trait Trait: system::Trait {
	type KittyIndex: Parameter + SimpleArithmetic + Bounded + Default + Copy;
}

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map T::KittyIndex => Option<Kitty>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): T::KittyIndex;

		/// Get kitty ID by account ID and user kitty index
		pub OwnedKitties get(owned_kitties): map (T::AccountId, T::KittyIndex) => T::KittyIndex;
		/// Get number of kitties by account ID
		pub OwnedKittiesCount get(owned_kitties_count): map T::AccountId => T::KittyIndex;
		/// Get index of kitties by KittyIndex
		pub OwnedKittiesIndex get(owned_kitties_index): map T::KittyIndex => T::KittyIndex;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;

			// 作业：重构create方法，避免重复代码
			Self::do_create(sender)?;
		}

		/// Breed kitties
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
		}

		pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			Self::do_transfer(sender, to, kitty_id)?;
		}
	}
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		payload.using_encoded(blake2_128)
	}

	fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
		// 作业：实现combine_dna
		let mut final_dna = 0b00000000;
		for i in 0..8 {
			let flag = 1 << i;
			if flag & selector == 0u8 {
				final_dna = final_dna | (dna2 & flag);
			} else {
				final_dna = final_dna | (dna1 & flag);
			}
		}
		// selector.map_bits(|bit, index| if (bit == 1) { dna1 & (1 << index) } else { dna2 & (1 << index) })
		// 注意 map_bits这个方法不存在。只要能达到同样效果，不局限算法
		// 测试数据：dna1 = 0b11110000, dna2 = 0b11001100, selector = 0b10101010, 返回值 0b11100100
		final_dna
	}

	fn next_kitty_id() -> result::Result<T::KittyIndex, &'static str> {
		let kitty_id = Self::kitties_count();
		if kitty_id == T::KittyIndex::max_value() {
			return Err("Kitties count overflow");
		}
		Ok(kitty_id)
	}

	fn insert_kitty(owner: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
		// Create and store kitty
		<Kitties<T>>::insert(kitty_id, kitty);
		<KittiesCount<T>>::put(kitty_id + 1.into());

		// Store the ownership information
		let user_kitties_id = Self::owned_kitties_count(owner.clone());
		<OwnedKitties<T>>::insert((owner.clone(), user_kitties_id), kitty_id);
		<OwnedKittiesIndex<T>>::insert(kitty_id, user_kitties_id);
		<OwnedKittiesCount<T>>::insert(owner, user_kitties_id + 1.into());
	}

  // mint a new Kitty
  fn do_create (who: T::AccountId) -> Result {
		let kitty_id = Self::next_kitty_id()?;

		// Generate a random 128bit value
		let payload = (<system::Module<T>>::random_seed(), &who, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		let dna = payload.using_encoded(blake2_128);

		// Create and store kitty
		Self::insert_kitty(who, kitty_id, Kitty(dna));

    Ok(())
  }

	fn do_breed(sender: T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> Result {
		let kitty1 = Self::kitty(kitty_id_1);
		let kitty2 = Self::kitty(kitty_id_2);

		ensure!(kitty1.is_some(), "Invalid kitty_id_1");
		ensure!(kitty2.is_some(), "Invalid kitty_id_2");
		ensure!(kitty_id_1 != kitty_id_2, "Needs different parent");

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.unwrap().0;
		let kitty2_dna = kitty2.unwrap().0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty
		for i in 0..kitty1_dna.len() {
			new_dna[i] = Self::combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

		Ok(())
	}

	fn do_transfer(sender: T::AccountId, to: T::AccountId, kitty_id: T::KittyIndex) -> Result {
		let kitty = Self::kitty(kitty_id);

		ensure!(kitty.is_some(), "Invalid kitty_id");
		ensure!(sender != to, "Needs different accouont");

		// kitties count
		let curr_owned_count_from = Self::owned_kitties_count(&sender);
		let curr_owned_count_to = Self::owned_kitties_count(&to);

		let new_owned_count_from = curr_owned_count_from.checked_sub(&T::KittyIndex::one()).ok_or("overflow owned_kitty_from")?;
		let new_owned_count_to = curr_owned_count_to.checked_add(&T::KittyIndex::one()).ok_or("overflow owned_kitty_to")?;

		let kitty_index = Self::owned_kitties_index(kitty_id);
		// swap last element
		if kitty_index != new_owned_count_from {
				let last_kitty_id = <OwnedKitties<T>>::get((sender.clone(), new_owned_count_from));
				<OwnedKitties<T>>::insert((sender.clone(), kitty_index), last_kitty_id);
				<OwnedKittiesIndex<T>>::insert(last_kitty_id, kitty_index);
		}
		// remove last element
		<OwnedKitties<T>>::remove((sender.clone(), new_owned_count_from));

		// add to target account
		<OwnedKitties<T>>::insert((to.clone(), curr_owned_count_to), kitty_id);
		<OwnedKittiesIndex<T>>::insert(kitty_id, curr_owned_count_to);

		// update OwnedKittiesCount for `from` and `to`
		OwnedKittiesCount::<T>::insert(sender.clone(), new_owned_count_from);
		OwnedKittiesCount::<T>::insert(to.clone(), new_owned_count_to);

		// Send a transfer event
		// TODO

		Ok(())
	}
}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, parameter_types};
	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
	use sr_primitives::weights::Weight;
	use sr_primitives::Perbill;

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
	impl Trait for Test {
		type KittyIndex = u64;
	}
	type KittiesModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_combine_dna() {
		with_externalities(&mut new_test_ext(), || {
			// 测试数据：dna1 = 0b11110000, dna2 = 0b11001100, selector = 0b10101010, 返回值 0b11100100
			let dna = KittiesModule::combine_dna(0b11110000, 0b11001100, 0b10101010);
			assert_eq!(dna, 0b11100100);
		});
	}

	#[test]
	fn it_works_for_transfer() {
		with_externalities(&mut new_test_ext(), || {
			assert_ok!(KittiesModule::do_create(1));
			assert_eq!(KittiesModule::kitties_count(), 1);
			assert_eq!(KittiesModule::owned_kitties((1, 0)), 0);
			assert_eq!(KittiesModule::owned_kitties_count(1), 1);
			assert_ok!(KittiesModule::do_transfer(1, 2, 0));
			assert_eq!(KittiesModule::owned_kitties_count(1), 0);
			assert_eq!(KittiesModule::owned_kitties_count(2), 1);
			assert_eq!(KittiesModule::owned_kitties((2, 0)), 0);
		});
	}
}
