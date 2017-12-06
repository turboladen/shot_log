use rocket_contrib::Template;
use diesel::{ExecuteDsl, ExpressionMethods, FilterDsl, JoinDsl, JoinOnDsl, LoadDsl};
use db_conn::DbConn;
use models::brands::Brand;
use models::lenses::Lens;
use models::user_lenses::{NewUserLens, UserLens, UserLensForm};
use models::users::CurrentUser;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::UUID;
use schema::{brands, lenses, user_lenses};
use super::template_contexts::{EmptyResourceContext, FlashContext, ListResourcesContext};
use uuid::Uuid;

#[derive(Serialize)]
struct FullUserLens {
    user_lens: UserLens,
    lens: Lens,
    brand: Brand,
}

#[get("/user_lenses", format = "text/html")]
fn index(current_user: CurrentUser, flash: Option<FlashMessage>, conn: DbConn) -> Template {
    let flash_context = match flash {
        Some(fm) => Some(FlashContext::new(fm)),
        None => None,
    };

    let data = user_lenses::table
        .inner_join(
            brands::table
                .inner_join(lenses::table.on(lenses::brand_id.eq(brands::id)))
                .on(user_lenses::lens_id.eq(lenses::id)),
        )
        .filter(user_lenses::user_id.eq(&current_user.id))
        .load::<(UserLens, (Brand, Lens))>(&*conn)
        .expect("Error loading user lenses");

    let full_user_lenses: Vec<FullUserLens> = data.into_iter()
        .map(|(uc, (brand, lens))| {
            FullUserLens {
                user_lens: uc,
                lens: lens,
                brand: brand,
            }
        })
        .collect();

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: flash_context,
        name: "My Lenses",
        resources: full_user_lenses,
    };

    Template::render("user_lenses/index", context)
}

#[get("/user_lenses/new")]
fn new(current_user: CurrentUser, flash: Option<FlashMessage>) -> Template {
    let flash_context = match flash {
        Some(fm) => Some(FlashContext::new(fm)),
        None => None,
    };

    let context = EmptyResourceContext {
        current_user: Some(current_user),
        flash: flash_context,
    };

    Template::render("user_lenses/form", context)
}

#[post("/user_lenses", data = "<user_lens_form>")]
fn create(
    current_user: CurrentUser,
    user_lens_form: Form<UserLensForm>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let uc = user_lens_form.get();
    let user_id: Uuid = current_user.id;
    let lens_id: Uuid = *uc.lens_id;

    let new_uc = NewUserLens {
        user_id: user_id,
        lens_id: lens_id,
        serial_number: uc.serial_number.clone(),
    };

    match ::diesel::insert_into(user_lenses::table)
        .values(&new_uc)
        .execute(&*conn)
    {
        Ok(_) => Ok(Flash::success(Redirect::to("/user_lenses"), "Added")),
        Err(err) => Err(Flash::error(
            Redirect::to("/user_lenses/new"),
            err.to_string(),
        )),
    }
}

#[delete("/user_lenses/<id>")]
fn destroy(
    current_user: CurrentUser,
    id: UUID,
    conn: DbConn,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    use schema::user_lenses::dsl::id as user_lens_id;
    use schema::user_lenses::dsl::user_id;

    let result = ::diesel::delete(
        user_lenses::table
            .filter(user_lens_id.eq(*id))
            .filter(user_id.eq(current_user.id)),
    ).execute(&*conn);

    match result {
        Ok(_uc) => Ok(Flash::success(Redirect::to("/user_lenses"), "Removed!")),
        Err(err) => Err(Flash::error(Redirect::to("/user_lenses"), err.to_string())),
    }
}
