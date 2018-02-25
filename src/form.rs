use rocket::request::{FromForm, FormItems};
use std::collections::HashMap;
use rocket::http::RawStr;
use Login;
use ForgotUsername; 
use Workshop;

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

//TODO fix this
impl<'f> FromForm<'f> for Workshop {
    type Error = ();
    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<Workshop, ()> {
        let map: HashMap<_, _> = items.collect();
       
        let date: &RawStr = "workshop-date".into();
        let date = match map.get(date) {
            Some(date) => date.to_string(),
            None => return Err(()),
        };
        let start_time: &RawStr = "workshop-starttime".into();
        let start_time = match map.get(start_time) {
            Some(start_time) => start_time.to_string(),
            None => return Err(()),
        };
        let end_time: &RawStr = "workshop-endtime".into();
        let end_time = match map.get(end_time) {
            Some(end_time) => end_time.to_string(),
            None => return Err(()),
        };
        let desc: &RawStr = "workshop-description".into();
        let desc = match map.get(desc) {
            Some(desc) => desc.to_string(),
            None => return Err(()),
        };
        let name: &RawStr = "workshop-name".into();
        let name = match map.get(name) {
            Some(name) => name.to_string(),
            None => return Err(()),
        };
        let reg_link: &RawStr = "workshop-reg-link".into();
        let reg_link = match map.get(reg_link) {
            Some(reg_link) => reg_link.to_string(),
            None => return Err(()),
        };
        let workshop = Workshop {
            date,
            end_time,
            start_time,
            reg_link,
            desc,
            name
        };
        Ok(workshop)
    }
}



