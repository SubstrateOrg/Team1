#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_storage, decl_module, StorageValue, StorageMap, Parameter, dispatch::Result};
use system::ensure_signed;
use codec::{Encode, Decode};
use rstd::{prelude::*};
use sr_primitives::traits::{SimpleArithmetic, One, Member};
use runtime_io::blake2_128;
use byteorder::{ByteOrder, LittleEndian};

pub trait Trait: system::Trait {
    type KittyId: Parameter + Member + SimpleArithmetic + Default + Copy;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<T> where T: Trait {
    id: T::KittyId,
    dna: u128,
    owner: T::AccountId,
}

impl<T> Kitty<T> where T: Trait {
    fn new(owner: T::AccountId, id: T::KittyId) -> Self {
        let dna_buf = (<system::Module<T>>::random_seed(), 
					<system::Module<T>>::block_number(), 
					owner.clone()).using_encoded(blake2_128);
        let dna = LittleEndian::read_u128(&dna_buf);
        Kitty { id, dna, owner }    
    }
}

impl<T> KittyByOwner<T> where T: Trait {
    fn add_kitty(owner: T::AccountId, id: T::KittyId) {
        let mut kitties;
        if let Some(kits) = Self::get(owner.clone()) {
            kitties = kits;
        } else {
            kitties = Vec::new();
        }

        kitties.push(id);
        Self::insert(owner, kitties);
    }
}

decl_module!{
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;
            
            
            let id = Self::next_kitty_id();
			<NextKittyId<T>>::mutate(|id| *id += One::one());

            let new_kitty = Kitty::new(sender.clone(), id);
            <Kitties<T>>::insert(id, new_kitty);
            <KittyByOwner<T>>::add_kitty(sender, id);

            Ok(())
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        NextKittyId get(next_kitty_id): T::KittyId;
        Kitties get(kitty): map T::KittyId => Option<Kitty<T>>;
        KittyByOwner get(owner): map T::AccountId => Option<Vec<T::KittyId>>;
    }
}