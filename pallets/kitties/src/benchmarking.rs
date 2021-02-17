#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

use crate::Module as KittiesModule;

// fn assert_last_event<T: Config>(generic_event: T::Event) {
//     let events = frame_system::Module::<T>::events();
//     let system_event: <T as frame_system::Config>::Event = generic_event.into();
//     // compare to the last event record
//     let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
//     assert_eq!(event, &system_event);
// }

benchmarks! {
    _ {
        let b in 1 .. 1000 => ();
    }

    do_create {
        let b in ...;
        let caller = whitelisted_caller();
        let _ = T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

    }: create (RawOrigin::Signed(caller.clone()))
        verify {
            let idx = OwnedKitties::<T>::get(&caller);
            assert_eq!(Kitties::<T>::contains_key(&caller, idx), true);
            // let amount = <T as Trait>::KittyDepositBase::get();
            // assert_last_event::<T>(Event::Created(Default::default(), caller.clone(), idx, amount).into());
        }

    do_transfer {
        let b in ...;
        let caller = whitelisted_caller();
        let _ = T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
        let target: T::AccountId = account("target", 0, 0);
        let _ = T::Currency::make_free_balance_be(&target, BalanceOf::<T>::max_value());

        KittiesModule::<T>::create(RawOrigin::Signed(caller.clone()).into());
        let idx = OwnedKitties::<T>::get(&caller);

    }: transfer (RawOrigin::Signed(caller.clone()), target.clone(), idx)
        verify {
            let idx2 = OwnedKitties::<T>::get(&target);
            assert_eq!(Kitties::<T>::contains_key(&target, idx2), true);
            // let amount = <T as Trait>::KittyDepositBase::get();
            // assert_last_event::<T>(Event::Created(Default::default(), caller.clone(), idx, amount).into());
        }

    do_breed {
        let b in ...;
        let caller = whitelisted_caller();
        let _ = T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
        KittiesModule::<T>::create(RawOrigin::Signed(caller.clone()).into());
        let k1 = LastKittyIndex::<T>::get();
        KittiesModule::<T>::create(RawOrigin::Signed(caller.clone()).into());
        let k2 = LastKittyIndex::<T>::get();

    }: breed (RawOrigin::Signed(caller.clone()), k1, k2)
        verify {
            let child = LastKittyIndex::<T>::get();
            let parents = KittyParents::<T>::get(child);
            assert_eq!(parents.contains(&k1), true);
            assert_eq!(parents.contains(&k2), true);
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn test_benchmarks() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_do_create::<Test>());
            assert_ok!(test_benchmark_do_transfer::<Test>());
        });
    }
}
