use crate::libacnh::obtained;
use std::{sync::Once, io::Write};

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        let mut file = std::fs::File::create("server-data/users/test.txt").unwrap();
        let _ = file.write_all(b"fish:1\nhouseware:5\nfossils:13\n");
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
fn GivenNotObtainedCard_WhenObtainsIt_ThenWriteOnUserFile() {
    initialize();

    let user: &str = "test";
    let card_type: &str = "fish";
    let id: i32 = 3;

    let _ = obtained::add_obtained_card(user, card_type, id);

    let expected_value: bool = true;
    let obtained_value: bool = obtained::check_obtained_card(user, card_type, id);

    assert_eq!(expected_value, obtained_value);
}

#[test]
fn GivenObtainedCard_WhenObtainsIt_ThenDoNotWriteOnFile() {
    initialize();

    let user: &str = "test";
    let card_type: &str = "fish";
    let id: i32 = 3;

    let _ = obtained::add_obtained_card(user, card_type, id);

    let expected_value: bool = true;
    let obtained_value: bool = obtained::check_obtained_card(user, card_type, id);

    assert_eq!(expected_value, obtained_value);
}
