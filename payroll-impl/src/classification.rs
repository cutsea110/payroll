use chrono::NaiveDate;
use std::any::Any;

use payroll_domain::{Paycheck, PaymentClassification};

#[derive(Debug, Clone)]
pub struct SalariedClassification {
    salary: f32,
}
impl SalariedClassification {
    pub fn new(salary: f32) -> Self {
        Self { salary }
    }
}
impl PaymentClassification for SalariedClassification {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, _pc: &Paycheck) -> f32 {
        self.salary
    }
}

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
        let overtime = (tc.hours - 8.0).max(0.0);
        let straight_time = tc.hours - overtime;
        straight_time * self.hourly_rate + overtime * self.hourly_rate * 1.5
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
        let pay_period = pc.get_pay_period();
        let mut total_pay = 0.0;
        for tc in &self.timecards {
            if pay_period.contains(&tc.date) {
                total_pay += self.calculate_pay_for_timecard(tc);
            }
        }
        total_pay
    }
}

#[derive(Debug, Clone)]
struct SalesReceipt {
    date: NaiveDate,
    amount: f32,
}
impl SalesReceipt {
    fn new(date: NaiveDate, amount: f32) -> Self {
        Self { date, amount }
    }
}

#[derive(Debug, Clone)]
pub struct CommissionedClassification {
    salary: f32,
    commission_rate: f32,
    sales_receipts: Vec<SalesReceipt>,
}
impl CommissionedClassification {
    pub fn new(salary: f32, commission_rate: f32) -> Self {
        Self {
            salary,
            commission_rate,
            sales_receipts: vec![],
        }
    }
    pub fn add_sales_receipt(&mut self, date: NaiveDate, amount: f32) {
        let sr = SalesReceipt::new(date, amount);
        self.sales_receipts.push(sr);
    }
    fn calculate_pay_for_sales_receipt(&self, sr: &SalesReceipt) -> f32 {
        sr.amount * self.commission_rate
    }
}
impl PaymentClassification for CommissionedClassification {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_pay(&self, pc: &Paycheck) -> f32 {
        let mut total_pay = self.salary;
        let pay_period = pc.get_pay_period();
        for sr in &self.sales_receipts {
            if pay_period.contains(&sr.date) {
                total_pay += self.calculate_pay_for_sales_receipt(sr);
            }
        }
        total_pay
    }
}
