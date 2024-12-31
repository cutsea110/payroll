use chrono::NaiveDate;
use dyn_clone::DynClone;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    ops::RangeInclusive,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
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

#[derive(Debug, Clone)]
struct PayrollDatabase {
    employees: Arc<Mutex<HashMap<EmployeeId, Employee>>>,
}
impl PayrollDatabase {
    fn new() -> Self {
        Self {
            employees: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[derive(Debug, Clone)]
struct PayrollDbDao;
impl<'a> EmployeeDao<MutexGuard<'a, HashMap<EmployeeId, Employee>>> for PayrollDbDao {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<MutexGuard<'a, HashMap<EmployeeId, Employee>>, Item = EmployeeId, Err = DaoError>
    {
        tx_rs::with_tx(
            move |tx: &mut MutexGuard<'a, HashMap<EmployeeId, Employee>>| {
                let emp_id = emp.id;
                if tx.contains_key(&emp_id) {
                    Err(DaoError::AlreadyExists(emp.id))
                } else {
                    tx.insert(emp.id, emp);
                    Ok(emp_id)
                }
            },
        )
    }
}

#[derive(Debug, Clone)]
struct AddSalariedEmployeeImpl {
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
impl<'a> HaveEmployeeDao<MutexGuard<'a, HashMap<EmployeeId, Employee>>>
    for AddSalariedEmployeeImpl
{
    fn dao(&self) -> &impl EmployeeDao<MutexGuard<'a, HashMap<EmployeeId, Employee>>> {
        &self.dao
    }
}
impl<'a> AddEmployee<MutexGuard<'a, HashMap<EmployeeId, Employee>>> for AddSalariedEmployeeImpl {
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
struct AddSalariedEmployeeTx {
    db: PayrollDatabase,
    usecase: RefCell<AddSalariedEmployeeImpl>,
}
impl AddSalariedEmployeeTx {
    fn new(
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

impl<'a> AddSalariedEmployeeTransaction<'a, MutexGuard<'a, HashMap<EmployeeId, Employee>>>
    for AddSalariedEmployeeTx
{
    type U = AddSalariedEmployeeImpl;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, UsecaseError>
    where
        F: FnOnce(
            &mut Self::U,
            &mut MutexGuard<'a, HashMap<EmployeeId, Employee>>,
        ) -> Result<T, UsecaseError>,
    {
        if let Ok(mut tx) = self.db.employees.lock() {
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx)
        } else {
            Err(UsecaseError::Dummy)
        }
    }
}

fn main() {
    let db = PayrollDatabase::new();
    let mut tx =
        AddSalariedEmployeeTx::new(1, "Bob".to_string(), "Home".to_string(), 1000.0, db.clone());
    println!("{:#?}", db);
    AddSalariedEmployeeTransaction::execute(&mut tx).expect("register employee Bob");
    println!("{:#?}", db);
}
