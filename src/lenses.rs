use rocket_contrib::Template;
use diesel::{JoinDsl, LoadDsl};
use db_conn::DbConn;
use models::brands::Brand;
use models::lenses::Lens;
use models::users::CurrentUser;
use schema::{brands, lenses};
use super::template_contexts::ListResourcesContext;

#[derive(Serialize)]
struct FullLens {
    lens: Lens,
    brand: Brand,
}

#[get("/lenses", format = "text/html")]
fn index(current_user: CurrentUser, conn: DbConn) -> Template {
    let lens_vec = lenses::table
        .inner_join(brands::table)
        .load::<(Lens, Brand)>(&*conn)
        .expect("Error loading film stocks with brands");

    let full_lenses: Vec<FullLens> = lens_vec
        .into_iter()
        .map(|(lens, brand)| {
            FullLens { lens: lens, brand: brand }
        })
        .collect();

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: None,
        name: "Lenses",
        resources: full_lenses,
    };

    Template::render("lenses/index", context)
}
