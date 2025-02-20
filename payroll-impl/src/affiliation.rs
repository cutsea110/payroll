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
    fn test_add_service_charge() {
        let mut af = UnionAffiliation::new(1.into(), 1.2);
        assert_eq!(af.service_charges.len(), 0);

        af.add_service_charge(NaiveDate::from_ymd_opt(2025, 2, 1).unwrap(), 7.50);
        assert_eq!(af.service_charges.len(), 1);
        assert_eq!(
            af.service_charges[0].date,
            NaiveDate::from_ymd_opt(2025, 2, 1).unwrap()
        );
        assert_eq!(af.service_charges[0].amount, 7.50);

        af.add_service_charge(NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(), 10.20);
        assert_eq!(af.service_charges.len(), 2);
        assert_eq!(
            af.service_charges[1].date,
            NaiveDate::from_ymd_opt(2025, 2, 28).unwrap()
        );
        assert_eq!(af.service_charges[1].amount, 10.20);

        af.add_service_charge(NaiveDate::from_ymd_opt(2025, 3, 3).unwrap(), 5.25);
        assert_eq!(af.service_charges.len(), 3);
        assert_eq!(
            af.service_charges[2].date,
            NaiveDate::from_ymd_opt(2025, 3, 3).unwrap()
        );
        assert_eq!(af.service_charges[2].amount, 5.25);
    }

    #[test]
    fn test_calculate_deductions() {
        use chrono::NaiveDate;
        use payroll_domain::{Affiliation, Paycheck};

        let af = UnionAffiliation {
            member_id: 1.into(),
            dues: 1.2,
            service_charges: vec![
                ServiceCharge::new(NaiveDate::from_ymd_opt(2025, 2, 1).unwrap(), 7.50),
                ServiceCharge::new(NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(), 10.20),
                ServiceCharge::new(NaiveDate::from_ymd_opt(2025, 3, 3).unwrap(), 5.25),
            ],
        };

        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
        );
        let deductions = af.calculate_deductions(&pc);
        assert_eq!(deductions, 6.0); // 1.2 * 5

        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2025, 2, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(),
        );
        let deductions = af.calculate_deductions(&pc);
        assert_eq!(deductions, 22.5); // 1.2 * 4 + 7.50 + 10.20

        let pc = Paycheck::new(
            NaiveDate::from_ymd_opt(2025, 3, 1).unwrap()
                ..=NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        );
        let deductions = af.calculate_deductions(&pc);
        assert_eq!(deductions, 10.05); // 1.2 * 4 + 5.25
    }
}
