use chrono::{Datelike, NaiveDate, Weekday};
use log::{debug, trace};
use std::any::Any;

use payroll_domain::{Affiliation, MemberId, Paycheck};

#[derive(Debug, Clone, PartialEq)]
struct ServiceCharge {
    date: NaiveDate,
    amount: f32,
}
impl ServiceCharge {
    fn new(date: NaiveDate, amount: f32) -> Self {
        Self { date, amount }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
            .fold(0f32, |acc, _| acc + self.dues);
        debug!("dues_amount: {}", dues_amount);
        let service_amount = self
            .service_charges
            .iter()
            .filter(|sc| pay_period.contains(&sc.date))
            .fold(0f32, |acc, sc| acc + sc.amount);
        debug!("service_amount: {}", service_amount);

        dues_amount + service_amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_service_charge() {
        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
        );
        let aff = UnionAffiliation::new(1.into(), 10.0);
        let deductions = aff.calculate_deductions(&pc);
        assert_eq!(deductions, 50.0);
    }

    #[test]
    fn test_add_single_service_charge() {
        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
        );
        let mut aff = UnionAffiliation::new(1.into(), 10.0);
        aff.add_service_charge(NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(), 105.0);
        let deductions = aff.calculate_deductions(&pc);
        assert_eq!(deductions, 155.0);
    }

    #[test]
    fn test_add_multiple_service_charges() {
        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
        );
        let mut aff = UnionAffiliation::new(1.into(), 10.0);
        aff.add_service_charge(NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(), 100.5);
        aff.add_service_charge(NaiveDate::from_ymd_opt(2025, 1, 20).unwrap(), 200.5);
        let deductions = aff.calculate_deductions(&pc);
        assert_eq!(deductions, 351.0);
    }

    #[test]
    fn test_add_outrange_service_charge() {
        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
        );
        let mut aff = UnionAffiliation::new(1.into(), 10.0);
        aff.add_service_charge(NaiveDate::from_ymd_opt(2025, 2, 15).unwrap(), 100.5);
        let deductions = aff.calculate_deductions(&pc);
        assert_eq!(deductions, 50.0);
    }
}
