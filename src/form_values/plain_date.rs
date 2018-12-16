use chrono::NaiveDate;
// use rocket::http::RawStr;
// use rocket::request::FromFormValue;

pub struct PlainDate(pub NaiveDate);

impl<'v> FromFormValue<'v> for PlainDate {
    // TODO: Change to flash?
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<PlainDate, &'v RawStr> {
        if form_value.len() > 14 {
            return Err(build_error());
        }

        let date_string = form_value
            .percent_decode()
            .map_err(|_| "Unable to decode date string")?;

        let trimmed = date_string.trim();

        let naive_date = try_year_first_with_dashes(trimmed)
            .or_else(|_| try_year_first_with_slashes(trimmed))
            .or_else(|_| try_month_first_with_slashes(trimmed));

        match naive_date {
            Ok(nd) => Ok(PlainDate(nd)),
            Err(e) => {
                println!("error: {:?}", e);
                Err(build_error())
            }
        }
    }
}

fn build_error<'v>() -> &'v RawStr {
    let err_message = "Date must be a US-formatted date, <= 10 characters (ex. 12/31/2017)";

    RawStr::from_str(err_message)
}

fn try_year_first_with_dashes(date_str: &str) -> ::chrono::format::ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

fn try_year_first_with_slashes(date_str: &str) -> ::chrono::format::ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y/%m/%d")
}

fn try_month_first_with_slashes(date_str: &str) -> ::chrono::format::ParseResult<NaiveDate> {
    println!("parsing str: {}", date_str);
    NaiveDate::parse_from_str(date_str, "%m/%d/%Y")
}
