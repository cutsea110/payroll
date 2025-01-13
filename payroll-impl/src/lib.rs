mod classification {
    use chrono::NaiveDate;
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
    struct TimeCard {
        date: NaiveDate,
        hours: f32,
    }
    impl TimeCard {
        fn new(date: NaiveDate, hours: f32) -> Self {
            Self { date, hours }
        }
    }

    #[derive(Debug, Clone)]
    pub struct HourlyClassification {
        hourly_rate: f32,
        timecards: Vec<TimeCard>,
    }
    impl HourlyClassification {
        pub fn new(hourly_rate: f32) -> Self {
            Self {
                hourly_rate,
                timecards: vec![],
            }
        }
        pub fn add_time_card(&mut self, date: NaiveDate, hours: f32) {
            self.timecards.push(TimeCard::new(date, hours));
        }
        fn calculate_pay_for_timecard(&self, tc: &TimeCard) -> f32 {
            let overtime = (tc.hours - 8.0).max(0.0);
            let straight_time = tc.hours - overtime;
            straight_time * self.hourly_rate + overtime * self.hourly_rate * 1.5
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
            let pay_period = pc.get_pay_period();
            let mut total_pay = 0.0;
            for tc in &self.timecards {
                if pay_period.contains(&tc.date) {
                    total_pay += self.calculate_pay_for_timecard(tc);
                }
            }
            total_pay
        }
    }

    #[derive(Debug, Clone)]
    struct SalesReceipt {
        date: NaiveDate,
        amount: f32,
    }
    impl SalesReceipt {
        fn new(date: NaiveDate, amount: f32) -> Self {
            Self { date, amount }
        }
    }

    #[derive(Debug, Clone)]
    pub struct CommissionedClassification {
        salary: f32,
        commission_rate: f32,
        sales_receipts: Vec<SalesReceipt>,
    }
    impl CommissionedClassification {
        pub fn new(salary: f32, commission_rate: f32) -> Self {
            Self {
                salary,
                commission_rate,
                sales_receipts: vec![],
            }
        }
        pub fn add_sales_receipt(&mut self, date: NaiveDate, amount: f32) {
            let sr = SalesReceipt::new(date, amount);
            self.sales_receipts.push(sr);
        }
        fn calculate_pay_for_sales_receipt(&self, sr: &SalesReceipt) -> f32 {
            sr.amount * self.commission_rate
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
            let mut total_pay = self.salary;
            let pay_period = pc.get_pay_period();
            for sr in &self.sales_receipts {
                if pay_period.contains(&sr.date) {
                    total_pay += self.calculate_pay_for_sales_receipt(sr);
                }
            }
            total_pay
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
    use chrono::{Datelike, NaiveDate, Weekday};
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
    struct UnionAffiliation {
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
        pub fn get_member_id(&self) -> MemberId {
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
            let mut total_deductions = 0.0;
            let pay_period = pc.get_pay_period();
            for d in pc.get_pay_period().start().iter_days() {
                if d > *pay_period.end() {
                    break;
                }
                if d.weekday() == Weekday::Fri {
                    total_deductions += self.dues;
                }
            }
            for sc in self.service_charges.iter() {
                if pay_period.contains(&sc.date) {
                    total_deductions += sc.amount;
                }
            }
            total_deductions
        }
    }
}
pub use affiliation::*;
