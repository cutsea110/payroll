use chrono::{Datelike, Days, NaiveDate, Weekday};
use std::ops::RangeInclusive;

use payroll_domain::PaymentSchedule;

#[derive(Debug, Clone)]
pub struct WeeklySchedule;
impl PaymentSchedule for WeeklySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        date.weekday() == Weekday::Fri
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        pay_date.checked_sub_days(Days::new(6)).unwrap()..=pay_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_is_pay_date() {
        let ws = WeeklySchedule;
        assert!(ws.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 10).unwrap()));
        assert!(!ws.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 11).unwrap()));
        assert!(ws.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 17).unwrap()));
        assert!(!ws.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 23).unwrap()));
        assert!(ws.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 24).unwrap()));
    }

    #[test]
    fn test_pay_period() {
        let ws = WeeklySchedule;
        let pay_date = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap();
        let pay_period = ws.get_pay_period(pay_date);
        assert_eq!(
            pay_period,
            NaiveDate::from_ymd_opt(2025, 1, 4).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 10).unwrap()
        );
        let pay_date = NaiveDate::from_ymd_opt(2025, 1, 24).unwrap();
        let pay_period = ws.get_pay_period(pay_date);
        assert_eq!(
            pay_period,
            NaiveDate::from_ymd_opt(2025, 1, 18).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 24).unwrap()
        );
    }
}
