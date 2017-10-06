use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable, Serialize)]
pub struct FilmFormat {
    pub id: Uuid,
    pub designation: String,
    pub stock_size_value: Option<f64>,
    pub stock_size_unit: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct FilmStock {
    pub id: Uuid,
    pub box_name: String,
    pub box_speed: Option<i32>,
    pub brand_id: Uuid,
    pub film_format_id: Uuid
}
