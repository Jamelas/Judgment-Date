#[derive(Debug, Copy, Clone)]
struct Date {
    days: i32
}


impl Date {

    // create Date from year/month/day triple
    fn from_ymd(year: i32, month: i32, day: i32) -> Date {

        // count extra days from leap years
        let mut leap_years= 1;
        if year > 0 {
            leap_years += (year - 1) / 400 - (year - 1) / 100 + (year - 1) / 4
        }
        if year <= 0 {
            leap_years = (year) / 400 - (year) / 100 + (year) / 4
        }

        // Number of days from past years
        let days_from_years = year * 365 + leap_years;

        // Number of days from past months in the current year
        let month_days: i32 = convert_months_to_days(month, days_from_years);

        return Date { days: days_from_years + month_days + day - 1 };
    }


    // convert back to year/month/day triple
    fn ymd(&self) -> (i32, i32, i32) {

        // Keep track of days left
        let mut total = self.days;

        //COUNT DAYS FROM PREVIOUS YEARS
        // Extract number of years from Date.days
        let mut count_years = (total as f32) / 365.2425;

        // Skip year 0 (Gregorian calender)
        if count_years < 1.0 { count_years -= 1.0 }

        let mut year = count_years as i32;

        let mut leap_years = 0;
        if year < 0 { year *= 1 };

        if year >= 400 { leap_years = (year / 400) - (year / 100) + (year / 4) }
            else if year >= 100 { leap_years = (year / 4) - (year / 100) }
            else if year >= 4 { leap_years = year / 4 };

        if leap_years < 0 {
            leap_years *= -1;
        }


        // Update the total (number of days to calculate)
        total = total - year * 365 - leap_years;
        if is_a_leap_year(year) { total += 1 }

        // COUNT DAYS FROM PREVIOUS MONTHS IN YEAR
        let mut month: i32 = total;
        if is_a_leap_year(year) {
            match month {
                336..=366 => month = 12,
                306..=335 => month = 11,
                275..=305 => month = 10,
                245..=274 => month = 9,
                214..=244 => month = 8,
                183..=213 => month = 7,
                153..=182 => month = 6,
                122..=152 => month = 5,
                92..=121 => month = 4,
                61..=91 => month = 3,
                32..=60 => month = 2,
                1..=31 => month = 1,
                _ => (),
            }
        } else {
            match month {
                335..=365 => month = 12,
                305..=334 => month = 11,
                274..=304 => month = 10,
                244..=273 => month = 9,
                213..=243 => month = 8,
                182..=212 => month = 7,
                152..=181 => month = 6,
                121..=151 => month = 5,
                91..=120 => month = 4,
                60..=90 => month = 3,
                32..=59 => month = 2,
                1..=31 => month = 1,
                _ => (),
            }
        }

        // COUNT DAYS
        let day =  if year <= 0 {
            total - convert_months_to_days(month, year)
        } else {
            total - convert_months_to_days(month, year)
        };

        return (year, month, day);
    }
}


// Determine the number of days passed in the selected year
fn convert_months_to_days(month: i32, year: i32) -> i32 {

    let i = if is_a_leap_year(year) { 1 } else { 0 };

    // Number of days in this year, prior to the current month
    let mut passed_days: i32 = month;
    match passed_days {
        1 => passed_days = 0,
        2 => passed_days = 31,
        3 => passed_days = 59 + i,
        4 => passed_days = 90 + i,
        5 => passed_days = 120 + i,
        6 => passed_days = 151 + i,
        7 => passed_days = 181 + i,
        8 => passed_days = 212 + i,
        9 => passed_days = 243 + i,
        10 => passed_days = 273 + i,
        11 => passed_days = 304 + i,
        12 => passed_days = 334 + i,
        _ => (),
    }
    return passed_days;
}


// Check if the year is a leap year
fn is_a_leap_year(year: i32) -> bool {
    let mut leap_year = false;
    if year == 0 { return true }
    if year % 4 == 0 { leap_year = true }
    if year % 100 == 0 { leap_year = false }
    if year % 400 == 0 { leap_year = true }
    return leap_year;
}



impl std::ops::Add<i32> for Date {
    type Output = Date;
    fn add(self, other: i32) -> Date {
        Date { days: self.days + other }
    }
}


impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let mut year = self.ymd().0;
        let month = self.ymd().1;
        let day =
            if year < 0 { self.ymd().2 + 1 }
            else { self.ymd().2 };

        let mut string = String::from("");

        // Check if date should be affixed with BC
        if year <= 0 {
            year -= 1;
            year *= -1;
            string.push_str(" BC")
        }
        write!(f, "{}/{}/{}{}", year, month, day, string)
    }
}


fn main() {
    // testing from_ymd; should print Date { days: 738885 }
    println!("{:?}", Date::from_ymd(2022, 12, 31));

    // testing Add and Display
    let date = Date::from_ymd(-1, 12, 31);
    // increase date by 30 days, 60 days, etc.
    for i in 0..20 {
        // first iteration should print 2/12/31 BC
        println!("{}", date + i * 30);
    }
}
