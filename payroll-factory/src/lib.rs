use payroll_domain::{
    Affiliation, MemberId, PaymentClassification, PaymentMethod, PaymentSchedule,
};

pub trait PayrollFactory {
    fn mk_salaried_classification(&self, salary: f32) -> Box<dyn PaymentClassification>;
    fn mk_hourly_classification(&self, hourly_rate: f32) -> Box<dyn PaymentClassification>;
    fn mk_commissioned_classification(
        &self,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn PaymentClassification>;

    fn mk_weekly_schedule(&self) -> Box<dyn PaymentSchedule>;
    fn mk_monthly_schedule(&self) -> Box<dyn PaymentSchedule>;
    fn mk_biweekly_schedule(&self) -> Box<dyn PaymentSchedule>;

    fn mk_hold_method(&self) -> Box<dyn PaymentMethod>;
    fn mk_direct_method(&self, bank: String, account: String) -> Box<dyn PaymentMethod>;
    fn mk_mail_method(&self, address: String) -> Box<dyn PaymentMethod>;

    fn mk_union_affiliation(&self, member_id: MemberId, dues: f32) -> Box<dyn Affiliation>;
    fn mk_no_affiliation(&self) -> Box<dyn Affiliation>;
}
