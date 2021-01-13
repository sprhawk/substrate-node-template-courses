use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

use super::*;
type Balances = pallet_balances::Module<Test>;

#[test]
fn owned_kitties_can_append_values() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let owner = Origin::signed(1);
        Balances::make_free_balance_be(&1, 100_000);
        assert_ok!(KittiesModule::create(owner,));
        assert_eq!(OwnedKitties::<Test>::contains_key(1), true);
        let idx = OwnedKitties::<Test>::get(1);
        assert_eq!(Kitties::<Test>::contains_key(1, idx), true);
        assert_eq!(KittyOwners::<Test>::contains_key(idx), true);
        let kitty_owner = KittyOwners::<Test>::get(idx);
        assert!(kitty_owner.is_some());
        assert_eq!(1, kitty_owner.unwrap());
    });
}

#[test]
fn transfer_kitty_works() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let from = 1;
        let to = 2;
        let origin = Origin::signed(1);
        Balances::make_free_balance_be(&from, 100_000);
        Balances::make_free_balance_be(&to, 100_000);
        assert_ok!(KittiesModule::create(origin.clone()));
        let idx = OwnedKitties::<Test>::get(from);
        assert_ok!(KittiesModule::transfer(origin, to, idx));
        assert_eq!(OwnedKitties::<Test>::contains_key(from), false);
        assert_eq!(OwnedKitties::<Test>::contains_key(to), true);

        assert_eq!(KittyOwners::<Test>::contains_key(idx), true);
        let kitty_owner = KittyOwners::<Test>::get(idx);
        assert!(kitty_owner.is_some());
        assert_eq!(to, kitty_owner.unwrap());
    });
}

// #[test]
// fn cant_transfer_same_account() {
//     new_test_ext().execute_with(|| {
//         run_to_block(10);
//         let from = 1;
//         let to = 1;
//         let origin = Origin::signed(1);
//         assert_ok!(KittiesModule::create(origin.clone()));
//         let idx = OwnedKitties::<Test>::get(from);
//         assert_noop!(
//             KittiesModule::transfer(origin, to, idx),
//             Error::<Test>::CantTransferSameAccount
//         );
//     });
// }

#[test]
fn breed_works() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let account = 1;
        let origin = Origin::signed(account);
        Balances::make_free_balance_be(&account, 100_000);
        assert_ok!(KittiesModule::create(origin.clone()));
        let k1 = LastKittyIndex::<Test>::get();
        assert_ok!(KittiesModule::create(origin.clone()));
        let k2 = LastKittyIndex::<Test>::get();

        assert_ok!(KittiesModule::breed(origin.clone(), k1, k2));
        let child = LastKittyIndex::<Test>::get();

        // test parents, partners, children, etc
        assert_eq!(KittyParents::<Test>::contains_key(child), true);
        let parents = KittyParents::<Test>::get(child);
        assert_eq!(parents.contains(&k1), true);
        assert_eq!(parents.contains(&k2), true);

        for parent in &parents {
            assert_eq!(KittyChildren::<Test>::contains_key(parent), true);
            let children = KittyChildren::<Test>::get(parent);
            assert_eq!(children.contains(&child), true);
        }

        for idx in 0..2 {
            let parent = parents[idx];
            assert_eq!(KittyPartners::<Test>::contains_key(parent), true);
            let partners = KittyPartners::<Test>::get(parent);
            let partner = parents[1 - idx];
            assert_eq!(partners.contains(&partner), true);
        }
    });
}

#[test]
fn cant_breed_with_same_kitty() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let account = 1;
        let origin = Origin::signed(account);
        Balances::make_free_balance_be(&account, 100_000);
        assert_ok!(KittiesModule::create(origin.clone()));
        let k1 = LastKittyIndex::<Test>::get();

        assert_noop!(
            KittiesModule::breed(origin.clone(), k1, k1),
            Error::<Test>::RequireDifferentParent,
        );
    });
}
