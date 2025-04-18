use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use abstract_tx::AddEmployee;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{
    Affiliation, EmployeeId, PaymentClassification, PaymentMethod, PaymentSchedule,
};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: AddHourlyEmployee トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddHourlyEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    id: EmployeeId,
    name: String,
    address: String,
    hourly_rate: f32,

    dao: T,
    payroll_factory: F,
}
impl<T, F> AddHourlyEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    pub fn new(
        id: EmployeeId,
        name: &str,
        address: &str,
        hourly_rate: f32,
        dao: T,
        payroll_factory: F,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            address: address.to_string(),
            hourly_rate,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for AddHourlyEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> AddEmployee for AddHourlyEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        self.payroll_factory
            .mk_hourly_classification(self.hourly_rate)
    }
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
        self.payroll_factory.mk_weekly_schedule()
    }
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
        self.payroll_factory.mk_hold_method()
    }
    fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
        self.payroll_factory.mk_no_affiliation()
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for AddHourlyEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("execute called");
        AddEmployee::execute(self)
            .map(|_| Response::EmployeeId(self.id))
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use dao::{DaoError, EmployeeDao};
    use payroll_domain::{
        Affiliation, Employee, EmployeeId, MemberId, NoAffiliation, Paycheck,
        PaymentClassification, PaymentMethod, PaymentSchedule,
    };
    use payroll_factory::PayrollFactory;
    use payroll_impl::{HoldMethod, HourlyClassification, WeeklySchedule};

    #[derive(Debug, Clone)]
    struct Tester {
        expect: Vec<Employee>,
        actual: Rc<RefCell<Vec<Employee>>>,
    }
    impl Tester {
        fn assert(&self) {
            let borrowed = self.actual.borrow();
            assert_eq!(borrowed.len(), self.expect.len());
            for (e, a) in self.expect.iter().zip(borrowed.iter()) {
                assert_eq!(a.id(), e.id());
                assert_eq!(a.name(), e.name());
                assert_eq!(a.address(), e.address());
                assert!(a
                    .classification()
                    .borrow()
                    .as_any()
                    .downcast_ref::<HourlyClassification>()
                    .is_some());
                assert_eq!(
                    a.classification()
                        .borrow()
                        .as_any()
                        .downcast_ref::<HourlyClassification>(),
                    e.classification()
                        .borrow()
                        .as_any()
                        .downcast_ref::<HourlyClassification>(),
                );
                assert!(a
                    .schedule()
                    .borrow()
                    .as_any()
                    .downcast_ref::<WeeklySchedule>()
                    .is_some());
                // 今の WeeklySchedule は特にフィールドがないのでこのテストは不要ではある
                assert_eq!(
                    a.schedule()
                        .borrow()
                        .as_any()
                        .downcast_ref::<WeeklySchedule>(),
                    e.schedule()
                        .borrow()
                        .as_any()
                        .downcast_ref::<WeeklySchedule>()
                );
                assert!(a
                    .method()
                    .borrow()
                    .as_any()
                    .downcast_ref::<HoldMethod>()
                    .is_some());
                // 今の HoldMethod は特にフィールドがないのでこのテストは不要ではある
                assert_eq!(
                    a.method().borrow().as_any().downcast_ref::<HoldMethod>(),
                    e.method().borrow().as_any().downcast_ref::<HoldMethod>()
                );
                assert!(a
                    .affiliation()
                    .borrow()
                    .as_any()
                    .downcast_ref::<NoAffiliation>()
                    .is_some());
                // 今の NoAffiliation は特にフィールドがないのでこのテストは不要ではある
                assert_eq!(
                    a.affiliation()
                        .borrow()
                        .as_any()
                        .downcast_ref::<NoAffiliation>(),
                    e.affiliation()
                        .borrow()
                        .as_any()
                        .downcast_ref::<NoAffiliation>()
                );
            }
        }
    }
    impl EmployeeDao for Tester {
        type Ctx<'a> = &'a ();

        fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, DaoError>
        where
            F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>,
        {
            f(&())
        }

        fn add<'a>(
            &self,
            emp: Employee,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = EmployeeId, Err = DaoError> {
            tx_rs::with_tx(move |_ctx| {
                self.actual.borrow_mut().push(emp);
                Ok(1.into()) // no care
            })
        }
        fn delete<'a>(
            &self,
            _id: EmployeeId,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_ctx| unreachable!("delete method should not be called"))
        }

        fn fetch<'a>(
            &self,
            _id: EmployeeId,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Employee, Err = DaoError> {
            tx_rs::with_tx(move |_ctx| unreachable!("fetch method should not be called"))
        }

        fn fetch_all<'a>(
            &self,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Vec<(EmployeeId, Employee)>, Err = DaoError>
        {
            tx_rs::with_tx(move |_ctx| unreachable!("fetch_all method should not be called"))
        }

        fn update<'a>(
            &self,
            _emp: Employee,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_ctx| unreachable!("update method should not be called"))
        }

        fn add_union_member<'a>(
            &self,
            _member_id: MemberId,
            _emp_id: EmployeeId,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_ctx| unreachable!("add_union_member method should not be called"))
        }

        fn delete_union_member<'a>(
            &self,
            _member_id: MemberId,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_ctx| {
                unreachable!("delete_union_member method should not be called")
            })
        }

        fn find_union_member<'a>(
            &self,
            _member_id: MemberId,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = EmployeeId, Err = DaoError> {
            tx_rs::with_tx(move |_ctx| {
                unreachable!("find_union_member method should not be called")
            })
        }

        fn record_paycheck<'a>(
            &self,
            _emp_id: EmployeeId,
            _paycheck: Paycheck,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_ctx| unreachable!("record_paycheck method should not be called"))
        }

        fn find_paycheck<'a>(
            &self,
            _emp_id: EmployeeId,
            _pay_date: NaiveDate,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Paycheck, Err = DaoError> {
            tx_rs::with_tx(move |_ctx| unreachable!("find_paycheck method should not be called"))
        }
    }
    impl PayrollFactory for Tester {
        fn mk_salaried_classification(
            &self,
            _salary: f32,
        ) -> Rc<RefCell<dyn PaymentClassification>> {
            unimplemented!("mk_salaried_classification is not implemented")
        }
        fn mk_hourly_classification(
            &self,
            hourly_rate: f32,
        ) -> Rc<RefCell<dyn PaymentClassification>> {
            Rc::new(RefCell::new(HourlyClassification::new(hourly_rate)))
        }
        fn mk_commissioned_classification(
            &self,
            _salary: f32,
            _commission_rate: f32,
        ) -> Rc<RefCell<dyn PaymentClassification>> {
            unimplemented!("mk_commissioned_classification is not implemented")
        }

        fn mk_weekly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
            Rc::new(RefCell::new(WeeklySchedule))
        }
        fn mk_monthly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
            unimplemented!("mk_monthly_schedule is not implemented")
        }
        fn mk_biweekly_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
            unimplemented!("mk_biweekly_schedule is not implemented")
        }

        fn mk_hold_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
            Rc::new(RefCell::new(HoldMethod))
        }

        fn mk_direct_method(&self, _bank: &str, _account: &str) -> Rc<RefCell<dyn PaymentMethod>> {
            unimplemented!("mk_direct_method is not implemented")
        }
        fn mk_mail_method(&self, _address: &str) -> Rc<RefCell<dyn PaymentMethod>> {
            unimplemented!("mk_mail_method is not implemented")
        }

        fn mk_union_affiliation(
            &self,
            _member_id: MemberId,
            _dues: f32,
        ) -> Rc<RefCell<dyn Affiliation>> {
            unimplemented!("mk_union_affiliation is not implemented")
        }
        fn mk_no_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
            Rc::new(RefCell::new(NoAffiliation))
        }
    }

    #[test]
    fn test_add_emp() {
        let t = Tester {
            expect: vec![Employee::new(
                1.into(),
                "Ace",
                "Office",
                Rc::new(RefCell::new(HourlyClassification::new(123.0))),
                Rc::new(RefCell::new(WeeklySchedule)),
                Rc::new(RefCell::new(HoldMethod)),
                Rc::new(RefCell::new(NoAffiliation)),
            )],
            actual: Rc::new(RefCell::new(vec![])),
        };

        let tx: Box<dyn tx_app::Transaction> = Box::new(AddHourlyEmployeeTx::new(
            1.into(),
            "Ace",
            "Office",
            123.0,
            t.clone(),
            t.clone(),
        ));
        let _ = tx.execute();

        t.assert();
    }
}
