use chrono::NaiveDate;
use dyn_clone::DynClone;
use log::trace;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, ops::RangeInclusive, rc::Rc};
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

trait Transaction<Ctx> {
    fn execute(&self, ctx: &mut Ctx);
}

#[derive(Debug, Clone)]
struct AddSalariedEmployeeTransaction<T, Ctx>
where
    T: EmployeeDao<Ctx>,
{
    emp_id: EmployeeId,
    name: String,
    address: String,
    salary: f32,

    dao: T,
    _phantom: std::marker::PhantomData<Ctx>,
}
impl<T, Ctx> AddSalariedEmployeeTransaction<T, Ctx>
where
    T: EmployeeDao<Ctx>,
{
    fn new(emp_id: EmployeeId, name: &str, address: &str, salary: f32, dao: T) -> Self {
        Self {
            emp_id,
            name: name.to_string(),
            address: address.to_string(),
            salary,

            dao,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<T, Ctx> Transaction<Ctx> for AddSalariedEmployeeTransaction<T, Ctx>
where
    T: EmployeeDao<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx) {
        let emp = Employee::new(
            self.emp_id,
            &self.name,
            &self.address,
            Rc::new(RefCell::new(SalariedClassification {
                salary: self.salary,
            })),
            Rc::new(RefCell::new(MonthlySchedule)),
        );
        trace!("Inserting employee: {:?}", emp);
        self.dao.insert(emp).run(ctx).expect("insert employee");
    }
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
impl EmployeeDao<()> for PayrollDatabase {
    fn insert(&self, emp: Employee) -> impl tx_rs::Tx<(), Item = EmployeeId, Err = DaoError> {
        tx_rs::with_tx(move |ctx| {
            let emp_id = emp.id;
            let mut employees = self.employees.borrow_mut();
            if employees.contains_key(&emp_id) {
                return Err(DaoError::AlreadyExists(emp_id));
            }
            employees.insert(emp_id, emp);
            Ok(emp_id)
        })
    }
}

fn main() {
    env_logger::init();

    let db = PayrollDatabase::new();
    let tx = AddSalariedEmployeeTransaction::new(1, "Bob", "Home", 1000.0, db.clone());
    println!("Before: {:#?}", db);
    tx.execute(&mut ());
    println!("After: {:#?}", db);
}
