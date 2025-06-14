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

pub trait PayrollFactory:
    SalariedClassificationFactory
    + HourlyClassificationFactory
    + CommissionedClassificationFactory
    + MonthlyScheduleFactory
    + WeeklyScheduleFactory
    + BiweeklyScheduleFactory
    + HoldMethodFactory
    + DirectMethodFactory
{
    fn mk_salaried_classification(&self, salary: f32) -> Arc<Mutex<dyn PaymentClassification>> {
        SalariedClassificationFactory::mk_classification(self, salary)
    }
    fn mk_hourly_classification(&self, hourly_rate: f32) -> Arc<Mutex<dyn PaymentClassification>> {
        HourlyClassificationFactory::mk_classification(self, hourly_rate)
    }
    fn mk_commissioned_classification(
        &self,
        salary: f32,
        commission_rate: f32,
    ) -> Arc<Mutex<dyn PaymentClassification>> {
        CommissionedClassificationFactory::mk_classification(self, salary, commission_rate)
    }

    fn mk_monthly_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>> {
        MonthlyScheduleFactory::mk_schedule(self)
    }
    fn mk_weekly_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>> {
        WeeklyScheduleFactory::mk_schedule(self)
    }
    fn mk_biweekly_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>> {
        BiweeklyScheduleFactory::mk_schedule(self)
    }

    fn mk_hold_method(&self) -> Arc<Mutex<dyn PaymentMethod>> {
        HoldMethodFactory::mk_method(self)
    }
    fn mk_direct_method(&self, bank: &str, account: &str) -> Arc<Mutex<dyn PaymentMethod>> {
        DirectMethodFactory::mk_method(self, bank, account)
    }
    fn mk_mail_method(&self, address: &str) -> Arc<Mutex<dyn PaymentMethod>>;

    fn mk_union_affiliation(&self, member_id: MemberId, dues: f32) -> Arc<Mutex<dyn Affiliation>>;
    fn mk_no_affiliation(&self) -> Arc<Mutex<dyn Affiliation>>;
}
