use anyhow;
use chrono::NaiveDate;
use log::{debug, trace};

use abstract_tx::ChangeEmployee;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
use payroll_impl::CommissionedClassification;
use tx_app::{Response, Transaction};

// ユースケース: AddSalesReceipt トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    date: NaiveDate,
    amount: f32,

    dao: T,
}
impl<T> AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, date: NaiveDate, amount: f32, dao: T) -> Self {
        Self {
            id,
            date,
            amount,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeEmployee for AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn change(&self, emp: &mut Employee) -> Result<(), DaoError> {
        trace!("change called");
        emp.classification()
            .borrow_mut()
            .as_any_mut()
            .downcast_mut::<CommissionedClassification>()
            .ok_or(DaoError::UnexpectedError(
                "classification is not CommissionedClassification".into(),
            ))?
            .add_sales_receipt(self.date, self.amount);
        debug!("sales receipt added: {:?}", emp.classification().borrow());
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T> Transaction for AddSalesReceiptTx<T>
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
    use std::{cell::RefCell, rc::Rc};

    use dao::{DaoError, EmployeeDao};
    use payroll_domain::{Employee, EmployeeId, MemberId, NoAffiliation, Paycheck};
    use payroll_impl::{BiweeklySchedule, CommissionedClassification, HoldMethod};

    #[derive(Debug, Clone)]
    enum Call {
        Fetch(EmployeeId),
        Update(Employee),
    }

    #[derive(Debug, Clone)]
    struct Tester {
        expect: Vec<Call>,
        actual: Rc<RefCell<Vec<Call>>>,

        fetched: Rc<RefCell<Vec<Result<Employee, DaoError>>>>,
        updated: Rc<RefCell<Vec<Result<(), DaoError>>>>,
    }
    impl Tester {
        fn assert(&self) {
            let borrowed = self.actual.borrow();
            assert_eq!(borrowed.len(), self.expect.len());
            for (e, a) in self.expect.iter().zip(borrowed.iter()) {
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
                                    .borrow()
                                    .as_any()
                                    .downcast_ref::<CommissionedClassification>(),
                                e.classification()
                                    .borrow()
                                    .as_any()
                                    .downcast_ref::<CommissionedClassification>(),
                            );
                            assert!(a
                                .schedule()
                                .borrow()
                                .as_any()
                                .downcast_ref::<BiweeklySchedule>()
                                .is_some());
                            // 今の BiweeklySchedule は特にフィールドがないのでこのテストは不要ではある
                            assert_eq!(
                                a.schedule()
                                    .borrow()
                                    .as_any()
                                    .downcast_ref::<BiweeklySchedule>(),
                                e.schedule()
                                    .borrow()
                                    .as_any()
                                    .downcast_ref::<BiweeklySchedule>()
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
                self.actual.borrow_mut().push(Call::Fetch(id));
                self.fetched.borrow_mut().pop().unwrap()
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
                self.actual.borrow_mut().push(Call::Update(emp));
                self.updated.borrow_mut().pop().unwrap()
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
        let mut cc = CommissionedClassification::new(123.0, 0.01);
        cc.add_sales_receipt(NaiveDate::from_ymd_opt(2025, 3, 5).unwrap(), 1000.0);
        let t = Tester {
            expect: vec![
                Call::Fetch(1.into()),
                Call::Update(Employee::new(
                    1.into(),
                    "Bob",
                    "Home",
                    Rc::new(RefCell::new(cc)),
                    Rc::new(RefCell::new(BiweeklySchedule)),
                    Rc::new(RefCell::new(HoldMethod)),
                    Rc::new(RefCell::new(NoAffiliation)),
                )),
            ],
            actual: Rc::new(RefCell::new(vec![])),

            fetched: Rc::new(RefCell::new(vec![Ok(Employee::new(
                1.into(),
                "Bob",
                "Home",
                Rc::new(RefCell::new(CommissionedClassification::new(123.0, 0.01))),
                Rc::new(RefCell::new(BiweeklySchedule)),
                Rc::new(RefCell::new(HoldMethod)),
                Rc::new(RefCell::new(NoAffiliation)),
            ))])),
            updated: Rc::new(RefCell::new(vec![Ok(())])),
        };

        let tx: Box<dyn tx_app::Transaction> = Box::new(AddSalesReceiptTx::new(
            1.into(),
            NaiveDate::from_ymd_opt(2025, 3, 5).unwrap(),
            1000.0,
            t.clone(),
        ));
        let _ = tx.execute();

        t.assert();
    }
}
