use chrono::NaiveDateTime;
use failure::{Error, ResultExt, err_msg};
use form::workshop::Workshop as WorkshopForm;
use rocket::request::Form;

#[derive(Queryable, Serialize, Debug)]
pub struct Workshop {
    pub id: i32,
    pub name: String,
    pub organizer: i32,
    pub description: String,
    pub location: String,
    pub date: NaiveDateTime,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub private: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

use schema::workshops;

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "workshops"]
pub struct NewWorkshop<'ws> {
    pub name: &'ws str,
    pub organizer: Option<i32>,
    pub description: &'ws str,
    pub location: &'ws str,
    #[column_name = "event_date"]
    pub date: NaiveDateTime,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub private: bool,
}

impl<'ws> From<&'ws Form<'ws, WorkshopForm>> for NewWorkshop<'ws> {
    fn from(form: &'ws Form<'ws, WorkshopForm>) -> Self {
        NewWorkshop {
            name: form.get().name(),
            organizer: None,
            description: form.get().description(),
            location: form.get().location(),
            date: form.get().date(),
            start_time: form.get().start_time(),
            end_time: form.get().end_time(),
            private: *form.get().private(),
        }
    }
}

impl<'ws> super::Validate for NewWorkshop<'ws> {
    fn validate(&mut self) -> Result<(), Error> {
      if self.organizer == None {
        bail!("no organizer specified");
      }
      Ok(())
    }
}

impl<'ws> super::Sanitize for NewWorkshop<'ws> {
    fn sanitize(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'ws> super::Resource for NewWorkshop<'ws> {
    fn save(&self) -> Result<(), Error> {
        use db;
        use diesel::RunQueryDsl;

        let _ = ::diesel::insert_into(workshops::table)
            .values(self)
            .execute(&db::establish_connection());

        Ok(())
    }

    fn update(&self) -> Result<(), Error> {
      use db;
      use diesel::RunQueryDsl;

      Ok(())
    }

    fn delete(&self) -> Result<(), Error> {
      use db;
      use diesel::RunQueryDsl;

      Ok(())
    }
}
