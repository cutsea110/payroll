use std::sync::{Arc, Mutex};

use payroll_domain::{
    Affiliation, MemberId, PaymentClassification, PaymentMethod, PaymentSchedule,
};

pub trait SalariedClassificationFactory {
    fn mk_classification(&self, salary: f32) -> Arc<Mutex<dyn PaymentClassification>>;
}
pub trait HourlyClassificationFactory {
    fn mk_classification(&self, hourly_rate: f32) -> Arc<Mutex<dyn PaymentClassification>>;
}
pub trait CommissionedClassificationFactory {
    fn mk_classification(
        &self,
        salary: f32,
        commission_rate: f32,
    ) -> Arc<Mutex<dyn PaymentClassification>>;
}
pub trait MonthlyScheduleFactory {
    fn mk_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>>;
}
pub trait WeeklyScheduleFactory {
    fn mk_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>>;
}
pub trait BiweeklyScheduleFactory {
    fn mk_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>>;
}
pub trait HoldMethodFactory {
    fn mk_method(&self) -> Arc<Mutex<dyn PaymentMethod>>;
}
pub trait DirectMethodFactory {
    fn mk_method(&self, bank: &str, account: &str) -> Arc<Mutex<dyn PaymentMethod>>;
}
pub trait MailMethodFactory {
    fn mk_method(&self, address: &str) -> Arc<Mutex<dyn PaymentMethod>>;
}
pub trait UnionAffiliationFactory {
    fn mk_affiliation(&self, member_id: MemberId, dues: f32) -> Arc<Mutex<dyn Affiliation>>;
}
pub trait NoAffiliationFactory {
    fn mk_affiliation(&self) -> Arc<Mutex<dyn Affiliation>>;
}
