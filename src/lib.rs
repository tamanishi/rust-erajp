use chrono::{DateTime, Local, TimeZone};
use chrono_tz::Tz::Asia__Tokyo;

mod table;
use table::EraItem;
use table::ERAS;

// to_era returns era name from year.
pub fn to_era(year: i32) -> Option<&'static str> {
    for era in ERAS.iter().rev() {
        if era.year <= year {
            return Some(era.name);
        }
    }
    return None;
}

// to_era_from_time returns era name from DateTime.
pub fn to_era_from_time(date_time: DateTime<Local>) -> Option<&'static str> {
    for era in ERAS.iter().rev() {
        let tz = Asia__Tokyo;
        let et = tz
            .with_ymd_and_hms(era.year, era.month, era.day, 0, 0, 0)
            .unwrap();
        if date_time.gt(&et) {
            return Some(era.name);
        }
    }
    return None;
}

// find returns EraItem.
pub fn find(date_time: DateTime<Local>) -> Option<&'static EraItem> {
    for era in ERAS.iter().rev() {
        let tz = Asia__Tokyo;
        let et = tz
            .with_ymd_and_hms(era.year, era.month, era.day, 0, 0, 0)
            .unwrap();
        if date_time.gt(&et) {
            return Some(&era);
        }
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
    fn test_to_era_too_old() {
        let result: Option<&str> = to_era(640);
        assert!(result.is_none());
    }

    #[test]
    fn test_to_era_from_time_now() {
        let now = Local::now();
        let result: &str = to_era_from_time(now).unwrap();
        assert_eq!(result, "令和");
    }

    #[test]
    fn test_to_era_from_time_too_old() {
        let before_era = Local.with_ymd_and_hms(640, 1, 1, 0, 0, 0).unwrap();
        let result: Option<&str> = to_era_from_time(before_era);
        assert!(result.is_none());
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
        let result: Option<&'static EraItem> = find(before_era);
        assert!(result.is_none());
    }
}
