use support::{decl_module, decl_storage, StorageValue, StorageMap};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;

pub trait Trait: system::Trait {
}

#[derive(Encode, Decode, Default)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();
			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
// 			KittiesCount::put(count + 1);
			let new_kitties_count = Self::kitties_count().checked_add(1)
                	.ok_or("Kitties count overflow")?;
            		KittiesCount::put(new_kitties_count);
		}
		/// Breed kitties
		pub fn breed(origin, kitty_id_one: u32, kitty_id_two: u32) {
			let sender = ensure_signed(origin)?;

            		let kitty1 = Self::kitty(kitty_id_one);
		    	let kitty2 = Self::kitty(kitty_id_two);
//		    	ensure!(kitty1.is_some(), "非法的id");
//		    	ensure!(kitty2.is_some(), "非法的id");
		    	ensure!(kitty_id_one != kitty_id_two, "需要一个父代和一个母代");
		    	let kitty_id = Self::kitties_count();
		    	let kitty1_dna = kitty1.0;
		    	let kitty2_dna = kitty2.0;
		    	//
		    	let random = (<system::Module<T>>::random_seed(), &sender,
            		<system::Module<T>>::extrinsic_index());
			let hashed = random.using_encoded(blake2_128);
			let mut new_dna = [0u8; 16];
		   	for i in 0..kitty1_dna.len() {
				if i%3 == 0 {
			    		new_dna[i] = kitty1_dna[i];
				} else if  i%3 == 1{
			    		new_dna[i] = kitty2_dna[i];
				} else {
			    		new_dna[i] = hashed[i];
				}
		    	}
		    	Kitties::insert(kitty_id, Kitty(new_dna));
//			KittiesCount::put(count + 1);
            		let new_kitties_count = Self::kitties_count().checked_add(1)
                	.ok_or("Kitties count overflow")?;
            		KittiesCount::put(new_kitties_count);
		}
	}
}
