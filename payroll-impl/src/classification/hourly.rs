use chrono::NaiveDate;
use log::{debug, trace};
use std::any::Any;

use payroll_domain::{Paycheck, PaymentClassification};

#[derive(Debug, Clone, PartialEq)]
struct TimeCard {
    date: NaiveDate,
    hours: f32,
}
impl TimeCard {
    fn new(date: NaiveDate, hours: f32) -> Self {
        Self { date, hours }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HourlyClassification {
    hourly_rate: f32,
    timecards: Vec<TimeCard>,
}
impl HourlyClassification {
    pub fn new(hourly_rate: f32) -> Self {
        Self {
            hourly_rate,
            timecards: vec![],
        }
    }
    pub fn add_timecard(&mut self, date: NaiveDate, hours: f32) {
        self.timecards.push(TimeCard::new(date, hours));
    }
    fn calculate_pay_for_timecard(&self, tc: &TimeCard) -> f32 {
        trace!("calculate_pay_for_timecard called");
        let overtime = (tc.hours - 8.0).max(0.0);
        debug!("overtime: {}", overtime);
        let straight_time = tc.hours - overtime;
        debug!("straight_time: {}", straight_time);

        (straight_time + overtime * 1.5) * self.hourly_rate
    }
}
impl PaymentClassification for HourlyClassification {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, pc: &Paycheck) -> f32 {
        trace!("calculate_pay called");
        let pay_period = pc.get_pay_period();
        debug!("pay_period: {} - {}", pay_period.start(), pay_period.end());
        let hourly_amount = self
            .timecards
            .iter()
            .filter(|tc| pay_period.contains(&tc.date))
            .fold(0f32, |acc, tc| acc + self.calculate_pay_for_timecard(tc));
        debug!("hourly_amount: {}", hourly_amount);

        hourly_amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_timecard() {
        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2021, 1, 25).unwrap()
                ..=NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        );
        let hc = HourlyClassification::new(10.0);
        let pay = hc.calculate_pay(&pc);
        assert_eq!(pay, 0.0);
    }

    #[test]
    fn test_add_single_timecard() {
        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2021, 1, 25).unwrap()
                ..=NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        );
        let mut hc = HourlyClassification::new(10.0);
        hc.add_timecard(NaiveDate::from_ymd_opt(2021, 1, 25).unwrap(), 8.0);
        let pay = hc.calculate_pay(&pc);
        assert_eq!(pay, 80.0); // 8 * 10
    }

    #[test]
    fn test_add_multiple_timecards() {
        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2021, 1, 25).unwrap()
                ..=NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        );
        let mut hc = HourlyClassification::new(10.0);
        hc.add_timecard(NaiveDate::from_ymd_opt(2021, 1, 25).unwrap(), 8.0);
        hc.add_timecard(NaiveDate::from_ymd_opt(2021, 1, 26).unwrap(), 8.0);
        let pay = hc.calculate_pay(&pc);
        assert_eq!(pay, 160.0); // 8 * 10 + 8 * 10
    }

    #[test]
    fn test_add_outrange_timecards() {
        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2021, 1, 25).unwrap()
                ..=NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        );
        let mut hc = HourlyClassification::new(10.0);
        hc.add_timecard(NaiveDate::from_ymd_opt(2021, 1, 25).unwrap(), 8.0);
        hc.add_timecard(NaiveDate::from_ymd_opt(2021, 1, 26).unwrap(), 8.0);
        hc.add_timecard(NaiveDate::from_ymd_opt(2021, 2, 1).unwrap(), 8.0);
        let pay = hc.calculate_pay(&pc);
        assert_eq!(pay, 160.0); // (8 + 8) * 10
    }

    #[test]
    fn test_add_overtime_timecard() {
        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2021, 1, 25).unwrap()
                ..=NaiveDate::from_ymd_opt(2021, 1, 31).unwrap(),
        );
        let mut hc = HourlyClassification::new(10.0);
        hc.add_timecard(NaiveDate::from_ymd_opt(2021, 1, 25).unwrap(), 10.0);
        let pay = hc.calculate_pay(&pc);
        assert_eq!(pay, 110.0); // (8 + 2 * 1.5) * 10
    }
}
