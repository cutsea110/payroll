use payroll_domain::{MemberId, NoAffiliation};
use payroll_factory::PayrollFactory;

use crate::{
    affiliation::UnionAffiliation,
    classification::{CommissionedClassification, HourlyClassification, SalariedClassification},
    method::{DirectMethod, HoldMethod, MailMethod},
    schedule::{BiweeklySchedule, MonthlySchedule, WeeklySchedule},
};

pub struct PayrollFactoryImpl;

impl PayrollFactory for PayrollFactoryImpl {
    fn mk_hourly_classification(
        &self,
        hourly_rate: f32,
    ) -> Box<dyn payroll_domain::PaymentClassification> {
        Box::new(HourlyClassification::new(hourly_rate))
    }
    fn mk_salaried_classification(
        &self,
        salary: f32,
    ) -> Box<dyn payroll_domain::PaymentClassification> {
        Box::new(SalariedClassification::new(salary))
    }
    fn mk_commissioned_classification(
        &self,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn payroll_domain::PaymentClassification> {
        Box::new(CommissionedClassification::new(salary, commission_rate))
    }
    fn mk_weekly_schedule(&self) -> Box<dyn payroll_domain::PaymentSchedule> {
        Box::new(WeeklySchedule)
    }
    fn mk_monthly_schedule(&self) -> Box<dyn payroll_domain::PaymentSchedule> {
        Box::new(MonthlySchedule)
    }
    fn mk_biweekly_schedule(&self) -> Box<dyn payroll_domain::PaymentSchedule> {
        Box::new(BiweeklySchedule)
    }
    fn mk_hold_method(&self) -> Box<dyn payroll_domain::PaymentMethod> {
        Box::new(HoldMethod)
    }
    fn mk_direct_method(
        &self,
        bank: String,
        account: String,
    ) -> Box<dyn payroll_domain::PaymentMethod> {
        Box::new(DirectMethod::new(&bank, &account))
    }
    fn mk_mail_method(&self, address: String) -> Box<dyn payroll_domain::PaymentMethod> {
        Box::new(MailMethod::new(&address))
    }
    fn mk_union_affiliation(
        &self,
        member_id: MemberId,
        dues: f32,
    ) -> Box<dyn payroll_domain::Affiliation> {
        Box::new(UnionAffiliation::new(member_id, dues))
    }
    fn mk_no_affiliation(&self) -> Box<dyn payroll_domain::Affiliation> {
        Box::new(NoAffiliation)
    }
}
