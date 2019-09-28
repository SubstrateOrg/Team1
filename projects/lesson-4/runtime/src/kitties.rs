use support::{decl_module, decl_storage, ensure, StorageValue, StorageMap, dispatch::Result, Parameter};
use sr_primitives::traits::{SimpleArithmetic, Bounded};
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
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			// user validate
			let sender = ensure_signed(origin)?;

			// 作业：重构create方法，避免重复代码
			// explanation: 宏里面没有IDE不能识别语法, 可以在将help method 写在宏外面 2019-09-26

			let kitty_id = Self::next_kitty_id().unwrap();

			let dna = Self::random_value(&sender);

			// Create and store kitty
			let kitty = Kitty(dna);

			Self::insert_kitty(sender.clone(), kitty_id, kitty);
		}

		/// Breed kitties
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)?;

			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
		}

		/// transfer kitty from one to another
		pub fn transfer(origin,to: T::AccountId,sender_kitty_id: T::KittyIndex){
			let sender = ensure_signed(origin)?;

			Self::do_transfer(sender, to, sender_kitty_id)?;
		}
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	// 作业：实现combine_dna
	// 伪代码：
	// selector.map_bits(|bit, index| if (bit == 1) { dna1 & (1 << index) } else { dna2 & (1 << index) })
	// 注意 map_bits这个方法不存在。只要能达到同样效果，不局限算法
	// 测试数据：dna1 = 0b11110000, dna2 = 0b11001100, selector = 0b10101010, 返回值 0b11100100
//	let mut ret: u8 = 0;
//	if selector & 0x01 {
//		ret |= dna1 & 0x01;
//	}else {
//		ret |= dna2 & 0x01;
//	}
    ((selector & dna1) | (!selector & dna2))
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		payload.using_encoded(blake2_128)
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
		<Kitties<T>>::insert(kitty_id.clone(), kitty);
		<KittiesCount<T>>::put(kitty_id.clone() + 1.into());

		// Store the ownership information
		let user_kitties_id = Self::owned_kitties_count(owner.clone());
		<OwnedKitties<T>>::insert((owner.clone(), user_kitties_id), kitty_id.clone());
		<OwnedKittiesCount<T>>::insert(owner, user_kitties_id + 1.into());
	}

    // reference from: https://github.com/woyoutlz/Team1/blob/master/projects/lesson-4/runtime/src/kitties.rs
    fn do_transfer(sender: T::AccountId,to: T::AccountId,sender_kitty_id: T::KittyIndex) -> Result {
        let kitty_id = Self::owned_kitties((sender.clone(),sender_kitty_id));
        let sender_counts = Self::owned_kitties_count(sender.clone());
        let kitty = Self::kitty(kitty_id);
        ensure!(kitty.is_some(), "Invalid kitty_id_1");
        <OwnedKitties<T>>::remove((sender.clone(), sender_kitty_id));
        let last_user_kitty_id = sender_counts - 1.into();
        if last_user_kitty_id != sender_kitty_id {
            let last_kitty_id =  Self::owned_kitties((sender.clone(),last_user_kitty_id));
            <OwnedKitties<T>>::remove((sender.clone(), last_user_kitty_id));
            <OwnedKitties<T>>::insert((sender.clone(), sender_kitty_id),last_kitty_id);
        }
        <OwnedKittiesCount<T>>::insert(sender, last_user_kitty_id);
        let user_kitties_id = Self::owned_kitties_count(to.clone());
        <OwnedKitties<T>>::insert((to.clone(), user_kitties_id), kitty_id);
        <OwnedKittiesCount<T>>::insert(to, user_kitties_id + 1.into());
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
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

		Ok(())
	}
}
