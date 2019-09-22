#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_storage, decl_module, StorageValue, StorageMap, Parameter, ensure, dispatch::Result};
use system::ensure_signed;
use codec::{Encode, Decode};
use rstd::{prelude::*};
use sr_primitives::traits::{SimpleArithmetic, One, Member, Bounded};
use runtime_io::blake2_128;
use byteorder::{ByteOrder, LittleEndian};
use rstd::result;

pub trait Trait: system::Trait {
    type KittyId: Parameter + Member + SimpleArithmetic + Default + Copy;
}

#[derive(Encode, Decode, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum KittySexual {
    Male,
    Female,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<T> where T: Trait {
    id: T::KittyId,
    parents: Option<(T::KittyId, T::KittyId)>,
    sexual: KittySexual, 
    dna: u128,
    owner: T::AccountId,
}

impl<T> Kitty<T> where T: Trait {
    fn new(owner: &T::AccountId, id: &T::KittyId, parents: &Option<(T::KittyId, T::KittyId)>, 
           dna: u128, sexual: KittySexual) -> Self {
        Kitty { id: *id, parents: *parents, sexual: sexual, dna:dna, owner: owner.clone() }
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
            let owner = ensure_signed(origin)?;
            
            let id = Self::mutate_kitty_id()?;
            let dna = Self::generate_dna(&owner, &id);
            let sexual = Self::sexual_from_dna(dna);
            let new_kitty = Kitty::new(&owner, &id, &None, dna, sexual);
            <Kitties<T>>::insert(id, new_kitty);
            <KittyByOwner<T>>::add_kitty(owner, id);

            Ok(())
        }

        fn bear_kitty(origin, father_id: T::KittyId, mother_id: T::KittyId) -> Result {
            let owner = ensure_signed(origin)?;
            let father = Self::kitty(father_id);
            ensure!(father.is_some(), "Father not exist");

            let mother = Self::kitty(mother_id);
            ensure!(mother.is_some(), "Mother not exist");

            let father = father.unwrap();
            ensure!(father.owner == owner, "Father not owner by origin");
            ensure!(father.sexual == KittySexual::Male, "Father sexual mismatch");

            let mother = mother.unwrap();
            ensure!(mother.owner == owner, "Mother not owner by origin");
            ensure!(mother.sexual == KittySexual::Female, "Mother sexual mismatch");

            let id = Self::mutate_kitty_id()?;
            let rnd_dna = Self::generate_dna(&owner, &id);

            let mask = father.dna ^ mother.dna;
            let new_dna = (father.dna & !mask) | (rnd_dna & mask);

            let sexual = Self::sexual_from_dna(new_dna);

            let new_kitty = Kitty::new(&owner, &id, 
                    &Option::Some((father.id, mother.id)), 
                    new_dna, sexual);

            <Kitties<T>>::insert(id, new_kitty);
            <KittyByOwner<T>>::add_kitty(owner, id);

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

impl<T: Trait> Module<T> {
    fn mutate_kitty_id() -> result::Result<T::KittyId, &'static str> {
        let id = Self::next_kitty_id();
        if id == T::KittyId::max_value() {
            return Err("Kitty id overflow");
        }
		<NextKittyId<T>>::mutate(|id| *id += One::one());
        Ok(id)
    }

    fn generate_dna(owner: &T::AccountId, kitty_id: &T::KittyId) -> u128 {
        let dna_buf = (<system::Module<T>>::random_seed(), 
					<system::Module<T>>::block_number(), 
					owner.clone(), kitty_id).using_encoded(blake2_128);
        LittleEndian::read_u128(&dna_buf)
    }

    fn sexual_from_dna(dna: u128) -> KittySexual {
        if dna.count_ones() % 2 == 0 {
            KittySexual::Male
        } else {
            KittySexual::Female
        }
    }
}