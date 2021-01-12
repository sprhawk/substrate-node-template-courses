use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

use super::*;

#[test]
fn owned_kitties_can_append_values() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let owner = Origin::signed(1);
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

#[test]
fn cant_transfer_same_account() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let from = 1;
        let to = 1;
        let origin = Origin::signed(1);
        assert_ok!(KittiesModule::create(origin.clone()));
        let idx = OwnedKitties::<Test>::get(from);
        assert_noop!(
            KittiesModule::transfer(origin, to, idx),
            Error::<Test>::CantTransferSameAccount
        );
    });
}

#[test]
fn breed_works() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let account = 1;
        let origin = Origin::signed(account);
        assert_ok!(KittiesModule::create(origin.clone()));
        let k1 = LastKittyIndex::<Test>::get();
        assert_ok!(KittiesModule::create(origin.clone()));
        let k2 = LastKittyIndex::<Test>::get();

        assert_ok!(KittiesModule::breed(origin.clone(), k1, k2));
    });
}

#[test]
fn cant_breed_with_same_kitty() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        let account = 1;
        let origin = Origin::signed(account);
        assert_ok!(KittiesModule::create(origin.clone()));
        let k1 = LastKittyIndex::<Test>::get();

        assert_noop!(
            KittiesModule::breed(origin.clone(), k1, k1),
            Error::<Test>::RequireDifferentParent,
        );
    });
}
