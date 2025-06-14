use std::sync::{Arc, Mutex};

use crate::{
    affiliation::UnionAffiliation,
    classification::{CommissionedClassification, HourlyClassification, SalariedClassification},
    method::{DirectMethod, HoldMethod, MailMethod},
    schedule::{BiweeklySchedule, MonthlySchedule, WeeklySchedule},
};
use payroll_domain::{
    Affiliation, MemberId, NoAffiliation, PaymentClassification, PaymentMethod, PaymentSchedule,
};
use payroll_factory::{
    BiweeklyScheduleFactory, CommissionedClassificationFactory, HoldMethodFactory,
    HourlyClassificationFactory, MonthlyScheduleFactory, PayrollFactory,
    SalariedClassificationFactory, WeeklyScheduleFactory,
};

#[derive(Debug, Clone)]
pub struct PayrollFactoryImpl;

impl SalariedClassificationFactory for PayrollFactoryImpl {
    fn mk_classification(&self, salary: f32) -> Arc<Mutex<dyn PaymentClassification>> {
        Arc::new(Mutex::new(SalariedClassification::new(salary)))
    }
}
impl HourlyClassificationFactory for PayrollFactoryImpl {
    fn mk_classification(&self, hourly_rate: f32) -> Arc<Mutex<dyn PaymentClassification>> {
        Arc::new(Mutex::new(HourlyClassification::new(hourly_rate)))
    }
}
impl CommissionedClassificationFactory for PayrollFactoryImpl {
    fn mk_classification(
        &self,
        salary: f32,
        commission_rate: f32,
    ) -> Arc<Mutex<dyn PaymentClassification>> {
        Arc::new(Mutex::new(CommissionedClassification::new(
            salary,
            commission_rate,
        )))
    }
}
impl MonthlyScheduleFactory for PayrollFactoryImpl {
    fn mk_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>> {
        Arc::new(Mutex::new(MonthlySchedule))
    }
}
impl WeeklyScheduleFactory for PayrollFactoryImpl {
    fn mk_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>> {
        Arc::new(Mutex::new(WeeklySchedule))
    }
}
impl BiweeklyScheduleFactory for PayrollFactoryImpl {
    fn mk_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>> {
        Arc::new(Mutex::new(BiweeklySchedule))
    }
}
impl HoldMethodFactory for PayrollFactoryImpl {
    fn mk_method(&self) -> Arc<Mutex<dyn PaymentMethod>> {
        Arc::new(Mutex::new(HoldMethod))
    }
}

impl PayrollFactory for PayrollFactoryImpl {
    fn mk_direct_method(&self, bank: &str, account: &str) -> Arc<Mutex<dyn PaymentMethod>> {
        Arc::new(Mutex::new(DirectMethod::new(bank, account)))
    }
    fn mk_mail_method(&self, address: &str) -> Arc<Mutex<dyn PaymentMethod>> {
        Arc::new(Mutex::new(MailMethod::new(address)))
    }
    fn mk_union_affiliation(&self, member_id: MemberId, dues: f32) -> Arc<Mutex<dyn Affiliation>> {
        Arc::new(Mutex::new(UnionAffiliation::new(member_id, dues)))
    }
    fn mk_no_affiliation(&self) -> Arc<Mutex<dyn Affiliation>> {
        Arc::new(Mutex::new(NoAffiliation))
    }
}
