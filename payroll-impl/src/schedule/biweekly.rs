use chrono::{Datelike, Days, NaiveDate, Weekday};
use std::{any::Any, ops::RangeInclusive};

use payroll_domain::PaymentSchedule;

#[derive(Debug, Clone, PartialEq)]
pub struct BiweeklySchedule;
impl PaymentSchedule for BiweeklySchedule {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        date.weekday() == Weekday::Fri && date.iso_week().week() % 2 == 0
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        pay_date.checked_sub_days(Days::new(13)).unwrap()..=pay_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_is_pay_date() {
        let bs = BiweeklySchedule;
        assert!(bs.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 10).unwrap()));
        assert!(!bs.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 11).unwrap()));
        assert!(!bs.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 17).unwrap()));
        assert!(!bs.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 23).unwrap()));
        assert!(bs.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 24).unwrap()));
    }

    #[test]
    fn test_pay_period() {
        let bs = BiweeklySchedule;
        let pay_date = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap();
        let pay_period = bs.get_pay_period(pay_date);
        assert_eq!(
            pay_period,
            NaiveDate::from_ymd_opt(2024, 12, 28).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 10).unwrap()
        );
        let pay_date = NaiveDate::from_ymd_opt(2025, 1, 24).unwrap();
        let pay_period = bs.get_pay_period(pay_date);
        assert_eq!(
            pay_period,
            NaiveDate::from_ymd_opt(2025, 1, 11).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 24).unwrap()
        );
    }
}
