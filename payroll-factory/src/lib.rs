use std::{cell::RefCell, rc::Rc};

use payroll_domain::{
    Affiliation, MemberId, PaymentClassification, PaymentMethod, PaymentSchedule,
};

pub trait PayrollFactory {
    fn mk_salaried_classification(&self, salary: f32) -> Rc<RefCell<dyn PaymentClassification>>;
    fn mk_hourly_classification(&self, hourly_rate: f32) -> Rc<RefCell<dyn PaymentClassification>>;
    fn mk_commissioned_classification(
        &self,
        salary: f32,
        commission_rate: f32,
    ) -> Rc<RefCell<dyn PaymentClassification>>;

    fn mk_weekly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
    fn mk_monthly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
    fn mk_biweekly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;

    fn mk_hold_method(&self) -> Rc<RefCell<dyn PaymentMethod>>;
    fn mk_direct_method(&self, bank: &str, account: &str) -> Rc<RefCell<dyn PaymentMethod>>;
    fn mk_mail_method(&self, address: &str) -> Rc<RefCell<dyn PaymentMethod>>;

    fn mk_union_affiliation(&self, member_id: MemberId, dues: f32) -> Rc<RefCell<dyn Affiliation>>;
    fn mk_no_affiliation(&self) -> Rc<RefCell<dyn Affiliation>>;
}

#[cfg(test)]
pub mod test_mock {
    use super::*;

    pub struct MockPayrollFactory {
        _mk_salaried_classification:
            Option<Box<dyn Fn(f32) -> Rc<RefCell<dyn PaymentClassification>>>>,
        _mk_hourly_classification:
            Option<Box<dyn Fn(f32) -> Rc<RefCell<dyn PaymentClassification>>>>,
        _mk_commissioned_classification:
            Option<Box<dyn Fn(f32, f32) -> Rc<RefCell<dyn PaymentClassification>>>>,

        _mk_weekly_schedule: Option<Box<dyn Fn() -> Rc<RefCell<dyn PaymentSchedule>>>>,
        _mk_monthly_schedule: Option<Box<dyn Fn() -> Rc<RefCell<dyn PaymentSchedule>>>>,
        _mk_biweekly_schedule: Option<Box<dyn Fn() -> Rc<RefCell<dyn PaymentSchedule>>>>,

        _mk_hold_method: Option<Box<dyn Fn() -> Rc<RefCell<dyn PaymentMethod>>>>,
        _mk_direct_method: Option<Box<dyn Fn(&str, &str) -> Rc<RefCell<dyn PaymentMethod>>>>,
        _mk_mail_method: Option<Box<dyn Fn(&str) -> Rc<RefCell<dyn PaymentMethod>>>>,

        _mk_union_affiliation: Option<Box<dyn Fn(MemberId, f32) -> Rc<RefCell<dyn Affiliation>>>>,
        _mk_no_affiliation: Option<Box<dyn Fn() -> Rc<RefCell<dyn Affiliation>>>>,
    }
    impl MockPayrollFactory {
        pub fn unimplemented() -> Self {
            Self {
                _mk_salaried_classification: None,
                _mk_hourly_classification: None,
                _mk_commissioned_classification: None,

                _mk_weekly_schedule: None,
                _mk_monthly_schedule: None,
                _mk_biweekly_schedule: None,

                _mk_hold_method: None,
                _mk_direct_method: None,
                _mk_mail_method: None,

                _mk_union_affiliation: None,
                _mk_no_affiliation: None,
            }
        }

        pub fn set_mk_salaried_classification(
            &mut self,
            f: Option<Box<dyn Fn(f32) -> Rc<RefCell<dyn PaymentClassification>>>>,
        ) {
            self._mk_salaried_classification = f;
        }
        pub fn set_mk_hourly_classification(
            &mut self,
            f: Option<Box<dyn Fn(f32) -> Rc<RefCell<dyn PaymentClassification>>>>,
        ) {
            self._mk_hourly_classification = f;
        }
        pub fn set_mk_commissioned_classification(
            &mut self,
            f: Option<Box<dyn Fn(f32, f32) -> Rc<RefCell<dyn PaymentClassification>>>>,
        ) {
            self._mk_commissioned_classification = f;
        }
        pub fn set_mk_weekly_schedule(
            &mut self,
            o: Option<Box<dyn Fn() -> Rc<RefCell<dyn PaymentSchedule>>>>,
        ) {
            self._mk_weekly_schedule = o;
        }
        pub fn set_mk_monthly_schedule(
            &mut self,
            o: Option<Box<dyn Fn() -> Rc<RefCell<dyn PaymentSchedule>>>>,
        ) {
            self._mk_monthly_schedule = o;
        }
        pub fn set_mk_biweekly_schedule(
            &mut self,
            o: Option<Box<dyn Fn() -> Rc<RefCell<dyn PaymentSchedule>>>>,
        ) {
            self._mk_biweekly_schedule = o;
        }
        pub fn set_mk_hold_method(
            &mut self,
            o: Option<Box<dyn Fn() -> Rc<RefCell<dyn PaymentMethod>>>>,
        ) {
            self._mk_hold_method = o;
        }
        pub fn set_mk_direct_method(
            &mut self,
            f: Option<Box<dyn Fn(&str, &str) -> Rc<RefCell<dyn PaymentMethod>>>>,
        ) {
            self._mk_direct_method = f;
        }
        pub fn set_mk_mail_method(
            &mut self,
            f: Option<Box<dyn Fn(&str) -> Rc<RefCell<dyn PaymentMethod>>>>,
        ) {
            self._mk_mail_method = f;
        }
        pub fn set_mk_union_affiliation(
            &mut self,
            f: Option<Box<dyn Fn(MemberId, f32) -> Rc<RefCell<dyn Affiliation>>>>,
        ) {
            self._mk_union_affiliation = f;
        }
        pub fn set_mk_no_affiliation(
            &mut self,
            o: Option<Box<dyn Fn() -> Rc<RefCell<dyn Affiliation>>>>,
        ) {
            self._mk_no_affiliation = o;
        }
    }

    impl PayrollFactory for MockPayrollFactory {
        fn mk_salaried_classification(
            &self,
            salary: f32,
        ) -> Rc<RefCell<dyn PaymentClassification>> {
            if let Some(f) = &self._mk_salaried_classification {
                return f(salary);
            }

            panic!("unexpected mk_salaried_classification call");
        }

        fn mk_hourly_classification(
            &self,
            hourly_rate: f32,
        ) -> Rc<RefCell<dyn PaymentClassification>> {
            if let Some(f) = &self._mk_hourly_classification {
                return f(hourly_rate);
            }

            panic!("unexpected mk_hourly_classification call");
        }

        fn mk_commissioned_classification(
            &self,
            salary: f32,
            commission_rate: f32,
        ) -> Rc<RefCell<dyn PaymentClassification>> {
            if let Some(f) = &self._mk_commissioned_classification {
                return f(salary, commission_rate);
            }

            panic!("unexpected mk_commissioned_classification call");
        }

        fn mk_weekly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
            if let Some(f) = &self._mk_weekly_schedule {
                return f();
            }

            panic!("unexpected mk_weekly_schedule call");
        }

        fn mk_monthly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
            if let Some(f) = &self._mk_monthly_schedule {
                return f();
            }

            panic!("unexpected mk_monthly_schedule call");
        }

        fn mk_biweekly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
            if let Some(f) = &self._mk_biweekly_schedule {
                return f();
            }

            panic!("unexpected mk_biweekly_schedule call");
        }

        fn mk_hold_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
            if let Some(f) = &self._mk_hold_method {
                return f();
            }

            panic!("unexpected mk_hold_method call");
        }

        fn mk_direct_method(&self, bank: &str, account: &str) -> Rc<RefCell<dyn PaymentMethod>> {
            if let Some(f) = &self._mk_direct_method {
                return f(bank, account);
            }

            panic!("unexpected mk_direct_method call");
        }

        fn mk_mail_method(&self, address: &str) -> Rc<RefCell<dyn PaymentMethod>> {
            if let Some(f) = &self._mk_mail_method {
                return f(address);
            }

            panic!("unexpected mk_mail_method call");
        }

        fn mk_union_affiliation(
            &self,
            member_id: MemberId,
            dues: f32,
        ) -> Rc<RefCell<dyn Affiliation>> {
            if let Some(f) = &self._mk_union_affiliation {
                return f(member_id, dues);
            }

            panic!("unexpected mk_union_affiliation call");
        }

        fn mk_no_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
            if let Some(f) = &self._mk_no_affiliation {
                return f();
            }

            panic!("unexpected mk_no_affiliation call");
        }
    }
}
