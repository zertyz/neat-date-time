//! Constains utility functions for storing dates (either absolute `u32` or relative `u16`),
//! so as to save space when compared to `chrono::NaiveDate` -- up to 2x saving.

/// how many fractional days there are in a year
const LEAP_YEAR_FACTOR: f64 = (400.0*365.0 + 400.0/4.0-400.0/100.0+400.0/400.0) / 400.0;

/// days for each month for non-leap years -- January is #0
const MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// days for each month for leap years -- January is #0
const MONTH_DAYS_LEAP_YEAR: [u32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// year's completed days when each month starts -- January: #0 = 0, meaning: zero completed days for the year at Jan 1st
const DAYS_UP_TO_MONTH_START: [u32; 12] = {
    let mut days_up_to_month_start = [0; 12];
    let mut partial_sum = 0;
    let mut month = 0;
    while month < 12 {
        days_up_to_month_start[month] = partial_sum;
        partial_sum += MONTH_DAYS[month];
        month += 1;
    }
    days_up_to_month_start
};

/// same as [DAYS_UP_TO_MONTH_START], but for leap years
const DAYS_UP_TO_MONTH_START_FOR_LEAP_YEARS: [u32; 12] = {
    let mut days_up_to_month_start_for_leap_years = [0; 12];
    let mut partial_sum = 0;
    let mut month = 0;
    while month < 12 {
        days_up_to_month_start_for_leap_years[month] = partial_sum;
        partial_sum += MONTH_DAYS_LEAP_YEAR[month];
        month += 1;
    }
    days_up_to_month_start_for_leap_years
};
// TODO improve the above 2 consts to the following, if stable Rust would ever allow "const iterators" -- for which we traverse all of it:
// const DAYS_UP_TO_MONTH_START_FOR_LEAP_YEARS: [u32; 12] = [0,1,2,3,4,5,6,7,8,9,10,11]
//     .map(|month0| (1..=month0)
//         .map(|month| MONTH_DAYS_LEAP_YEAR[month-1])
//         .sum()
//     );

/// returns a `u32` to represent a "naive date" (with no timezone consideration) from
/// the given `year`, `month` and `day`.\
/// See [ymd_from_u32()] for the reverse operation.
pub const fn u32_from_ymd(year: u16, month: u8, day: u8) -> u32 {
    let leap_days_since_era_start = ((year-1) / 4 - (year-1) / 100 + (year-1) / 400) as u32;
    let era_days_to_year_start = (leap_days_since_era_start*366) + ((year as u32 - leap_days_since_era_start - 1)*365);

    let year_day = match is_leap_year(year) {
        true  => DAYS_UP_TO_MONTH_START_FOR_LEAP_YEARS,
        false => DAYS_UP_TO_MONTH_START,
    }[month as usize - 1] + (day as u32 - 1);

    era_days_to_year_start + year_day

}

/// returns the `year`, `month` and `day` represented by the give u32 naive `date`.\
/// See [u32_from_ymd()] for the reverse operation.
pub /*const*/ fn ymd_from_u32(date: u32) -> (u16, u8, u8) {
    
    let year0f = date as f64 / LEAP_YEAR_FACTOR;
    let year0 = year0f as u32;
    let leap_days_since_era_start = year0 / 4 - year0 / 100 + year0 / 400;
    let mut year1 = year0 + 1;
    let days_up_to_year_start = (leap_days_since_era_start*366) + ((year1 as u32 - leap_days_since_era_start - 1)*365);

    let year_day = date - days_up_to_year_start;
    let days_to_month_start = if is_leap_year(year1 as u16) {
        DAYS_UP_TO_MONTH_START_FOR_LEAP_YEARS.as_ref()
    } else {
        DAYS_UP_TO_MONTH_START.as_ref()
    };
    let mut month0 = year_day / 31;    // hint to the loop bellow... causing it to loop only once or, on most times, not loop at all
    loop {
        if days_to_month_start[month0 as usize] <= year_day {
            break;
        }
        month0 -= 1;
    }

    let mut day0 = year_day - days_to_month_start[month0 as usize];

    let month_days = if is_leap_year(year1 as u16) {
        MONTH_DAYS_LEAP_YEAR
    } else {
        MONTH_DAYS
    };

    if day0 >= month_days[month0 as usize] {
        day0 -= month_days[month0 as usize];
        month0 += 1;
    }
    if month0 >= 12 {
        month0 -= 12;
        year1 += 1;
    }

    (year1 as u16, month0 as u8 + 1, day0 as u8 + 1)
}

/// returns a human readable "YYYY-MM-DD" date from the given `year`, `month` and `day`
pub fn string_from_ymd(year: u16, month: u8, day: u8) -> String {
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// returns a human readable "YYYY-MM-DD" date from the u32 `date` generated by [u32_from_ymd()]
pub fn string_from_u32(date: u32) -> String {
    let (year, month, day) = ymd_from_u32(date);
    string_from_ymd(year, month, day)
}

/// returns whether there is a "29th feb" day for the given `year` (starting at year #1)
pub const fn is_leap_year(year1: u16) -> bool {
    year1 % 4 == 0 && (year1 % 100 != 0 || year1 % 400 == 0)
}


#[cfg(any(test, feature = "dox"))]
mod tests {
    use super::*;

    /// tests dates to `u32` conversion and vice-versa
    #[cfg_attr(not(feature = "dox"), test)]
    fn naive_date_conversions() {
        let (original_year, original_month, original_day) = (1979, 01, 22);
        let epoch = u32_from_ymd(original_year as u16, original_month as u8, original_day as u8);
        dbg!(epoch);
        let (reconstructed_year, reconstructed_month, reconstructed_day) = ymd_from_u32(epoch);
        assert_eq!((reconstructed_year, reconstructed_month, reconstructed_day), (original_year, original_month, original_day), "naive dates <--> u32 conversions failed");
    }

    /// tests we're able to represent all possible dates in sequence -- from year 1 up to today;\
    /// proves the "sequential / non-skipping" aspect of our dates representation
    #[cfg_attr(not(feature = "dox"), test)]
    fn comprehensive_representation() {
        let mut expected_u32_date = 0;
        for year in 1..=2022 {
            let month_days = if is_leap_year(year) {
                MONTH_DAYS_LEAP_YEAR
            } else {
                MONTH_DAYS
            };
            for month in 1..=12 {
                for day in 1..=month_days[month as usize - 1] as u8 {
                    let observed_u32_date = u32_from_ymd(year, month, day);
                    assert_eq!(observed_u32_date, expected_u32_date, "`u32` encoded date value is wrong for date {} while traversing all possible dates", string_from_ymd(year, month, day));
                    let (reconstituted_year, reconstituted_month, reconstituted_day) = ymd_from_u32(observed_u32_date);
                    assert_eq!((reconstituted_year, reconstituted_month, reconstituted_day),
                               (year, month, day), "reconstituted date is wrong while encoding/decoding all possible dates");
                    expected_u32_date += 1;
                }
            }
        }
    }

    /// attests our date conversions remain feithful to their once encoded values, taking a sample as proof --
    /// we are bound to produce the same results throughout all of 0.1.* version series (at least).\
    /// Note that the mentioned attestation is fully issued when both this and the "sequential / non-skipping"
    /// aspect of our dates representation is given by [comprehensive_representation()]
    #[cfg_attr(not(feature = "dox"), test)]
    fn stable_encoding() {
        let known_ymd_and_u32_dates = [
            (2022, 07, 06, 738341),
            (2022, 05, 23, 738297),
        ];
        for (year, month, day, u32_date) in known_ymd_and_u32_dates {
            let observed_u32_date = u32_from_ymd(year, month, day);
            assert_eq!(observed_u32_date, u32_date, "`u32_from_ymd({}, {}, {})` no longer prduces the same results", year, month, day);
            let (reconstructed_year, reconstructed_month, reconstructed_day) = ymd_from_u32(u32_date);
            assert_eq!((reconstructed_year, reconstructed_month, reconstructed_day), (year, month, day), "`ymd_from_u32({})` no longer prduces the same results", u32_date);
        }
    }

    /// tests our date functions can initialize constants
    #[cfg_attr(not(feature = "dox"), test)]
    fn const_functions() {
        const EPOCH_DATE: u32 = u32_from_ymd(2022, 5, 23);
        assert_eq!((2022, 5, 23), ymd_from_u32(EPOCH_DATE), "const date check failed");
    }


}