table! {
    brands (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    cameras (id) {
        id -> Uuid,
        model -> Varchar,
        brand_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    film_formats (id) {
        id -> Uuid,
        designation -> Varchar,
        stock_size_value -> Nullable<Float8>,
        stock_size_unit -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    film_stocks (id) {
        id -> Uuid,
        box_name -> Varchar,
        box_speed -> Nullable<Int4>,
        brand_id -> Uuid,
        film_format_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    lenses (id) {
        id -> Uuid,
        model -> Varchar,
        focal_length_min_value -> Float8,
        focal_length_min_unit -> Varchar,
        focal_length_max_value -> Nullable<Float8>,
        focal_length_max_unit -> Nullable<Varchar>,
        aperture_max -> Float8,
        aperture_min -> Nullable<Float8>,
        element_count -> Nullable<Int4>,
        group_count -> Nullable<Int4>,
        filter_thread_diameter_value -> Nullable<Float8>,
        filter_thread_diameter_unit -> Nullable<Varchar>,
        notes -> Nullable<Varchar>,
        brand_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    rolls (id) {
        id -> Uuid,
        film_stock_id -> Uuid,
        user_camera_id -> Uuid,
        display_id -> Varchar,
        loaded_at -> Date,
        finished_at -> Nullable<Date>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    shots (id) {
        id -> Uuid,
        roll_id -> Uuid,
        exposure_number -> Varchar,
        user_lens_id -> Nullable<Uuid>,
        shot_at -> Nullable<Timestamptz>,
        focal_length_value -> Nullable<Float8>,
        focal_length_unit -> Nullable<Varchar>,
        aperture -> Nullable<Float8>,
        notes -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user_cameras (id) {
        id -> Uuid,
        user_id -> Uuid,
        camera_id -> Uuid,
        roll_prefix -> Varchar,
        serial_number -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    user_lenses (id) {
        id -> Uuid,
        user_id -> Uuid,
        lens_id -> Uuid,
        serial_number -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(cameras -> brands (brand_id));
joinable!(film_stocks -> brands (brand_id));
joinable!(film_stocks -> film_formats (film_format_id));
joinable!(lenses -> brands (brand_id));
joinable!(rolls -> film_stocks (film_stock_id));
joinable!(rolls -> user_cameras (user_camera_id));
joinable!(shots -> rolls (roll_id));
joinable!(shots -> user_lenses (user_lens_id));
joinable!(user_cameras -> cameras (camera_id));
joinable!(user_cameras -> users (user_id));
joinable!(user_lenses -> lenses (lens_id));
joinable!(user_lenses -> users (user_id));

allow_tables_to_appear_in_same_query!(
    brands,
    cameras,
    film_formats,
    film_stocks,
    lenses,
    rolls,
    shots,
    user_cameras,
    user_lenses,
    users,
);
