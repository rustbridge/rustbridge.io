use chrono::NaiveDateTime;
use failure::{Error, ResultExt};

#[derive(Identifiable, Queryable, Serialize, Debug)]
#[table_name = "invite_confirmations"]
pub struct InviteConfirmationModel {
    pub id: i32,
    pub code: String,
    pub invite_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

use schema::invite_confirmations;

#[derive(Insertable, Deserialize, AsChangeset, Debug)]
#[table_name = "invite_confirmations"]
pub struct InviteConfirmation<'ic> {
    pub code: &'ic str,
    pub invite_id: i32,
}

impl<'ic> super::Validate for InviteConfirmation<'ic> {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'ic> super::Sanitize for InviteConfirmation<'ic> {
    fn sanitize(&self) -> Result<(), Error> {
        Ok(())
    }
}

use db;

impl<'ic> super::Resource for InviteConfirmation<'ic> {
    type Model = InviteConfirmationModel;

    fn create(&self) -> Result<Option<i32>, Error> {
        use diesel::RunQueryDsl;
        use schema::invite_confirmations::dsl::*;

        let _ = ::diesel::insert_into(invite_confirmations)
            .values(self)
            .execute(&db::establish_connection());

        let model_id = Self::read_all()
            .unwrap()
            .iter()
            .filter(|ic| ic.code == self.code)
            .collect::<Vec<&Self::Model>>()[0]
            .id;

        Ok(Some(model_id))
    }

    fn read_all() -> Result<Vec<Self::Model>, Error> {
        use diesel::prelude::*;
        use schema::invite_confirmations::dsl::*;

        let items: Vec<Self::Model> =
            invite_confirmations.get_results(&db::establish_connection())?;

        Ok(items)
    }

    fn read_one(model_id: usize) -> Result<Self::Model, Error> {
        use diesel::prelude::*;
        use schema::invite_confirmations::dsl::*;

        let item: Self::Model = invite_confirmations
            .filter(id.eq(model_id as i32))
            .get_result(&db::establish_connection())?;

        Ok(item)
    }

    fn update(&self, model_id: usize) -> Result<(), Error> {
        use diesel::prelude::*;
        use schema::invite_confirmations::dsl::*;

        let pending_invite_code = Self::read_one(model_id)?;
        ::diesel::update(&pending_invite_code)
            .set(self)
            .execute(&db::establish_connection())?;

        Ok(())
    }

    fn delete(model_id: usize) -> Result<(), Error> {
        use diesel::prelude::*;

        let confirmed_invite_code = Self::read_one(model_id)?;
        ::diesel::delete(&confirmed_invite_code).execute(&db::establish_connection())?;

        Ok(())
    }
}
