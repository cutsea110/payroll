use std::{cell::RefCell, fmt::Debug, rc::Rc};
use thiserror::Error;
use tx_rs::Tx;

mod payroll_domain {
    mod types {
        pub type EmployeeId = u32;
    }
    pub use types::*;

    mod bo {
        use chrono::NaiveDate;
        use std::{cell::RefCell, fmt::Debug, ops::RangeInclusive, rc::Rc};

        use crate::{
            Affiliation, EmployeeId, PaymentClassification, PaymentMethod, PaymentSchedule,
        };

        #[derive(Debug, Clone)]
        pub struct Employee {
            id: EmployeeId,
            name: String,
            address: String,

            classification: Rc<RefCell<dyn PaymentClassification>>,
            schedule: Rc<RefCell<dyn PaymentSchedule>>,
            method: Rc<RefCell<dyn PaymentMethod>>,
            affiliation: Rc<RefCell<dyn Affiliation>>,
        }
        impl Employee {
            pub fn new(
                id: EmployeeId,
                name: &str,
                address: &str,
                classification: Rc<RefCell<dyn PaymentClassification>>,
                schedule: Rc<RefCell<dyn PaymentSchedule>>,
                method: Rc<RefCell<dyn PaymentMethod>>,
                affiliation: Rc<RefCell<dyn Affiliation>>,
            ) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    classification,
                    schedule,
                    method,
                    affiliation,
                }
            }
            pub fn emp_id(&self) -> EmployeeId {
                self.id
            }
            pub fn set_name(&mut self, name: &str) {
                self.name = name.to_string();
            }
            pub fn set_classification(
                &mut self,
                classification: Rc<RefCell<dyn PaymentClassification>>,
            ) {
                self.classification = classification;
            }
            pub fn set_schedule(&mut self, schedule: Rc<RefCell<dyn PaymentSchedule>>) {
                self.schedule = schedule;
            }
        }

        #[derive(Debug, Clone)]
        pub struct Paycheck {
            period: RangeInclusive<NaiveDate>,

            gross_pay: f32,
            deductions: f32,
            net_pay: f32,
        }
        impl Paycheck {
            pub fn new(period: RangeInclusive<NaiveDate>) -> Self {
                Self {
                    period,
                    gross_pay: 0.0,
                    deductions: 0.0,
                    net_pay: 0.0,
                }
            }
        }
    }
    pub use bo::*;

    mod interface {
        mod payment_classification {
            use dyn_clone::DynClone;
            use std::fmt::Debug;

            pub trait PaymentClassification: Debug + DynClone {
                fn calculate_pay(&self) -> f32;
            }
            dyn_clone::clone_trait_object!(PaymentClassification);
        }
        pub use payment_classification::*;

        mod payment_schedule {
            use chrono::NaiveDate;
            use dyn_clone::DynClone;
            use std::{fmt::Debug, ops::RangeInclusive};

            pub trait PaymentSchedule: Debug + DynClone {
                fn is_pay_date(&self, date: NaiveDate) -> bool;
                fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate>;
            }
            dyn_clone::clone_trait_object!(PaymentSchedule);
        }
        pub use payment_schedule::*;

        mod payment_method {
            use dyn_clone::DynClone;
            use std::fmt::Debug;

            use crate::Paycheck;

            pub trait PaymentMethod: Debug + DynClone {
                // TODO: return type
                fn pay(&self, pc: &Paycheck);
            }
            dyn_clone::clone_trait_object!(PaymentMethod);
        }
        pub use payment_method::*;

        mod affiliation {
            use dyn_clone::DynClone;
            use std::fmt::Debug;

            pub trait Affiliation: Debug + DynClone {
                fn calculate_deductions(&self) -> f32 {
                    0.0
                }
            }
            dyn_clone::clone_trait_object!(Affiliation);
        }
        pub use affiliation::*;
    }
    pub use interface::*;
}
use payroll_domain::*;

mod payroll_impl {
    mod classification {
        use crate::payroll_domain::PaymentClassification;

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
            fn calculate_pay(&self) -> f32 {
                unimplemented!();
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
            fn calculate_pay(&self) -> f32 {
                unimplemented!();
            }
        }

        #[derive(Debug, Clone)]
        pub struct CommissionedClassification {
            salary: f32,
            commission_rate: f32,
        }
        impl PaymentClassification for CommissionedClassification {
            fn calculate_pay(&self) -> f32 {
                unimplemented!();
            }
        }
    }
    pub use classification::*;

    mod schedule {
        use chrono::NaiveDate;
        use std::{fmt::Debug, ops::RangeInclusive};

        use crate::payroll_domain::PaymentSchedule;

        #[derive(Debug, Clone)]
        pub struct MonthlySchedule;
        impl PaymentSchedule for MonthlySchedule {
            fn is_pay_date(&self, date: NaiveDate) -> bool {
                unimplemented!();
            }
            fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
                unimplemented!();
            }
        }

        #[derive(Debug, Clone)]
        pub struct WeeklySchedule;
        impl PaymentSchedule for WeeklySchedule {
            fn is_pay_date(&self, date: NaiveDate) -> bool {
                unimplemented!();
            }
            fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
                unimplemented!();
            }
        }

        #[derive(Debug, Clone)]
        pub struct BiweeklySchedule;
        impl PaymentSchedule for BiweeklySchedule {
            fn is_pay_date(&self, date: NaiveDate) -> bool {
                unimplemented!();
            }
            fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
                unimplemented!();
            }
        }
    }
    pub use schedule::*;

    mod method {
        use crate::{Paycheck, PaymentMethod};

        #[derive(Debug, Clone)]
        pub struct HoldMethod;
        impl PaymentMethod for HoldMethod {
            fn pay(&self, pc: &Paycheck) {
                unimplemented!();
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
                unimplemented!();
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
                unimplemented!();
            }
        }
    }
    pub use method::*;

    mod affiliation {
        use crate::Affiliation;

        #[derive(Debug, Clone)]
        pub struct NoAffiliation;
        impl Affiliation for NoAffiliation {
            fn calculate_deductions(&self) -> f32 {
                unimplemented!();
            }
        }

        #[derive(Debug, Clone)]
        pub struct UnionAffiliation {
            member_id: i32,
            dues: f32,
        }
        impl UnionAffiliation {
            pub fn new(member_id: i32, dues: f32) -> Self {
                Self { member_id, dues }
            }
        }
        impl Affiliation for UnionAffiliation {
            fn calculate_deductions(&self) -> f32 {
                unimplemented!();
            }
        }
    }
    pub use affiliation::*;
}
use payroll_impl::*;

mod dao {
    use thiserror::Error;

    use crate::payroll_domain::{Employee, EmployeeId};

    #[derive(Debug, Clone, Eq, PartialEq, Error)]
    pub enum DaoError {
        #[error("EmployeeId({0}) already exists")]
        AlreadyExists(EmployeeId),
        #[error("EmployeeId({0}) not found")]
        NotFound(EmployeeId),
        #[error("Insert failed: {0}")]
        InsertFailed(String),
    }

    pub trait EmployeeDao<Ctx> {
        fn insert(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
        fn fetch(&self, emp_id: EmployeeId)
            -> impl tx_rs::Tx<Ctx, Item = Employee, Err = DaoError>;
        fn update(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    }

    pub trait HaveEmployeeDao<Ctx> {
        fn dao(&self) -> &impl EmployeeDao<Ctx>;
    }
}
use dao::*;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
enum UsecaseError {
    #[error("dummy error")]
    Dummy,
}
// Usecase
trait AddEmployee<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        self.dao()
            .insert(Employee::new(
                self.get_emp_id(),
                self.get_name(),
                self.get_address(),
                self.get_classification(),
                self.get_schedule(),
                Rc::new(RefCell::new(HoldMethod)),
                Rc::new(RefCell::new(NoAffiliation)),
            ))
            .map_err(|_| UsecaseError::Dummy)
    }
}
trait ChgEmployee<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn change(&self, emp: &mut Employee);

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp_id = self.get_emp_id();
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .map_err(|_| UsecaseError::Dummy)
                .run(ctx)?;
            self.change(&mut emp);
            self.dao()
                .update(emp)
                .map_err(|_| UsecaseError::Dummy)
                .run(ctx)
        })
    }
}

// Service
trait AddEmployeeTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: AddEmployee<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<EmployeeId, ServiceError> {
        self.run_tx(move |usecase, ctx| usecase.execute().run(ctx))
    }
}

trait ChgEmployeeTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: ChgEmployee<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(move |usecase, ctx| usecase.execute().run(ctx))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
enum ServiceError {
    #[error("dummy error")]
    Dummy,
}

trait Transaction {
    type T;
    fn execute(&mut self) -> Result<Self::T, ServiceError>;
}

mod payroll_db {
    use std::{
        cell::{RefCell, RefMut},
        collections::HashMap,
        fmt::Debug,
        rc::Rc,
    };

    use crate::{DaoError, Employee, EmployeeDao, EmployeeId};

    #[derive(Debug, Clone)]
    pub struct PayrollDatabase {
        employees: Rc<RefCell<HashMap<EmployeeId, Employee>>>,
    }
    impl PayrollDatabase {
        pub fn new() -> Self {
            Self {
                employees: Rc::new(RefCell::new(HashMap::new())),
            }
        }
        pub fn transaction_employees(&self) -> RefMut<HashMap<EmployeeId, Employee>> {
            self.employees.borrow_mut()
        }
    }
    pub type PayrollDbCtx<'a> = RefMut<'a, HashMap<EmployeeId, Employee>>;

    #[derive(Debug, Clone)]
    pub struct PayrollDbDao;
    impl<'a> EmployeeDao<PayrollDbCtx<'a>> for PayrollDbDao {
        fn insert(
            &self,
            emp: Employee,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = EmployeeId, Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                let emp_id = emp.emp_id();
                if tx.contains_key(&emp_id) {
                    Err(DaoError::AlreadyExists(emp_id))
                } else {
                    tx.insert(emp_id, emp);
                    Ok(emp_id)
                }
            })
        }
        fn fetch(
            &self,
            emp_id: EmployeeId,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = Employee, Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                tx.get(&emp_id).cloned().ok_or(DaoError::NotFound(emp_id))
            })
        }
        fn update(
            &self,
            emp: Employee,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                let emp_id = emp.emp_id();
                if tx.contains_key(&emp_id) {
                    tx.insert(emp_id, emp);
                    Ok(())
                } else {
                    Err(DaoError::NotFound(emp_id))
                }
            })
        }
    }
}
use payroll_db::*;

mod tx_impl {
    mod add_salaried_emp {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            AddEmployee, EmployeeDao, EmployeeId, HaveEmployeeDao, MonthlySchedule,
            PaymentClassification, PaymentSchedule, SalariedClassification,
        };

        #[derive(Debug, Clone)]
        pub struct AddSalariedEmployeeImpl {
            id: EmployeeId,
            name: String,
            address: String,
            salary: f32,

            dao: PayrollDbDao,
        }
        impl AddSalariedEmployeeImpl {
            pub fn new(id: EmployeeId, name: String, address: String, salary: f32) -> Self {
                Self {
                    id,
                    name,
                    address,
                    salary,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for AddSalariedEmployeeImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> AddEmployee<PayrollDbCtx<'a>> for AddSalariedEmployeeImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_name(&self) -> &str {
                self.name.as_str()
            }
            fn get_address(&self) -> &str {
                self.address.as_str()
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(SalariedClassification::new(self.salary)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(MonthlySchedule))
            }
        }
    }
    pub use add_salaried_emp::*;

    mod chg_emp_name {
        use std::fmt::Debug;

        use crate::{
            payroll_db::PayrollDbDao, ChgEmployee, Employee, EmployeeDao, EmployeeId,
            HaveEmployeeDao, PayrollDbCtx,
        };

        #[derive(Debug, Clone)]
        pub struct ChgEmployeeNameImpl {
            id: EmployeeId,
            new_name: String,

            dao: PayrollDbDao,
        }
        impl ChgEmployeeNameImpl {
            pub fn new(id: EmployeeId, new_name: String) -> Self {
                Self {
                    id,
                    new_name,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for ChgEmployeeNameImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> ChgEmployee<PayrollDbCtx<'a>> for ChgEmployeeNameImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn change(&self, emp: &mut Employee) {
                emp.set_name(self.new_name.as_str());
            }
        }
    }
    pub use chg_emp_name::*;

    mod chg_hourly_emp {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            payroll_db::PayrollDbDao, ChgEmployee, Employee, EmployeeDao, EmployeeId,
            HaveEmployeeDao, HourlyClassification, PayrollDbCtx, WeeklySchedule,
        };

        #[derive(Debug, Clone)]
        pub struct ChgHourlyEmployeeImpl {
            id: EmployeeId,
            hourly_rate: f32,

            dao: PayrollDbDao,
        }
        impl ChgHourlyEmployeeImpl {
            pub fn new(id: EmployeeId, hourly_rate: f32) -> Self {
                Self {
                    id,
                    hourly_rate,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for ChgHourlyEmployeeImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> ChgEmployee<PayrollDbCtx<'a>> for ChgHourlyEmployeeImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn change(&self, emp: &mut Employee) {
                emp.set_classification(Rc::new(RefCell::new(HourlyClassification::new(
                    self.hourly_rate,
                ))));
                emp.set_schedule(Rc::new(RefCell::new(WeeklySchedule)));
            }
        }
    }
    pub use chg_hourly_emp::*;
}
use tx_impl::*;

mod mock_tx_impl {
    mod add_salaried_emp {
        use std::{cell::RefCell, fmt::Debug};

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            AddEmployeeTransaction, AddSalariedEmployeeImpl, EmployeeId, ServiceError, Transaction,
            UsecaseError,
        };

        #[derive(Debug, Clone)]
        pub struct AddSalariedEmployeeTx {
            db: PayrollDatabase,
            usecase: RefCell<AddSalariedEmployeeImpl>,
        }
        impl AddSalariedEmployeeTx {
            pub fn new(
                id: EmployeeId,
                name: String,
                address: String,
                salary: f32,
                db: PayrollDatabase,
            ) -> Self {
                Self {
                    db,
                    usecase: RefCell::new(AddSalariedEmployeeImpl::new(id, name, address, salary)),
                }
            }
        }

        impl<'a> AddEmployeeTransaction<'a, PayrollDbCtx<'a>> for AddSalariedEmployeeTx {
            type U = AddSalariedEmployeeImpl;

            fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
            where
                F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
            {
                let mut tx = self.db.transaction_employees();
                let mut usecase = self.usecase.borrow_mut();
                f(&mut usecase, &mut tx).map_err(|_| ServiceError::Dummy)
            }
        }

        impl Transaction for AddSalariedEmployeeTx {
            type T = EmployeeId;
            fn execute(&mut self) -> Result<EmployeeId, ServiceError> {
                AddEmployeeTransaction::execute(self).map_err(|_| ServiceError::Dummy)
            }
        }
    }
    pub use add_salaried_emp::*;

    mod chg_emp_name {
        use std::{cell::RefCell, fmt::Debug};

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            ChgEmployeeNameImpl, ChgEmployeeTransaction, EmployeeId, ServiceError, Transaction,
            UsecaseError,
        };

        #[derive(Debug, Clone)]
        pub struct ChgEmployeeNameTx {
            db: PayrollDatabase,
            usecase: RefCell<ChgEmployeeNameImpl>,
        }
        impl ChgEmployeeNameTx {
            pub fn new(id: EmployeeId, new_name: String, db: PayrollDatabase) -> Self {
                Self {
                    db,
                    usecase: RefCell::new(ChgEmployeeNameImpl::new(id, new_name)),
                }
            }
        }

        impl<'a> ChgEmployeeTransaction<'a, PayrollDbCtx<'a>> for ChgEmployeeNameTx {
            type U = ChgEmployeeNameImpl;

            fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
            where
                F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
            {
                let mut tx = self.db.transaction_employees();
                let mut usecase = self.usecase.borrow_mut();
                f(&mut usecase, &mut tx).map_err(|_| ServiceError::Dummy)
            }
        }

        impl Transaction for ChgEmployeeNameTx {
            type T = ();
            fn execute(&mut self) -> Result<(), ServiceError> {
                ChgEmployeeTransaction::execute(self).map_err(|_| ServiceError::Dummy)
            }
        }
    }
    pub use chg_emp_name::*;

    mod chg_hourly_emp {
        use std::cell::RefCell;

        use crate::{
            ChgEmployeeTransaction, ChgHourlyEmployeeImpl, EmployeeId, PayrollDatabase,
            PayrollDbCtx, ServiceError, Transaction, UsecaseError,
        };

        #[derive(Debug, Clone)]
        pub struct ChgHourlyClassificationTx {
            db: PayrollDatabase,
            usecase: RefCell<ChgHourlyEmployeeImpl>,
        }
        impl ChgHourlyClassificationTx {
            pub fn new(id: EmployeeId, hourly_rate: f32, db: PayrollDatabase) -> Self {
                Self {
                    db,
                    usecase: RefCell::new(ChgHourlyEmployeeImpl::new(id, hourly_rate)),
                }
            }
        }

        impl<'a> ChgEmployeeTransaction<'a, PayrollDbCtx<'a>> for ChgHourlyClassificationTx {
            type U = ChgHourlyEmployeeImpl;

            fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
            where
                F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
            {
                let mut tx = self.db.transaction_employees();
                let mut usecase = self.usecase.borrow_mut();
                f(&mut usecase, &mut tx).map_err(|_| ServiceError::Dummy)
            }
        }

        impl Transaction for ChgHourlyClassificationTx {
            type T = ();
            fn execute(&mut self) -> Result<(), ServiceError> {
                ChgEmployeeTransaction::execute(self).map_err(|_| ServiceError::Dummy)
            }
        }
    }
    pub use chg_hourly_emp::*;
}
use mock_tx_impl::*;

fn main() {
    env_logger::init();

    let db = PayrollDatabase::new();
    let tx: &mut dyn Transaction<T = _> = &mut AddSalariedEmployeeTx::new(
        1,
        "Bob".to_string(),
        "Home".to_string(),
        1000.0,
        db.clone(),
    );
    println!("{:#?}", db);
    Transaction::execute(tx).expect("register employee Bob");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgEmployeeNameTx::new(1, "Alice".to_string(), db.clone());
    Transaction::execute(tx).expect("change employee name");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut ChgHourlyClassificationTx::new(1, 10.0, db.clone());
    Transaction::execute(tx).expect("change employee to hourly");
    println!("{:#?}", db);
}
