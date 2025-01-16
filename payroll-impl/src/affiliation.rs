use chrono::{Datelike, NaiveDate, Weekday};
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
        let mut total_deductions = 0.0;
        let pay_period = pc.get_pay_period();
        for d in pc.get_pay_period().start().iter_days() {
            if d > *pay_period.end() {
                break;
            }
            if d.weekday() == Weekday::Fri {
                total_deductions += self.dues;
            }
        }
        for sc in self.service_charges.iter() {
            if pay_period.contains(&sc.date) {
                total_deductions += sc.amount;
            }
        }
        total_deductions
    }
}
