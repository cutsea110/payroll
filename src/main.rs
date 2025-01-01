use chrono::NaiveDate;
use dyn_clone::DynClone;
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    fmt::Debug,
    ops::RangeInclusive,
    rc::Rc,
};
use thiserror::Error;
use tx_rs::Tx;

type EmployeeId = u32;
#[derive(Debug, Clone)]
struct Employee {
    id: EmployeeId,
    name: String,
    address: String,

    classification: Rc<RefCell<dyn PaymentClassification>>,
    schedule: Rc<RefCell<dyn PaymentSchedule>>,
}
impl Employee {
    fn new(
        id: EmployeeId,
        name: &str,
        address: &str,
        classification: Rc<RefCell<dyn PaymentClassification>>,
        schedule: Rc<RefCell<dyn PaymentSchedule>>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            address: address.to_string(),
            classification,
            schedule,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Error)]
enum DaoError {
    #[error("EmployeeId({0}) already exists")]
    AlreadyExists(EmployeeId),
    #[error("EmployeeId({0}) not found")]
    NotFound(EmployeeId),
    #[error("Insert failed: {0}")]
    InsertFailed(String),
}

// Dao
trait EmployeeDao<Ctx> {
    fn insert(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
}

trait PaymentClassification: Debug + DynClone {
    fn calculate_pay(&self) -> f32;
}
dyn_clone::clone_trait_object!(PaymentClassification);

#[derive(Debug, Clone)]
struct SalariedClassification {
    salary: f32,
}
impl PaymentClassification for SalariedClassification {
    fn calculate_pay(&self) -> f32 {
        unimplemented!();
    }
}

#[derive(Debug, Clone)]
struct HourlyClassification {
    hourly_rate: f32,
}
impl PaymentClassification for HourlyClassification {
    fn calculate_pay(&self) -> f32 {
        unimplemented!();
    }
}

#[derive(Debug, Clone)]
struct CommissionedClassification {
    salary: f32,
    commission_rate: f32,
}
impl PaymentClassification for CommissionedClassification {
    fn calculate_pay(&self) -> f32 {
        unimplemented!();
    }
}

trait PaymentSchedule: Debug + DynClone {
    fn is_pay_date(&self, date: NaiveDate) -> bool;
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate>;
}
dyn_clone::clone_trait_object!(PaymentSchedule);

#[derive(Debug, Clone)]
struct MonthlySchedule;
impl PaymentSchedule for MonthlySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        unimplemented!();
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        unimplemented!();
    }
}

#[derive(Debug, Clone)]
struct WeeklySchedule;
impl PaymentSchedule for WeeklySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        unimplemented!();
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        unimplemented!();
    }
}

#[derive(Debug, Clone)]
struct BiweeklySchedule;
impl PaymentSchedule for BiweeklySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        unimplemented!();
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        unimplemented!();
    }
}

trait HaveEmployeeDao<Ctx> {
    fn dao(&self) -> &impl EmployeeDao<Ctx>;
}

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
            ))
            .map_err(|_| UsecaseError::Dummy)
    }
}

// Service
trait AddSalariedEmployeeTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: AddEmployee<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, UsecaseError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<EmployeeId, UsecaseError> {
        self.run_tx(move |usecase, ctx| usecase.execute().run(ctx).map_err(|_| UsecaseError::Dummy))
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

#[derive(Debug, Clone)]
struct PayrollDatabase {
    employees: Rc<RefCell<HashMap<EmployeeId, Employee>>>,
}
impl PayrollDatabase {
    fn new() -> Self {
        Self {
            employees: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}
type PayrollDbCtx<'a> = RefMut<'a, HashMap<EmployeeId, Employee>>;

#[derive(Debug, Clone)]
struct PayrollDbDao;
impl<'a> EmployeeDao<PayrollDbCtx<'a>> for PayrollDbDao {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = EmployeeId, Err = DaoError> {
        tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
            let emp_id = emp.id;
            if tx.contains_key(&emp_id) {
                Err(DaoError::AlreadyExists(emp.id))
            } else {
                tx.insert(emp.id, emp);
                Ok(emp_id)
            }
        })
    }
}

mod add_salaried_emp {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use super::{
        AddEmployee, AddSalariedEmployeeTransaction, EmployeeDao, EmployeeId, HaveEmployeeDao,
        MonthlySchedule, PaymentClassification, PaymentSchedule, PayrollDatabase, PayrollDbCtx,
        PayrollDbDao, SalariedClassification, ServiceError, Transaction, UsecaseError,
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
        fn new(id: EmployeeId, name: String, address: String, salary: f32) -> Self {
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
            Rc::new(RefCell::new(SalariedClassification {
                salary: self.salary,
            }))
        }
        fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
            Rc::new(RefCell::new(MonthlySchedule))
        }
    }

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

    impl<'a> AddSalariedEmployeeTransaction<'a, PayrollDbCtx<'a>> for AddSalariedEmployeeTx {
        type U = AddSalariedEmployeeImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, UsecaseError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.employees.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx)
        }
    }

    impl Transaction for AddSalariedEmployeeTx {
        type T = EmployeeId;
        fn execute(&mut self) -> Result<EmployeeId, ServiceError> {
            AddSalariedEmployeeTransaction::execute(self).map_err(|_| ServiceError::Dummy)
        }
    }
}
use add_salaried_emp::AddSalariedEmployeeTx;

fn main() {
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
}
