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
			if count + 1 == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}
		pub fn breed1(origin,k1:u32 ,k2:u32){
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();
			if count + 1 == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let kitty1 = Self::kitty(k1);
			let kitty2 = Self::kitty(k2);
			let payload = (kitty1.0,kitty2.0, <system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}
		pub fn breed2(origin,k1:u32 ,k2:u32){
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();
			if count + 1 == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let kitty1 = Self::kitty(k1);
			let kitty2 = Self::kitty(k2);
			let payload = (kitty1.0,kitty2.0, <system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let mut dna = payload.using_encoded(blake2_128);
			let mut i = 0;
			let dna1 = kitty1.0;
			let dna2 = kitty2.0;
			while i<16 {
				match dna[i] % 3  {
					1 => {
						dna[i] = dna1[i] + dna2[i];
					},
					2 => {
						dna[i] = dna1[i];
					},
					0 => {
						dna[i] = dna2[i];
					},
					_ => {
						dna[i] = 0;
					}
				}
				i = i + 1
			}
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}
	}
}
