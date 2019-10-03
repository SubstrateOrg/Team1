#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_storage, decl_module, StorageValue, StorageMap, Parameter, ensure, dispatch::Result, traits::Currency};
use system::ensure_signed;
use codec::{Encode, Decode};
use rstd::{prelude::*};
use sr_primitives::traits::{SimpleArithmetic, One, Member, Bounded};
use runtime_io::blake2_128;
use byteorder::{ByteOrder, LittleEndian};
use rstd::result;

pub trait Trait: balances::Trait {
    type KittyId: Parameter + Member + SimpleArithmetic + Default + Copy;
}


decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        NextKittyId get(next_kitty_id): T::KittyId;
        Kitties get(kitties): map T::KittyId => Option<Kitty<T>>;
        KittyItems get(kitty_items): map (T::AccountId, Option<T::KittyId>) => Option<KittyLinkedItem<T>>;
    }
}

#[derive(Encode, Decode, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum KittySexual {
    Male,
    Female,
}

#[derive(Encode, Decode, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<T> where T: Trait {
    id: T::KittyId,
    parents: Option<(T::KittyId, T::KittyId)>,
    sexual: KittySexual,
    dna: u128,
    owner: T::AccountId,
    price: Option<T::Balance>,
}

impl<T> Kitty<T> where T: Trait {
    fn new(owner: &T::AccountId, id: &T::KittyId, parents: &Option<(T::KittyId, T::KittyId)>, 
           dna: u128, sexual: KittySexual) -> Self {
        Kitty { id: *id, parents: *parents, sexual: sexual, dna:dna, owner: owner.clone(), price:None }
    }

    pub fn is_on_sale(self) -> bool {
        self.price.is_some()
    }
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct KittyLinkedItem<T> where T: Trait {
    owner: T::AccountId,
    id:   Option<T::KittyId>,
    prev: Option<T::KittyId>,
    next: Option<T::KittyId>,
}

impl<T> KittyLinkedItem<T> where T: Trait {
    fn new(owner: &T::AccountId, id: &Option<T::KittyId>) -> Self {
        KittyLinkedItem { owner: owner.clone(), id: id.clone(), prev: None, next: None }
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
            //<Kitties<T>>::insert(owner, id, new_kitty);
            
            <Kitties<T>>::insert(id, new_kitty);
            Self::add_kitty_item(owner, id)?;

            Ok(())
        }

        fn transfer_kitty(origin, to: T::AccountId, id: T::KittyId) -> Result {
            let owner = ensure_signed(origin)?;
            let kitty = Self::kitties(id);
            ensure!(kitty.is_some(), "Kitty not exist");
            let kitty = kitty.unwrap();
            ensure!(owner != to, "Can not self transfer");
            ensure!(kitty.owner == owner, "Only owner can transfer kitty");
            ensure!(!kitty.is_on_sale(), "Kitty on sale can not be transferred");

            Self::do_transfer_kitty(owner.clone(), to.clone(), id)?;
            Ok(())
        }

        fn sell_kitty(origin, id: T::KittyId, #[compact] price: T::Balance) -> Result {
            let seller = ensure_signed(origin)?;
            let mut kitty = Self::find_kitty(&id)?;
            ensure!(kitty.owner == seller, "Only owner can sell kitty");
            ensure!(!kitty.price.is_some(), "Kitty already on sale");

            kitty.price = Option::Some(price);
            <Kitties<T>>::insert(id, kitty);
            // <Kitties<T>>::mutate(id, |item| {
            //     match item {
            //         Some(_item) => { _item.price = Option::Some(price); },
            //         _ => {},
            //     }
            // });

            Ok(())
        }

        fn cancel_sell_kitty(origin, id: T::KittyId) -> Result {
            let seller = ensure_signed(origin)?;
            let mut kitty = Self::find_kitty(&id)?;
            ensure!(kitty.owner == seller, "Owned by others");
            ensure!(kitty.price.is_some(), "Kitty not on sale");

            kitty.price = Option::None;
            <Kitties<T>>::insert(id, kitty);
            Ok(())
        }

        fn buy_kitty(origin, id: T::KittyId, #[compact] price: T::Balance) -> Result {
            let buyer = ensure_signed(origin)?;
            let kitty: Kitty<T> = Self::find_kitty(&id)?;
            ensure!(kitty.clone().is_on_sale(), "Kitty not on sale");
            ensure!(kitty.price.unwrap() <= price, "Offering a lower bid than sell price");
            
            <balances::Module<T> as Currency<_>>::transfer(&buyer, &kitty.owner, kitty.price.unwrap())?;
            <Kitties<T>>::mutate(id, |item| {
                match item {
                    Some(_item) => { _item.price = None; },
                    _ => {},
                }
            });
            Self::do_transfer_kitty(kitty.owner, buyer.clone(), id)?;
            Ok(())
        }

        fn bear_kitty(origin, father_id: T::KittyId, mother_id: T::KittyId) -> Result {
            let owner = ensure_signed(origin)?;
            let father = Self::find_kitty(&father_id)?;
            let mother = Self::find_kitty(&mother_id)?;
    
            ensure!(father.owner == owner, "Father not owner by origin");
            ensure!(father.sexual == KittySexual::Male, "Father sexual mismatch");

            ensure!(mother.owner == owner, "Mother not owner by origin");
            ensure!(mother.sexual == KittySexual::Female, "Mother sexual mismatch");

            let id = Self::mutate_kitty_id()?;
            let rnd_dna = Self::generate_dna(&owner, &id);

            // 基准代码要求实现combine_dna，实现如下
            // return (dna1 & selector) | (dna2 & !selector)

            let mask = father.dna ^ mother.dna;
            let new_dna = (father.dna & !mask) | (rnd_dna & mask);

            let sexual = Self::sexual_from_dna(new_dna);

            let new_kitty = Kitty::new(&owner, &id, 
                    &Option::Some((father.id, mother.id)), 
                    new_dna, sexual);

            <Kitties<T>>::insert(id, new_kitty);
            Self::add_kitty_item(owner, id)?;

            Ok(())
        }
    }
}


impl<T: Trait> Module<T> {
    fn find_kitty(id: &T::KittyId) -> result::Result<Kitty<T>, &'static str> {
        let kitty = Self::kitties(id);
        ensure!(kitty.is_some(), "Kitty not exist");
        Ok(kitty.unwrap())
    }

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

    fn do_transfer_kitty(from: T::AccountId, to: T::AccountId, id: T::KittyId) -> Result {
        // mutate owner in Kitty struct
        <Kitties<T>>::mutate(id, |item| {
            match item {
                Some(_item) => { _item.owner = to.clone() },
                _ => {},
            }
        });

        // update link list
        Self::remove_kitty_item(from, id)?;
        Self::add_kitty_item(to, id)?;
        Ok(())
    }

    fn remove_kitty_item(owner: T::AccountId, id: T::KittyId) -> Result {
        let linked_item = Self::kitty_items((owner.clone(), Option::Some(id)));
        ensure!(linked_item.is_some(), "Fatal error");
        let linked_item = linked_item.unwrap();
        <KittyItems<T>>::mutate((owner.clone(), linked_item.prev), |item| {
            match item {
                Some(_item) => { _item.next = linked_item.next; },
                _ => {},
            }
        });
        <KittyItems<T>>::mutate((owner.clone(), linked_item.next), |item| {
            match item {
                Some(_item) => { _item.prev = linked_item.prev; },
                _ => {},
            }
        });
        <KittyItems<T>>::remove((owner, Option::Some(id)));
        Ok(())
    }

    fn add_kitty_item(owner: T::AccountId, id: T::KittyId) -> Result {
        let head = Self::kitty_items((owner.clone(), Option::None));
        if ! head.is_some() {
            let new_head = KittyLinkedItem::new(&owner, &Option::None);
            <KittyItems<T>>::insert((owner.clone(), Option::None), new_head);
        }

        let head = Self::kitty_items((owner.clone(), Option::None));
        let head = head.unwrap();
        let first_item = Self::kitty_items((owner.clone(), head.next));

        ensure!(first_item.is_some(), "Fatal error");
        let first_item = first_item.unwrap();

        let mut new_item = KittyLinkedItem::new(&owner, &Option::Some(id));
        new_item.prev = first_item.prev;
        new_item.next = head.next;

        <KittyItems<T>>::mutate((owner.clone(), head.next), |item| {
            match item {
                Some(_item) => {_item.prev = Some(id);},
                _ => {},
            }
        });

        <KittyItems<T>>::mutate((owner.clone(), None), |item| {
            match item {
                Some(_item) => {_item.next = Some(id);},
                _ => {},
            }
        });

        <KittyItems<T>>::insert((owner, Option::Some(id)), new_item);

        Ok(())
    }
}