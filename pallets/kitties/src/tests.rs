use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

use super::*;

#[test]
fn owned_kitties_can_append_values() {
    new_test_ext().execute_with(|| {
        run_to_block(10);
        assert_eq!(KittiesModule::create(Origin::signed(1),), Ok(()));
    });
}
