use std::sync::{Arc, Mutex};

use payroll_domain::{
    Affiliation, MemberId, PaymentClassification, PaymentMethod, PaymentSchedule,
};

pub trait PayrollFactory {
    fn mk_salaried_classification(&self, salary: f32) -> Arc<Mutex<dyn PaymentClassification>>;
    fn mk_hourly_classification(&self, hourly_rate: f32) -> Arc<Mutex<dyn PaymentClassification>>;
    fn mk_commissioned_classification(
        &self,
        salary: f32,
        commission_rate: f32,
    ) -> Arc<Mutex<dyn PaymentClassification>>;

    fn mk_weekly_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>>;
    fn mk_monthly_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>>;
    fn mk_biweekly_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>>;

    fn mk_hold_method(&self) -> Arc<Mutex<dyn PaymentMethod>>;
    fn mk_direct_method(&self, bank: &str, account: &str) -> Arc<Mutex<dyn PaymentMethod>>;
    fn mk_mail_method(&self, address: &str) -> Arc<Mutex<dyn PaymentMethod>>;

    fn mk_union_affiliation(&self, member_id: MemberId, dues: f32) -> Arc<Mutex<dyn Affiliation>>;
    fn mk_no_affiliation(&self) -> Arc<Mutex<dyn Affiliation>>;
}
