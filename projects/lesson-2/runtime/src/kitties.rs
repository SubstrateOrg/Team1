/// reference from here: https://github.com/anray1980/Team5
/// I still don't understand something like 
/// `AllKittiesArray get(kitty_by_index): map u64 => T::Hash;`

use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use sr_primitives::traits::{Hash};
use codec::{Encode, Decode};

#[derive(Encode, Decode, Default, Clone)]
pub struct Kitty<Hash, U64> {
    id: Hash,
    dna: Hash,
    price: U64,
    gen: U64,
}

pub trait Trait: system::Trait {}


decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, u64>;
        KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
        AllKittiesCount get(all_kitties_count): u64;
        AllKittiesIndex: map T::Hash => u64;

        OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;
        OwnedKittiesIndex: map T::Hash => u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_kitty(origin, salt: u64) -> Result {
            let _sender = ensure_signed(origin)?;
            let random_hash = (<system::Module<T>>::random_seed(), &_sender, salt)
                .using_encoded(<T as system::Trait>::Hashing::hash);

            ensure!(!<KittyOwner<T>>::exists(random_hash), "Kitty already exists");

            let _all_kitties_count = Self::all_kitties_count();
            let _new_all_kitties_count = _all_kitties_count.checked_add(1)
                .ok_or("Overflow adding a new kitty to total supply")?;

            let _owned_kitty_count = Self::owned_kitty_count(&_sender);
            let _new_owned_kitty_count = _owned_kitty_count.checked_add(1)
                .ok_or("Overflow adding a new kitty to account")?;

            let new_kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: 0,
                gen: 0,
            };

            <Kitties<T>>::insert(random_hash, new_kitty);
            <KittyOwner<T>>::insert(random_hash, &_sender);

            <AllKittiesArray<T>>::insert(_all_kitties_count, random_hash);
            <AllKittiesCount>::put(_new_all_kitties_count);
            <AllKittiesIndex<T>>::insert(random_hash, _all_kitties_count);

            <OwnedKittiesArray<T>>::insert((_sender.clone(), _owned_kitty_count), random_hash);
            <OwnedKittiesCount<T>>::insert(&_sender, _new_owned_kitty_count);
            <OwnedKittiesIndex<T>>::insert(random_hash, _owned_kitty_count);

            Ok(())
        }
    }
}