//! Constains utility functions for storing dates (either absolute `u32` or relative `u16`),
//! so as to save space when compared to `chrono::NaiveDate` -- up to 2x saving.


/// returns an `u32` to represent a "naive date" (with no timezone consideration) from
/// the given `year`, `month` and `day`.\
/// See [ymd_from_u32()] for the reverse operation.
pub fn u32_from_ymd(year: u16, month: u8, day: u8) -> u32 {
    ((year as u32) << 9) | ((month as u32) << 5) | day as u32
}

/// returns the `year`, `month` and `day` represented by the give u32 naive `date`.\
/// See [u32_from_ymd()] for the reverse operation.
pub fn ymd_from_u32(date: u32) -> (u16, u8, u8) {
    (
        ( (date & (0xFFFF<<9)) >> 9 ) as u16,
        ( (date & (0xF<<5)) >> 5 )    as u8,
        (  date & 0x1F )              as u8
    )
}

/// returns a human readable "YYYY-MM-DD" date from the u32 `date` generated by [u32_from_ymd()]
pub fn string_from_u32(date: u32) -> String {
    let (year, month, day) = ymd_from_u32(date);
    format!("{:04}-{:02}-{:02}", year, month, day)
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

}