use rocket::request::{FromForm, FormItems};
use std::collections::HashMap;
use rocket::http::RawStr;
use Login;
use ForgotUsername; 

impl<'f> FromForm<'f> for ForgotUsername {
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<ForgotUsername, ()> {
        let map: HashMap<_, _> = items.collect();

        let login: &RawStr = "login".into();

        // super ugly, there is two buttons in the reset form
        // don't know how to seperate the clicks in a nice way..
        // TODO: fix this!
        let login = match map.get(login) {
            Some(login) => true,
            None => false
        };
        
        let email: &RawStr = "email".into();

        let email = match map.get(email) {
            Some(email) => email.to_string(),
            None => "".to_string(),
        };

        let forgot_username = ForgotUsername {
            email,
            login,
        };

        Ok(forgot_username)
    }
}

impl<'f> FromForm<'f> for Login {
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<Login, ()> {
        let map: HashMap<_, _> = items.collect();
        let email: &RawStr = "email".into();

        let email = match map.get(email) {
            Some(email) => email.url_decode().map_err(|_| ())?,
            None => return Err(()),
        };

        let password: &RawStr = "password".into();
        let password = match map.get(password) {
            Some(password) => password.to_string(),
            None => return Err(()),
        };

        let login = Login {
            email,
            password, 
        };
        Ok(login)
    }
}
