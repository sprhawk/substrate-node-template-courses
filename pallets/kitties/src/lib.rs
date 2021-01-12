#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{Get, Randomness},
    Parameter, StorageMap,
};
use frame_system::ensure_signed;
use sp_io::hashing::blake2_128;
use sp_runtime::DispatchError;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// type KittyIndex = u32;
pub type KittyIndexOf<T> = <T as Trait>::KittyIndex;

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

// use a trait to make kitty index as any type
// that implemented Uniquekittyindex trait
// can be used, not restricted by AtLeast32BitUnsigned,
// etc
pub trait UniqueKittyIndex: Sized {
    fn next_kitty_idx(&self) -> Option<Self>;
}

impl UniqueKittyIndex for u32 {
    fn next_kitty_idx(&self) -> Option<Self> {
        if self < &Self::MAX {
            Some(self + 1)
        } else {
            None
        }
    }
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;
    type KittyIndex: Parameter + UniqueKittyIndex + Default + Copy;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as Kitties {
        pub Kitties get(fn kitties): double_map hasher(blake2_128_concat) T::AccountId,  hasher(blake2_128_concat) KittyIndexOf<T> => Option<Kitty>;
        pub LastKittyIndex get(fn last_kitty_idx): KittyIndexOf<T>;
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) KittyIndexOf<T> => Option<T::AccountId>;
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        KittyIndex = <T as Trait>::KittyIndex,
    {
        Created(AccountId, KittyIndex),
        Transferred(AccountId, AccountId, KittyIndex),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        KittiesCountOverflow,
        InvalidKittyId,
        RequireDifferentParent,
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn create(origin) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let kitty_id = Self::next_kitty_id()?;
            let dna = Self::random_value(&sender);
            let kitty = Kitty(dna);

            Self::insert_kitty(&sender, kitty_id, kitty);
            Self::deposit_event(RawEvent::Created(sender, kitty_id));
            // Return a successful DispatchResult
            Ok(())
        }
        #[weight = 10_000]
        pub fn transfer(origin, to: T::AccountId, kitty_id: KittyIndexOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            <KittyOwners<T>>::insert(kitty_id, to.clone());
            Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id));
            // Return a successful DispatchResult
            Ok(())
        }

        #[weight = 10_000]
        pub fn breed(origin, kitty_id_1: KittyIndexOf<T>, kitty_id_2: KittyIndexOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;
            Self::deposit_event(RawEvent::Created(sender, new_kitty_id));
            // Return a successful DispatchResult
            Ok(())
        }
    }
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
    (selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
    fn next_kitty_id() -> sp_std::result::Result<KittyIndexOf<T>, DispatchError> {
        let kitty_id = Self::last_kitty_idx().next_kitty_idx();

        Ok(kitty_id.ok_or(Error::<T>::KittiesCountOverflow)?)
    }

    fn random_value(sender: &T::AccountId) -> [u8; 16] {
        let context: &[u8] = b"kitties pallet context";
        let r = T::Randomness::random(context);
        let payload = (r, &sender, <frame_system::Module<T>>::extrinsic_index());

        payload.using_encoded(blake2_128)
    }

    fn insert_kitty(owner: &T::AccountId, kitty_id: KittyIndexOf<T>, kitty: Kitty) {
        <Kitties<T>>::insert(owner, kitty_id, kitty);
        <LastKittyIndex<T>>::put(kitty_id);
        <KittyOwners<T>>::insert(kitty_id, owner);
    }

    fn do_breed(
        owner: &T::AccountId,
        kitty_id_1: KittyIndexOf<T>,
        kitty_id_2: KittyIndexOf<T>,
    ) -> sp_std::result::Result<KittyIndexOf<T>, DispatchError> {
        let kitty1 = Self::kitties(owner, kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
        let kitty2 = Self::kitties(owner, kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

        ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

        let kitty_id = Self::next_kitty_id()?;

        let kitty1_dna = kitty1.0;
        let kitty2_dna = kitty2.0;
        let selector = Self::random_value(&owner);
        let mut new_dna = [0u8; 16];

        for i in 0..kitty1_dna.len() {
            new_dna[i] = combine_dna(kitty1_dna[1], kitty2_dna[i], selector[i]);
        }

        Self::insert_kitty(owner, kitty_id, Kitty(new_dna));
        Ok(kitty_id)
    }
}
