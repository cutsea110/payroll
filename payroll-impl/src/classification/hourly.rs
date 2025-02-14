use chrono::NaiveDate;
use log::{debug, trace};
use std::any::Any;

use payroll_domain::{Paycheck, PaymentClassification};

#[derive(Debug, Clone)]
struct TimeCard {
    date: NaiveDate,
    hours: f32,
}
impl TimeCard {
    fn new(date: NaiveDate, hours: f32) -> Self {
        Self { date, hours }
    }
}

#[derive(Debug, Clone)]
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
        trace!("HourlyClassification::calculate_pay_for_timecard called");
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
        trace!("HourlyClassification::calculate_pay called");
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
