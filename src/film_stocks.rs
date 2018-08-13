use db_conn::DbConn;
use diesel::{QueryDsl, RunQueryDsl};
use models::brands::Brand;
use models::film_formats::{for_display, FilmFormat};
use models::film_stocks::{FilmStock, SerializableFilmStock};
use models::users::CurrentUser;
use rocket_contrib::{Json, Template};
use schema::{brands, film_formats, film_stocks};
use serializables::DropDown;
use super::template_contexts::ListResourcesContext;
use uuid::Uuid;

#[get("/film_stocks", format = "text/html")]
fn index(current_user: CurrentUser, conn: DbConn) -> Template {
    let joined_film_stocks = film_stocks::table
        .inner_join(brands::table)
        .inner_join(film_formats::table)
        .load::<(FilmStock, Brand, FilmFormat)>(&*conn)
        .expect("Error loading film stocks with associations");

    let serializable_film_stocks = joined_film_stocks
        .into_iter()
        .map(|(fs, b, ff)| {
            SerializableFilmStock {
                film_stock: fs,
                brand: b,
                film_format: ff,
            }
        })
        .collect();

    let context = ListResourcesContext {
        current_user: Some(current_user),
        flash: None,
        name: "Film Stocks",
        resources: serializable_film_stocks,
    };

    Template::render("film_stocks/index", context)
}

#[get("/film_stocks", format = "application/json")]
fn drop_down(_current_user: CurrentUser, conn: DbConn) -> Json<Vec<DropDown>> {
    let joined_film_stocks = film_stocks::table
        .inner_join(brands::table)
        .inner_join(film_formats::table)
        .select((
            film_stocks::id,
            film_stocks::box_name,
            film_stocks::box_speed,
            brands::name,
            film_formats::designation,
            film_formats::stock_size_value,
            film_formats::stock_size_unit,
        ))
        .load::<(
            Uuid,
            String,
            Option<i32>,
            String,
            String,
            Option<f64>,
            Option<String>,
        )>(&*conn)
        .expect("Error loading film stocks with associations");

    let film_stock_drop_downs: Vec<DropDown> = joined_film_stocks
        .into_iter()
        .map(
            |(id, box_name, box_speed, brand_name, ff_designation, ff_ss_value, ff_ss_unit)| {
                let mut label = format!("{} {}", brand_name, box_name);

                box_speed.and_then(|bs| Some(label.push_str(&format!(" {}", bs))));

                let ff_display = for_display(&ff_designation, &ff_ss_value, &ff_ss_unit);
                label.push_str(&format!(" ({}/{})", ff_designation, ff_display));

                DropDown {
                    id: id,
                    label: label,
                }
            },
        )
        .collect();

    Json(film_stock_drop_downs)
}
