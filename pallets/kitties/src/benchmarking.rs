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
        });
    }
}
