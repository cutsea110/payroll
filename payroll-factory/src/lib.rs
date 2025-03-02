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
        _mk_salaried_classification: Box<dyn Fn(f32) -> Rc<RefCell<dyn PaymentClassification>>>,
        _mk_hourly_classification: Box<dyn Fn(f32) -> Rc<RefCell<dyn PaymentClassification>>>,
        _mk_commissioned_classification:
            Box<dyn Fn(f32, f32) -> Rc<RefCell<dyn PaymentClassification>>>,

        _mk_weekly_schedule: Rc<RefCell<dyn PaymentSchedule>>,
        _mk_monthly_schedule: Rc<RefCell<dyn PaymentSchedule>>,
        _mk_biweekly_schedule: Rc<RefCell<dyn PaymentSchedule>>,

        _mk_hold_method: Rc<RefCell<dyn PaymentMethod>>,
        _mk_direct_method: Box<dyn Fn(&str, &str) -> Rc<RefCell<dyn PaymentMethod>>>,
        _mk_mail_method: Box<dyn Fn(&str) -> Rc<RefCell<dyn PaymentMethod>>>,

        _mk_union_affiliation: Box<dyn Fn(MemberId, f32) -> Rc<RefCell<dyn Affiliation>>>,
        _mk_no_affiliation: Rc<RefCell<dyn Affiliation>>,
    }
    impl MockPayrollFactory {
        pub fn unimplemented() -> Self {
            Self {
                _mk_salaried_classification: unimplemented!(
                    "unexpectedly mk_salaried_classification called"
                ),
                _mk_hourly_classification: unimplemented!(
                    "unexpectedly mk_hourly_classification called"
                ),
                _mk_commissioned_classification: unimplemented!(
                    "unexpectedly mk_commissioned_classification called"
                ),

                _mk_weekly_schedule: unimplemented!("unexpectedly mk_weekly_schedule called"),
                _mk_monthly_schedule: unimplemented!("unexpectedly mk_monthly_schedule called"),
                _mk_biweekly_schedule: unimplemented!("unexpectedly mk_biweekly_schedule called"),

                _mk_hold_method: unimplemented!("unexpectedly mk_hold_method called"),
                _mk_direct_method: unimplemented!("unexpectedly mk_direct_method called"),
                _mk_mail_method: unimplemented!("unexpectedly mk_mail_method called"),

                _mk_union_affiliation: unimplemented!("unexpectedly mk_union_affiliation called"),
                _mk_no_affiliation: unimplemented!("unexpectedly mk_no_affiliation called"),
            }
        }

        pub fn set_mk_salaried_classification(
            &mut self,
            f: Box<dyn Fn(f32) -> Rc<RefCell<dyn PaymentClassification>>>,
        ) {
            self._mk_salaried_classification = f;
        }
        pub fn set_mk_hourly_classification(
            &mut self,
            f: Box<dyn Fn(f32) -> Rc<RefCell<dyn PaymentClassification>>>,
        ) {
            self._mk_hourly_classification = f;
        }
        pub fn set_mk_commissioned_classification(
            &mut self,
            f: Box<dyn Fn(f32, f32) -> Rc<RefCell<dyn PaymentClassification>>>,
        ) {
            self._mk_commissioned_classification = f;
        }
        pub fn set_mk_weekly_schedule(&mut self, o: Rc<RefCell<dyn PaymentSchedule>>) {
            self._mk_weekly_schedule = o;
        }
        pub fn set_mk_monthly_schedule(&mut self, o: Rc<RefCell<dyn PaymentSchedule>>) {
            self._mk_monthly_schedule = o;
        }
        pub fn set_mk_biweekly_schedule(&mut self, o: Rc<RefCell<dyn PaymentSchedule>>) {
            self._mk_biweekly_schedule = o;
        }
        pub fn set_mk_hold_method(&mut self, o: Rc<RefCell<dyn PaymentMethod>>) {
            self._mk_hold_method = o;
        }
        pub fn set_mk_direct_method(
            &mut self,
            f: Box<dyn Fn(&str, &str) -> Rc<RefCell<dyn PaymentMethod>>>,
        ) {
            self._mk_direct_method = f;
        }
        pub fn set_mk_mail_method(
            &mut self,
            f: Box<dyn Fn(&str) -> Rc<RefCell<dyn PaymentMethod>>>,
        ) {
            self._mk_mail_method = f;
        }
        pub fn set_mk_union_affiliation(
            &mut self,
            f: Box<dyn Fn(MemberId, f32) -> Rc<RefCell<dyn Affiliation>>>,
        ) {
            self._mk_union_affiliation = f;
        }
        pub fn set_mk_no_affiliation(&mut self, o: Rc<RefCell<dyn Affiliation>>) {
            self._mk_no_affiliation = o;
        }
    }

    impl PayrollFactory for MockPayrollFactory {
        fn mk_salaried_classification(
            &self,
            salary: f32,
        ) -> Rc<RefCell<dyn PaymentClassification>> {
            (self._mk_salaried_classification)(salary)
        }

        fn mk_hourly_classification(
            &self,
            hourly_rate: f32,
        ) -> Rc<RefCell<dyn PaymentClassification>> {
            (self._mk_hourly_classification)(hourly_rate)
        }

        fn mk_commissioned_classification(
            &self,
            salary: f32,
            commission_rate: f32,
        ) -> Rc<RefCell<dyn PaymentClassification>> {
            (self._mk_commissioned_classification)(salary, commission_rate)
        }

        fn mk_weekly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
            self._mk_weekly_schedule.clone()
        }

        fn mk_monthly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
            self._mk_monthly_schedule.clone()
        }

        fn mk_biweekly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
            self._mk_biweekly_schedule.clone()
        }

        fn mk_hold_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
            self._mk_hold_method.clone()
        }

        fn mk_direct_method(&self, bank: &str, account: &str) -> Rc<RefCell<dyn PaymentMethod>> {
            (self._mk_direct_method)(bank, account)
        }

        fn mk_mail_method(&self, address: &str) -> Rc<RefCell<dyn PaymentMethod>> {
            (self._mk_mail_method)(address)
        }

        fn mk_union_affiliation(
            &self,
            member_id: MemberId,
            dues: f32,
        ) -> Rc<RefCell<dyn Affiliation>> {
            (self._mk_union_affiliation)(member_id, dues)
        }

        fn mk_no_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
            self._mk_no_affiliation.clone()
        }
    }
}
