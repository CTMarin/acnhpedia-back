use crate::libacnh::obtained;
use std::{sync::Once, io::Write};

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        let mut file = std::fs::File::create("server-data/users/test.txt").unwrap();
        let _ = file.write_all(b"fish:1\nhouseware:5\nfossils:13\nvillager:7\nfurniture:4\n");
    });
}

#[test]
fn GivenObtainedCard_WhenCheckIt_ThenReturnTrue() {
    initialize();

    let user: &str = "test";
    let card_type: &str = "fish";
    let id: i32 = 1;

    obtained::check_obtained_card(user, card_type, id);

    let expected_value: bool = true;
    let obtained_value: bool = obtained::check_obtained_card(user, card_type, id);

    assert_eq!(expected_value, obtained_value);
}

#[test]
fn GivenNotObtainedCardDifferentId_WhenCheckIt_ThenReturnFalse() {
    initialize();

    let user: &str = "test";
    let card_type: &str = "fish";
    let id: i32 = 3;

    obtained::check_obtained_card(user, card_type, id);

    let expected_value: bool = false;
    let obtained_value: bool = obtained::check_obtained_card(user, card_type, id);

    assert_eq!(expected_value, obtained_value);
}

#[test]
fn GivenNotObtainedCardDifferentCardType_WhenCheckIt_ThenReturnFalse() {
    initialize();

    let user: &str = "test";
    let card_type: &str = "houseware";
    let id: i32 = 13;

    obtained::check_obtained_card(user, card_type, id);

    let expected_value: bool = false;
    let obtained_value: bool = obtained::check_obtained_card(user, card_type, id);

    assert_eq!(expected_value, obtained_value);
}

#[test]
fn GivenNotExistingUser_WhenCheckACardObtained_ThenReturnFalse() {
    initialize();

    let user: &str = "notexist";
    let card_type: &str = "fossils";
    let id: i32 = 13;

    obtained::check_obtained_card(user, card_type, id);

    let expected_value: bool = false;
    let obtained_value: bool = obtained::check_obtained_card(user, card_type, id);

    assert_eq!(expected_value, obtained_value);
}


#[test]
fn GivenNotObtainedCard_WhenObtainsIt_ThenReturnCode200() {
    initialize();

    let user: &str = "test";
    let card_type: &str = "fish";
    let id: i32 = 3;

    let expected_value: i32 = 200;
    let obtained_value: i32 = obtained::add_obtained_card(user, card_type, id).unwrap();

    assert_eq!(expected_value, obtained_value);
    assert!(obtained::check_obtained_card(user, card_type, id));
}

#[test]
fn GivenObtainedCard_WhenObtainsIt_ThenReturnError409() {
    initialize();

    let user: &str = "test";
    let card_type: &str = "furniture";
    let id: i32 = 4;

    let expected_value: i32 = 409;
    let obtained_value: i32 = obtained::add_obtained_card(user, card_type, id).unwrap_err();

    assert_eq!(expected_value, obtained_value);
}

#[test]
fn GivenObtainedCard_WhenRemoveFromObtained_ThenRemovesCardFromDB() {
    initialize();

    let user: &str = "test";
    let card_type: &str = "fossils";
    let id: i32 = 13;

    let expected_value: i32 = 200;
    let obtained_value: i32 = obtained::remove_obtained_card(user, card_type, id).unwrap();

    assert_eq!(expected_value, obtained_value);
    assert!(!obtained::check_obtained_card(user, card_type, id));
}

#[test]
fn GivenNotObtainedCard_WhenRemoveFromObtained_ThenReturns404() {
    initialize();

    let user: &str = "test";
    let card_type: &str = "fish";
    let id: i32 = 99;

    let expected_value: i32 = 404;
    let obtained_value: i32 = obtained::remove_obtained_card(user, card_type, id).unwrap_err();

    assert_eq!(expected_value, obtained_value);
}
