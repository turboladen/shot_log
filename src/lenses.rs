use rocket_contrib::Template;
use diesel::{JoinDsl, LoadDsl};
use db_conn::DbConn;
use models::brand::Brand;
use models::lens::Lens;
use models::user::CurrentUser;
use schema::{brands, lenses};

#[derive(Serialize)]
struct TemplateContext<'a> {
    current_user: CurrentUser,
    name: &'a str,
    lenses: Vec<FullLens>
}

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

    let context = TemplateContext {
        current_user: current_user,
        name: "Lenses",
        lenses: full_lenses,
    };

    Template::render("lenses/index", context)
}
