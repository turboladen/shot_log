use ::actix::prelude::*;
use ::actix_web::*;
use ::actix_web::error::ErrorInternalServerError;
use crate::db_conn::DbExecutor;
use crate::models::brands::Brand;
use diesel::prelude::*;

pub(crate) struct GetBrands;

impl Message for GetBrands {
    type Result = Result<Vec<Brand>, Error>;
}

impl Handler<GetBrands> for DbExecutor {
    type Result = Result<Vec<Brand>, Error>;

    fn handle(&mut self, _msg: GetBrands, _: &mut Self::Context) -> Self::Result {
        use crate::schema::brands::table as brands;

        let conn: &PgConnection = &self.0.get().unwrap();

        brands.load(&*conn)
            .map_err(|e| ErrorInternalServerError(e))
    }
}
