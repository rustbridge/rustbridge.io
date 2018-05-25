use chrono::NaiveDateTime;
use failure::{Error, ResultExt};
use form::invite::Invite as InviteForm;
use rocket::request::Form;

#[derive(Queryable, Serialize, Debug)]
pub struct Invite {
    pub id: i32,
    pub workshop_id: i32,
    pub email: String,
    pub attending: bool,
    pub pending: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

use schema::invites;

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "invites"]
pub struct NewInvite<'i> {
    pub workshop_id: i32,
    pub email: &'i str,
}

impl<'i> From<&'i Form<'i, InviteForm>> for NewInvite<'i> {
    fn from(form: &'i Form<'i, InviteForm>) -> Self {
        NewInvite {
            workshop_id: form.get().id() as i32,
            email: form.get().email(),
        }
    }
}

impl<'i> super::Validate for NewInvite<'i> {
    fn validate(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'i> super::Sanitize for NewInvite<'i> {
    fn sanitize(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'i> super::Resource for NewInvite<'i> {
    fn save(&self) -> Result<(), Error> {
        use db;
        use diesel::RunQueryDsl;

        let _ = ::diesel::insert_into(invites::table)
            .values(self)
            .execute(&db::establish_connection());

        Ok(())
    }
    
    fn update(&self) -> Result<(), Error> {
      Ok(())
    }

    fn delete(&self) -> Result<(), Error> {
      Ok(())
    }
}
