use anyhow;
use chrono::NaiveDate;
use log::{debug, trace};

use abstract_tx::{ChangeEmployee, UsecaseError};
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
use payroll_impl::HourlyClassification;
use tx_app::{Response, Transaction};

// ユースケース: AddTimeCard トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    date: NaiveDate,
    hours: f32,

    dao: T,
}
impl<T> AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, date: NaiveDate, hours: f32, dao: T) -> Self {
        Self {
            id,
            date,
            hours,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeEmployee for AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    fn run_tx<'a, G, R>(&'a self, f: G) -> Result<R, UsecaseError>
    where
        G: FnOnce(Self::Ctx<'a>) -> Result<R, DaoError>,
    {
        trace!("run_tx called");
        // 今は DB しかないのでサービスレベルトランザクションが DB のトランザクションと同一視されている
        self.dao()
            .run_tx(f)
            .map_err(UsecaseError::ChangeEmployeeFailed)
    }

    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn change(&self, emp: &mut Employee) -> Result<(), DaoError> {
        trace!("change called");
        emp.classification()
            .lock()
            .unwrap()
            .as_any_mut()
            .downcast_mut::<HourlyClassification>()
            .ok_or(DaoError::UnexpectedError(
                "classification is not HourlyClassification".into(),
            ))?
            .add_timecard(self.date, self.hours);
        debug!("timecard added: {:?}", emp.classification());
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T> Transaction for AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("execute called");
        ChangeEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::sync::{Arc, Mutex};

    use dao::{DaoError, EmployeeDao};
    use payroll_domain::{Employee, EmployeeId, MemberId, NoAffiliation, Paycheck};
    use payroll_impl::{HoldMethod, HourlyClassification, WeeklySchedule};

    #[derive(Debug, Clone)]
    enum Call {
        Fetch(EmployeeId),
        Update(Employee),
    }

    #[derive(Debug, Clone)]
    struct Tester {
        expect: Vec<Call>,
        actual: Arc<Mutex<Vec<Call>>>,

        fetched: Arc<Mutex<Vec<Result<Employee, DaoError>>>>,
        updated: Arc<Mutex<Vec<Result<(), DaoError>>>>,
    }
    impl Tester {
        fn assert(&self) {
            let locked = self.actual.lock().unwrap();
            assert_eq!(locked.len(), self.expect.len());
            for (e, a) in self.expect.iter().zip(locked.iter()) {
                match e {
                    Call::Fetch(e) => {
                        if let Call::Fetch(a) = a {
                            assert_eq!(a, e);
                        } else {
                            assert!(false, "unexpected call: {:?}", a);
                        }
                    }
                    Call::Update(e) => {
                        if let Call::Update(a) = a {
                            assert_eq!(a.id(), e.id());
                            assert_eq!(a.name(), e.name());
                            assert_eq!(a.address(), e.address());
                            assert_eq!(
                                a.classification()
                                    .lock()
                                    .unwrap()
                                    .as_any()
                                    .downcast_ref::<HourlyClassification>(),
                                e.classification()
                                    .lock()
                                    .unwrap()
                                    .as_any()
                                    .downcast_ref::<HourlyClassification>(),
                            );
                            assert!(a
                                .schedule()
                                .lock()
                                .unwrap()
                                .as_any()
                                .downcast_ref::<WeeklySchedule>()
                                .is_some());
                            // 今の WeeklySchedule は特にフィールドがないのでこのテストは不要ではある
                            assert_eq!(
                                a.schedule()
                                    .lock()
                                    .unwrap()
                                    .as_any()
                                    .downcast_ref::<WeeklySchedule>(),
                                e.schedule()
                                    .lock()
                                    .unwrap()
                                    .as_any()
                                    .downcast_ref::<WeeklySchedule>()
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
                        } else {
                            assert!(false, "unexpected call: {:?}", a);
                        }
                    }
                }
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
            _emp: Employee,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = EmployeeId, Err = DaoError> {
            tx_rs::with_tx(move |_ctx| unreachable!("add method should not be called"))
        }
        fn delete<'a>(
            &self,
            _id: EmployeeId,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_ctx| unreachable!("delete method should not be called"))
        }

        fn fetch<'a>(
            &self,
            id: EmployeeId,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Employee, Err = DaoError> {
            tx_rs::with_tx(move |_ctx| {
                self.actual.lock().unwrap().push(Call::Fetch(id));
                self.fetched.lock().unwrap().pop().unwrap()
            })
        }

        fn fetch_all<'a>(
            &self,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Vec<(EmployeeId, Employee)>, Err = DaoError>
        {
            tx_rs::with_tx(move |_ctx| unreachable!("fetch_all method should not be called"))
        }

        fn update<'a>(
            &self,
            emp: Employee,
        ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |_ctx| {
                self.actual.lock().unwrap().push(Call::Update(emp));
                self.updated.lock().unwrap().pop().unwrap()
            })
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

    #[test]
    fn test_add_timecard() {
        let mut hc = HourlyClassification::new(12.0);
        hc.add_timecard(NaiveDate::from_ymd_opt(2025, 3, 5).unwrap(), 8.0);
        let t = Tester {
            expect: vec![
                Call::Fetch(1.into()),
                Call::Update(Employee::new(
                    1.into(),
                    "Bob",
                    "Home",
                    Arc::new(Mutex::new(hc)),
                    Arc::new(Mutex::new(WeeklySchedule)),
                    Arc::new(Mutex::new(HoldMethod)),
                    Arc::new(Mutex::new(NoAffiliation)),
                )),
            ],
            actual: Arc::new(Mutex::new(vec![])),

            fetched: Arc::new(Mutex::new(vec![Ok(Employee::new(
                1.into(),
                "Bob",
                "Home",
                Arc::new(Mutex::new(HourlyClassification::new(12.0))),
                Arc::new(Mutex::new(WeeklySchedule)),
                Arc::new(Mutex::new(HoldMethod)),
                Arc::new(Mutex::new(NoAffiliation)),
            ))])),
            updated: Arc::new(Mutex::new(vec![Ok(())])),
        };

        let tx: Box<dyn tx_app::Transaction> = Box::new(AddTimeCardTx::new(
            1.into(),
            NaiveDate::from_ymd_opt(2025, 3, 5).unwrap(),
            8.0,
            t.clone(),
        ));
        let _ = tx.execute();

        t.assert();
    }
}
