use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn registration_and_unregistration_should_work() {
    new_test_ext().execute_with(|| {
        assert_eq!(ValidatorRegistry::mission_of(1), 0);
        assert_eq!(ValidatorRegistry::validators(10), Vec::<u64>::new());
        assert_ok!(ValidatorRegistry::register(Origin::signed(1), 10));
        assert_eq!(ValidatorRegistry::mission_of(1), 10);
        assert_eq!(ValidatorRegistry::validators(10), vec![1]);
        assert_ok!(ValidatorRegistry::register(Origin::signed(2), 10));
        assert_eq!(ValidatorRegistry::mission_of(2), 10);
        assert_eq!(ValidatorRegistry::validators(10), vec![1, 2]);
        assert_ok!(ValidatorRegistry::unregister(Origin::signed(2)));
        assert_eq!(ValidatorRegistry::validators(10), vec![1]);
        assert_ok!(ValidatorRegistry::unregister(Origin::signed(1)));
        assert_eq!(ValidatorRegistry::mission_of(1), 0);
        assert_eq!(ValidatorRegistry::validators(10), Vec::<u64>::new());
    });
}

#[test]
fn re_registration_should_not_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(ValidatorRegistry::register(Origin::signed(1), 10));
        assert_eq!(ValidatorRegistry::mission_of(1), 10);
        assert_eq!(ValidatorRegistry::validators(10), vec![1]);
        assert_noop!(
            ValidatorRegistry::register(Origin::signed(1), 11),
            Error::<Test>::AlreadyRegistered
        );
        assert_eq!(ValidatorRegistry::validators(10), vec![1]);
    });
}

#[test]
fn registration_with_invalid_mission_id_should_not_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ValidatorRegistry::register(Origin::signed(1), 0),
            pallet_mission_tokens::Error::<Test>::InvalidMissionTokenId
        );
        assert_noop!(
            ValidatorRegistry::register(Origin::signed(1), 13),
            pallet_mission_tokens::Error::<Test>::InvalidMissionTokenId
        );
    });
}
