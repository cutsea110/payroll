use chrono::{Datelike, NaiveDate, Weekday};
use log::{debug, trace};
use std::any::Any;

use payroll_domain::{Affiliation, MemberId, Paycheck};

#[derive(Debug, Clone)]
struct ServiceCharge {
    date: NaiveDate,
    amount: f32,
}
impl ServiceCharge {
    fn new(date: NaiveDate, amount: f32) -> Self {
        Self { date, amount }
    }
}

#[derive(Debug, Clone)]
pub struct UnionAffiliation {
    member_id: MemberId,
    dues: f32,
    service_charges: Vec<ServiceCharge>,
}
impl UnionAffiliation {
    pub fn new(member_id: MemberId, dues: f32) -> Self {
        Self {
            member_id,
            dues,
            service_charges: vec![],
        }
    }
    pub fn member_id(&self) -> MemberId {
        self.member_id
    }
    pub fn add_service_charge(&mut self, date: NaiveDate, amount: f32) {
        let sc = ServiceCharge::new(date, amount);
        self.service_charges.push(sc);
    }
}
impl Affiliation for UnionAffiliation {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn calculate_deductions(&self, pc: &Paycheck) -> f32 {
        trace!("UnionAffiliation::calculate_deductions called");
        let pay_period = pc.get_pay_period();
        debug!("pay_period: {} - {}", pay_period.start(), pay_period.end());
        let dues_amount = pay_period
            .start()
            .iter_days()
            .take_while(|d| *d <= *pay_period.end())
            .filter(|d| d.weekday() == Weekday::Fri)
            .count() as f32
            * self.dues;
        debug!("dues_amount: {}", dues_amount);
        let service_amount = self
            .service_charges
            .iter()
            .filter(|sc| pay_period.contains(&sc.date))
            .map(|sc| sc.amount)
            .sum::<f32>();
        debug!("service_amount: {}", service_amount);

        dues_amount + service_amount
    }
}
