use chrono::{Datelike, Days, NaiveDate};
use std::{any::Any, ops::RangeInclusive};

use payroll_domain::PaymentSchedule;

#[derive(Debug, Clone, PartialEq)]
pub struct MonthlySchedule;
impl MonthlySchedule {
    pub fn is_last_day_of_month(&self, date: NaiveDate) -> bool {
        date.month() != date.checked_add_days(Days::new(1)).unwrap().month()
    }
}
impl PaymentSchedule for MonthlySchedule {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        self.is_last_day_of_month(date)
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        pay_date.with_day(1).unwrap()..=pay_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_pay_date() {
        let ms = MonthlySchedule;
        assert!(ms.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap()));
        assert!(!ms.is_pay_date(NaiveDate::from_ymd_opt(2025, 1, 30).unwrap()));
        assert!(!ms.is_pay_date(NaiveDate::from_ymd_opt(2028, 2, 28).unwrap()));
        assert!(ms.is_pay_date(NaiveDate::from_ymd_opt(2028, 2, 29).unwrap()));
    }

    #[test]
    fn test_pay_period() {
        let ms = MonthlySchedule;
        let pay_date = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
        let pay_period = ms.get_pay_period(pay_date);
        assert_eq!(
            pay_period,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 15).unwrap()
        );
        let pay_date = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();
        let pay_period = ms.get_pay_period(pay_date);
        assert_eq!(
            pay_period,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 31).unwrap()
        );
        let pay_date = NaiveDate::from_ymd_opt(2028, 2, 20).unwrap();
        let pay_period = ms.get_pay_period(pay_date);
        assert_eq!(
            pay_period,
            NaiveDate::from_ymd_opt(2028, 2, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2028, 2, 20).unwrap()
        );
        let pay_date = NaiveDate::from_ymd_opt(2028, 2, 29).unwrap();
        let pay_period = ms.get_pay_period(pay_date);
        assert_eq!(
            pay_period,
            NaiveDate::from_ymd_opt(2028, 2, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2028, 2, 29).unwrap()
        );
    }
}
