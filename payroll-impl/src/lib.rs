mod classification {
    use std::any::Any;

    use payroll_domain::{Paycheck, PaymentClassification};

    #[derive(Debug, Clone)]
    pub struct SalariedClassification {
        salary: f32,
    }
    impl SalariedClassification {
        pub fn new(salary: f32) -> Self {
            Self { salary }
        }
    }
    impl PaymentClassification for SalariedClassification {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn calculate_pay(&self, _pc: &Paycheck) -> f32 {
            self.salary
        }
    }

    #[derive(Debug, Clone)]
    pub struct HourlyClassification {
        hourly_rate: f32,
    }
    impl HourlyClassification {
        pub fn new(hourly_rate: f32) -> Self {
            Self { hourly_rate }
        }
    }
    impl PaymentClassification for HourlyClassification {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn calculate_pay(&self, pc: &Paycheck) -> f32 {
            unimplemented!()
        }
    }

    #[derive(Debug, Clone)]
    pub struct CommissionedClassification {
        salary: f32,
        commission_rate: f32,
    }
    impl CommissionedClassification {
        pub fn new(salary: f32, commission_rate: f32) -> Self {
            Self {
                salary,
                commission_rate,
            }
        }
    }
    impl PaymentClassification for CommissionedClassification {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        fn calculate_pay(&self, pc: &Paycheck) -> f32 {
            unimplemented!()
        }
    }
}
pub use classification::*;

mod schedule {
    use chrono::{Datelike, Days, NaiveDate, Weekday};
    use std::ops::RangeInclusive;

    use payroll_domain::PaymentSchedule;

    #[derive(Debug, Clone)]
    pub struct MonthlySchedule;
    impl MonthlySchedule {
        pub fn is_last_day_of_month(&self, date: NaiveDate) -> bool {
            date.month() != date.checked_add_days(Days::new(1)).unwrap().month()
        }
    }
    impl PaymentSchedule for MonthlySchedule {
        fn is_pay_date(&self, date: NaiveDate) -> bool {
            self.is_last_day_of_month(date)
        }
        fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
            pay_date.with_day(1).unwrap()..=pay_date
        }
    }

    #[derive(Debug, Clone)]
    pub struct WeeklySchedule;
    impl PaymentSchedule for WeeklySchedule {
        fn is_pay_date(&self, date: NaiveDate) -> bool {
            date.weekday() == Weekday::Fri
        }
        fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
            pay_date.checked_sub_days(Days::new(6)).unwrap()..=pay_date
        }
    }

    #[derive(Debug, Clone)]
    pub struct BiweeklySchedule;
    impl PaymentSchedule for BiweeklySchedule {
        fn is_pay_date(&self, date: NaiveDate) -> bool {
            date.weekday() == Weekday::Fri && date.iso_week().week() % 2 == 0
        }
        fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
            pay_date.checked_sub_days(Days::new(13)).unwrap()..=pay_date
        }
    }
}
pub use schedule::*;

mod method {
    use payroll_domain::{Paycheck, PaymentMethod};

    #[derive(Debug, Clone)]
    pub struct HoldMethod;
    impl PaymentMethod for HoldMethod {
        fn pay(&self, pc: &Paycheck) {
            println!("HoldMethod.pay: {:?}", pc);
        }
    }

    #[derive(Debug, Clone)]
    pub struct DirectMethod {
        bank: String,
        account: String,
    }
    impl DirectMethod {
        pub fn new(bank: &str, account: &str) -> Self {
            Self {
                bank: bank.to_string(),
                account: account.to_string(),
            }
        }
    }
    impl PaymentMethod for DirectMethod {
        fn pay(&self, pc: &Paycheck) {
            println!("DirectMethod to {} {}: {:#?}", self.bank, self.account, pc);
        }
    }

    #[derive(Debug, Clone)]
    pub struct MailMethod {
        address: String,
    }
    impl MailMethod {
        pub fn new(address: &str) -> Self {
            Self {
                address: address.to_string(),
            }
        }
    }
    impl PaymentMethod for MailMethod {
        fn pay(&self, pc: &Paycheck) {
            println!("MailMethod to {}: {:#?}", self.address, pc);
        }
    }
}
pub use method::*;

mod affiliation {
    use payroll_domain::{Affiliation, MemberId, Paycheck};

    #[derive(Debug, Clone)]
    struct UnionAffiliation {
        member_id: MemberId,
        dues: f32,
    }
    impl UnionAffiliation {
        pub fn new(member_id: MemberId, dues: f32) -> Self {
            Self { member_id, dues }
        }
    }
    impl Affiliation for UnionAffiliation {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn calculate_deductions(&self, pc: &Paycheck) -> f32 {
            unimplemented!()
        }
    }
}
pub use affiliation::*;
