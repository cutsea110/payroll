use std::{cell::RefCell, rc::Rc};

use crate::{
    affiliation::UnionAffiliation,
    classification::{CommissionedClassification, HourlyClassification, SalariedClassification},
    method::{DirectMethod, HoldMethod, MailMethod},
    schedule::{BiweeklySchedule, MonthlySchedule, WeeklySchedule},
};
use payroll_domain::{
    Affiliation, MemberId, NoAffiliation, PaymentClassification, PaymentMethod, PaymentSchedule,
};
use payroll_factory::PayrollFactory;

#[derive(Debug, Clone)]
pub struct PayrollFactoryImpl;

impl PayrollFactory for PayrollFactoryImpl {
    fn mk_hourly_classification(&self, hourly_rate: f32) -> Rc<RefCell<dyn PaymentClassification>> {
        Rc::new(RefCell::new(HourlyClassification::new(hourly_rate)))
    }
    fn mk_salaried_classification(&self, salary: f32) -> Rc<RefCell<dyn PaymentClassification>> {
        Rc::new(RefCell::new(SalariedClassification::new(salary)))
    }
    fn mk_commissioned_classification(
        &self,
        salary: f32,
        commission_rate: f32,
    ) -> Rc<RefCell<dyn PaymentClassification>> {
        Rc::new(RefCell::new(CommissionedClassification::new(
            salary,
            commission_rate,
        )))
    }
    fn mk_weekly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
        Rc::new(RefCell::new(WeeklySchedule))
    }
    fn mk_monthly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
        Rc::new(RefCell::new(MonthlySchedule))
    }
    fn mk_biweekly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
        Rc::new(RefCell::new(BiweeklySchedule))
    }
    fn mk_hold_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
        Rc::new(RefCell::new(HoldMethod))
    }
    fn mk_direct_method(&self, bank: &str, account: &str) -> Rc<RefCell<dyn PaymentMethod>> {
        Rc::new(RefCell::new(DirectMethod::new(bank, account)))
    }
    fn mk_mail_method(&self, address: &str) -> Rc<RefCell<dyn PaymentMethod>> {
        Rc::new(RefCell::new(MailMethod::new(address)))
    }
    fn mk_union_affiliation(&self, member_id: MemberId, dues: f32) -> Rc<RefCell<dyn Affiliation>> {
        Rc::new(RefCell::new(UnionAffiliation::new(member_id, dues)))
    }
    fn mk_no_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
        Rc::new(RefCell::new(NoAffiliation))
    }
}
