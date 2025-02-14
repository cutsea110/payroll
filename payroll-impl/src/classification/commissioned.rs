use chrono::NaiveDate;
use log::{debug, trace};
use std::any::Any;

use payroll_domain::{Paycheck, PaymentClassification};

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
        trace!("CommissionedClassification::calculate_pay called");
        let pay_period = pc.get_pay_period();
        debug!("pay_period: {} - {}", pay_period.start(), pay_period.end());
        let commissioned_amount = self
            .sales_receipts
            .iter()
            .filter(|sr| pay_period.contains(&sr.date))
            .fold(0 as f32, |acc, sr| {
                acc + self.calculate_pay_for_sales_receipt(sr)
            });
        debug!("commissioned_amount: {}", commissioned_amount);

        self.salary + commissioned_amount
    }
}
