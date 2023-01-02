use chrono::{DateTime, Local, TimeZone};
use chrono_tz::Tz::Asia__Tokyo;

mod table;
use table::EraItem;

// to_era return era name from year.
pub fn to_era(year: i32) -> Option<&'static str> {
    for i in (0..table::ERAS.len()).rev() {
        if table::ERAS[i].year <= year {
            return Some(table::ERAS[i].name);
        };
    }
    return None;
}

// to_era_from_time return era name from DateTime.
pub fn to_era_from_time(date_time: DateTime<Local>) -> Option<&'static str> {
    for i in (0..table::ERAS.len()).rev() {
        let tz = Asia__Tokyo;
        let et = tz
            .with_ymd_and_hms(
                table::ERAS[i].year,
                table::ERAS[i].month,
                table::ERAS[i].day,
                0,
                0,
                0,
            )
            .unwrap();
        if date_time.gt(&et) {
            return Some(table::ERAS[i].name);
        };
    }
    return None;
}

// FindEra return EraItem.
pub fn find(date_time: DateTime<Local>) -> Option<&'static EraItem> {
    for i in (0..table::ERAS.len()).rev() {
        let tz = Asia__Tokyo;
        let et = tz
            .with_ymd_and_hms(
                table::ERAS[i].year,
                table::ERAS[i].month,
                table::ERAS[i].day,
                0,
                0,
                0,
            )
            .unwrap();
        if date_time.gt(&et) {
            return Some(&table::ERAS[i]);
        };
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        year: i32,
        era: &'static str,
    }

    const TESTCASES: [TestCase; 10] = [
        TestCase {
            year: 1900,
            era: "明治",
        },
        TestCase {
            year: 1911,
            era: "明治",
        },
        TestCase {
            year: 1912,
            era: "大正",
        },
        TestCase {
            year: 1925,
            era: "大正",
        },
        TestCase {
            year: 1926,
            era: "昭和",
        },
        TestCase {
            year: 1988,
            era: "昭和",
        },
        TestCase {
            year: 1989,
            era: "平成",
        },
        TestCase {
            year: 2016,
            era: "平成",
        },
        TestCase {
            year: 2019,
            era: "令和",
        },
        TestCase {
            year: 2039,
            era: "令和",
        },
    ];

    #[test]
    fn test_to_era() {
        for test_case in TESTCASES {
            let result: &str = to_era(test_case.year).unwrap();
            assert_eq!(result, test_case.era);
        }
    }

    #[test]
    fn test_now() {
        let now = Local::now();
        let result: &str = to_era_from_time(now).unwrap();
        assert_eq!(result, "令和");
    }

    #[test]
    fn test_find_found() {
        let now = Local::now();
        let result: &EraItem = find(now).unwrap();
        assert_eq!(result.name, "令和");
    }

    #[test]
    fn test_find_not_found() {
        let before_era = Local.with_ymd_and_hms(640, 1, 1, 0, 0, 0).unwrap();
        // it fails. chrono may not be able to handle 1624/2/30 (寛永 established date)
        let result: Option<&'static EraItem> = find(before_era);
        assert!(result.is_none());
    }
}
