use anyhow;
use log::trace;
use std::sync::{Arc, Mutex};

use abstract_tx::{AddEmployee, UsecaseError};
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{
    Affiliation, EmployeeId, PaymentClassification, PaymentMethod, PaymentSchedule,
};
use payroll_factory::{
    HoldMethodFactory, MonthlyScheduleFactory, NoAffiliationFactory, SalariedClassificationFactory,
};
use tx_app::{Response, Transaction};

// ユースケース: AddSalariedEmployee トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddSalariedEmployeeTx<T, F>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    name: String,
    address: String,
    salary: f32,

    dao: T,
    payroll_factory: F,
}
impl<T, F> AddSalariedEmployeeTx<T, F>
where
    T: EmployeeDao,
{
    pub fn new(
        id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
        dao: T,
        payroll_factory: F,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            address: address.to_string(),
            salary,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for AddSalariedEmployeeTx<T, F>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> AddEmployee for AddSalariedEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: SalariedClassificationFactory
        + MonthlyScheduleFactory
        + HoldMethodFactory
        + NoAffiliationFactory,
{
    fn run_tx<'a, G, R>(&'a self, f: G) -> Result<R, UsecaseError>
    where
        G: FnOnce(Self::Ctx<'a>) -> Result<R, DaoError>,
    {
        trace!("run_tx called");
        // 今は DB しかないのでサービスレベルトランザクションが DB のトランザクションと同一視されている
        self.dao()
            .run_tx(f)
            .map_err(UsecaseError::AddEmployeeFailed)
    }

    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_classification(&self) -> Arc<Mutex<dyn PaymentClassification>> {
        self.payroll_factory.mk_classification(self.salary)
    }
    fn get_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>> {
        self.payroll_factory.mk_schedule()
    }
    fn get_method(&self) -> Arc<Mutex<dyn PaymentMethod>> {
        self.payroll_factory.mk_method()
    }
    fn get_affiliation(&self) -> Arc<Mutex<dyn Affiliation>> {
        self.payroll_factory.mk_affiliation()
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for AddSalariedEmployeeTx<T, F>
where
    T: EmployeeDao,
    F: SalariedClassificationFactory
        + MonthlyScheduleFactory
        + HoldMethodFactory
        + NoAffiliationFactory,
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
    use std::sync::{Arc, Mutex};

    use dao::{DaoError, EmployeeDao};
    use payroll_domain::{
        Affiliation, Employee, EmployeeId, MemberId, NoAffiliation, Paycheck,
        PaymentClassification, PaymentMethod, PaymentSchedule,
    };
    use payroll_factory::{
        HoldMethodFactory, MonthlyScheduleFactory, NoAffiliationFactory,
        SalariedClassificationFactory,
    };
    use payroll_impl::{HoldMethod, MonthlySchedule, SalariedClassification};

    #[derive(Debug, Clone)]
    struct Tester {
        expect: Vec<Employee>,
        actual: Arc<Mutex<Vec<Employee>>>,
    }
    impl Tester {
        fn assert(&self) {
            let locked = self.actual.lock().unwrap();
            assert_eq!(locked.len(), self.expect.len());
            for (e, a) in self.expect.iter().zip(locked.iter()) {
                assert_eq!(a.id(), e.id());
                assert_eq!(a.name(), e.name());
                assert_eq!(a.address(), e.address());
                assert!(a
                    .classification()
                    .lock()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<SalariedClassification>()
                    .is_some());
                assert_eq!(
                    a.classification()
                        .lock()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<SalariedClassification>(),
                    e.classification()
                        .lock()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<SalariedClassification>(),
                );
                assert!(a
                    .schedule()
                    .lock()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<MonthlySchedule>()
                    .is_some());
                // 今の MonthlySchedule は特にフィールドがないのでこのテストは不要ではある
                assert_eq!(
                    a.schedule()
                        .lock()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<MonthlySchedule>(),
                    e.schedule()
                        .lock()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<MonthlySchedule>()
                );
                assert!(a
                    .method()
                    .lock()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<HoldMethod>()
                    .is_some());
                // 今の HoldMethod は特にフィールドがないのでこのテストは不要ではある
                assert_eq!(
                    a.method()
                        .lock()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<HoldMethod>(),
                    e.method()
                        .lock()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<HoldMethod>()
                );
                assert!(a
                    .affiliation()
                    .lock()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<NoAffiliation>()
                    .is_some());
                // 今の NoAffiliation は特にフィールドがないのでこのテストは不要ではある
                assert_eq!(
                    a.affiliation()
                        .lock()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<NoAffiliation>(),
                    e.affiliation()
                        .lock()
                        .unwrap()
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
                self.actual.lock().unwrap().push(emp);
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
    }
    impl SalariedClassificationFactory for Tester {
        fn mk_classification(&self, salary: f32) -> Arc<Mutex<dyn PaymentClassification>> {
            Arc::new(Mutex::new(SalariedClassification::new(salary)))
        }
    }
    impl MonthlyScheduleFactory for Tester {
        fn mk_schedule(&self) -> Arc<Mutex<dyn PaymentSchedule>> {
            Arc::new(Mutex::new(MonthlySchedule))
        }
    }
    impl HoldMethodFactory for Tester {
        fn mk_method(&self) -> Arc<Mutex<dyn PaymentMethod>> {
            Arc::new(Mutex::new(HoldMethod))
        }
    }
    impl NoAffiliationFactory for Tester {
        fn mk_affiliation(&self) -> Arc<Mutex<dyn Affiliation>> {
            Arc::new(Mutex::new(NoAffiliation))
        }
    }

    #[test]
    fn test_add_emp() {
        let t = Tester {
            expect: vec![Employee::new(
                1.into(),
                "Bob",
                "Home",
                Arc::new(Mutex::new(SalariedClassification::new(123.0))),
                Arc::new(Mutex::new(MonthlySchedule)),
                Arc::new(Mutex::new(HoldMethod)),
                Arc::new(Mutex::new(NoAffiliation)),
            )],
            actual: Arc::new(Mutex::new(vec![])),
        };

        let tx: Box<dyn tx_app::Transaction> = Box::new(AddSalariedEmployeeTx::new(
            1.into(),
            "Bob",
            "Home",
            123.0,
            t.clone(),
            t.clone(),
        ));
        let _ = tx.execute();

        t.assert();
    }
}
