use chrono::NaiveDateTime;
use failure::{err_msg, Error, ResultExt};
use form::workshop::Workshop as WorkshopForm;
use rocket::request::Form;

/// # WorkshopModel
///
/// WorkshopModel represents an entry in the workshops table
#[derive(Identifiable, Queryable, Serialize, Debug)]
#[table_name = "workshops"]
pub struct WorkshopModel {
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

/// Convert a WorkshopForm into a Workshop
impl<'ws> From<&'ws Form<'ws, WorkshopForm>> for Workshop<'ws> {
    fn from(form: &'ws Form<'ws, WorkshopForm>) -> Self {
        Workshop {
            name: Some(form.get().name()),
            organizer: None,
            description: Some(form.get().description()),
            location: Some(form.get().location()),
            date: Some(form.get().date()),
            start_time: Some(form.get().start_time()),
            end_time: Some(form.get().end_time()),
            private: Some(*form.get().private()),
        }
    }
}

use schema::workshops;

#[derive(Insertable, Deserialize, AsChangeset, Debug)]
#[table_name = "workshops"]
pub struct Workshop<'wu> {
    pub name: Option<&'wu str>,
    pub organizer: Option<i32>,
    pub description: Option<&'wu str>,
    pub location: Option<&'wu str>,
    #[column_name = "event_date"]
    pub date: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub private: Option<bool>,
}

impl<'ws> super::Validate for Workshop<'ws> {
    fn validate(&self) -> Result<(), Error> {
        if self.organizer == None {
            bail!("no organizer specified");
        }
        Ok(())
    }
}

impl<'ws> super::Sanitize for Workshop<'ws> {
    fn sanitize(&self) -> Result<(), Error> {
        Ok(())
    }
}

use db;

impl<'ws> super::Resource for Workshop<'ws> {
    type Model = WorkshopModel;

    fn create(&self) -> Result<Option<i32>, Error> {
        use super::{Sanitize, Validate};
        use diesel::RunQueryDsl;
        use schema::workshops::dsl::*;

        self.validate()?;
        self.sanitize()?;

        let _ = ::diesel::insert_into(workshops)
            .values(self)
            .execute(&db::establish_connection())?;

        Ok(None)
    }

    fn read_all() -> Result<Vec<Self::Model>, Error> {
        use diesel::prelude::*;
        use schema::workshops::dsl::*;

        let items: Vec<Self::Model> = workshops.get_results(&db::establish_connection())?;

        Ok(items)
    }

    fn read_one(model_id: usize) -> Result<Self::Model, Error> {
        use diesel::prelude::*;
        use schema::workshops::dsl::*;

        let item: Self::Model = workshops
            .filter(id.eq(model_id as i32))
            .get_result(&db::establish_connection())?;

        Ok(item)
    }

    fn update(&self, model_id: usize) -> Result<(), Error> {
        use diesel::prelude::*;
        use model::{Sanitize, Validate};
        use schema::workshops::dsl::*;

        self.validate()?;
        self.sanitize()?;

        let existing_workshop = Self::read_one(model_id)?;
        ::diesel::update(&existing_workshop)
            .set(self)
            .execute(&db::establish_connection())?;

        Ok(())
    }

    fn delete(&self, model_id: usize) -> Result<(), Error> {
        use diesel::prelude::*;

        let existing_workshop = Self::read_one(model_id)?;
        ::diesel::delete(&existing_workshop).execute(&db::establish_connection())?;

        Ok(())
    }
}
