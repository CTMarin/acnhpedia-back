
#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use crate::libacnh::users;
    use std::{sync::Once, io::Write};

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            let mut file = std::fs::File::create("server-data/users_test.txt").unwrap();
            let _ = file.write_all(b"exists1@uma.es:3xists1:exists1\nexists2@uma.es:3xists2:exists2\nexists3@uma.es:3xists3:exists3\n");
        });
    }

    #[test]
    fn GivenNonExistingUser_WhenWriteUser_ThenUserInFile() {
        initialize();

        let _ = users::register_user("new@uma.es", "new", "p4sswd", "p4sswd");

        let expected_value = true;
        let obtained_value = users::check_user("new@uma.es");

        assert_eq!(expected_value, obtained_value);
    } 

    #[test]
    fn GivenExistingUser_WhenWriteUser_ThenReturnError() {
        initialize();

        let expected_value = 409;
        let obtained_value = users::register_user("exists2@uma.es", "exists2", "p4sswd", "p4sswd").unwrap_err();

        assert_eq!(expected_value, obtained_value);
    } 

    #[test]
    fn GivenExistingUser_WhenCheckUser_ThenReturnTrue() {
        initialize();

        let expected_value = true;
        let obtained_value = users::check_user("exists2@uma.es");
        
        assert_eq!(expected_value, obtained_value);
    }   

    #[test]
    fn GivenNonExistingUser_WhenCheckUser_ThenReturnError() {
        initialize();

        let expected_value = false;
        let obtained_value = users::check_user("not_exists@uma.es");

        assert_eq!(expected_value, obtained_value);
    }

    #[test]
    fn GivenExistingUserAndCorretPassword_WhenLoginUser_ThenReturnOk() {
        initialize();

        let expected_value = 200;
        let obtained_value = users::check_login("exists2@uma.es", "3xists2").unwrap();

        assert_eq!(expected_value, obtained_value);
    }
 
    #[test]
    fn GivenNonExistingUser_WhenLoginUser_ThenReturnErr404() {
        initialize();

        let expected_value = 404;
        let obtained_value = users::check_login("not_exists@uma.es", "n0t_3xists").unwrap_err();

        assert_eq!(expected_value, obtained_value);
    }

    #[test]
    fn GivenExistingUserAndWrongPassword_WhenLoginUser_ThenReturnErr401() {
        initialize();

        let expected_value = 401;
        let obtained_value = users::check_login("exists2@uma.es", "wr0ng_p4ss").unwrap_err();

        assert_eq!(expected_value, obtained_value);
    }
}