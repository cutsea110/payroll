use chrono::NaiveDate;
use dyn_clone::DynClone;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, ops::RangeInclusive, rc::Rc};
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

#[derive(Debug, Clone)]
enum DaoError {
    Unknown,
}

trait EmployeeDao<Ctx> {
    fn insert(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
}
trait HaveEmployeeDao<Ctx> {
    fn dao(&self) -> Box<&impl EmployeeDao<Ctx>>;
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

trait ITransaction<Ctx> {
    fn execute(&self, ctx: &mut Ctx);
}
trait IEmployeeCreatable {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
}

impl<T, Ctx> ITransaction<Ctx> for T
where
    T: IEmployeeCreatable + HaveEmployeeDao<Ctx>, // a.k.a IAddEmployeeTransaction
{
    fn execute(&self, ctx: &mut Ctx) {
        let emp = Employee::new(
            self.get_emp_id(),
            self.get_name(),
            self.get_address(),
            self.get_classification(),
            self.get_schedule(),
        );
        self.dao().insert(emp).run(ctx).expect("insert employee");
    }
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
impl<T, Ctx> HaveEmployeeDao<Ctx> for AddSalariedEmployeeTransaction<T, Ctx>
where
    T: EmployeeDao<Ctx>,
{
    fn dao(&self) -> Box<&impl EmployeeDao<Ctx>> {
        Box::new(&self.dao)
    }
}
impl<T, Ctx> IEmployeeCreatable for AddSalariedEmployeeTransaction<T, Ctx>
where
    T: EmployeeDao<Ctx>,
{
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
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
struct AddHourlyEmployeeTransaction<T, Ctx>
where
    T: EmployeeDao<Ctx>,
{
    emp_id: EmployeeId,
    name: String,
    address: String,
    hourly_rate: f32,

    dao: T,
    _phantom: std::marker::PhantomData<Ctx>,
}
impl<T, Ctx> AddHourlyEmployeeTransaction<T, Ctx>
where
    T: EmployeeDao<Ctx>,
{
    fn new(emp_id: EmployeeId, name: &str, address: &str, hourly_rate: f32, dao: T) -> Self {
        Self {
            emp_id,
            name: name.to_string(),
            address: address.to_string(),
            hourly_rate,
            dao,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<T, Ctx> HaveEmployeeDao<Ctx> for AddHourlyEmployeeTransaction<T, Ctx>
where
    T: EmployeeDao<Ctx>,
{
    fn dao(&self) -> Box<&impl EmployeeDao<Ctx>> {
        Box::new(&self.dao)
    }
}
impl<T, Ctx> IEmployeeCreatable for AddHourlyEmployeeTransaction<T, Ctx>
where
    T: EmployeeDao<Ctx>,
{
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }

    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        Rc::new(RefCell::new(HourlyClassification {
            hourly_rate: self.hourly_rate,
        }))
    }
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
        Rc::new(RefCell::new(WeeklySchedule))
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
        tx_rs::with_tx(move |_| {
            let emp_id = emp.id;
            self.employees.borrow_mut().insert(emp_id, emp);
            Ok(emp_id)
        })
    }
}

fn main() {
    let db = PayrollDatabase::new();
    let tx: &dyn ITransaction<()> =
        &AddSalariedEmployeeTransaction::new(1, "Bob", "Home", 1000.0, db.clone());
    println!("Before: {:#?}", db);
    tx.execute(&mut ());
    println!("After: {:#?}", db);

    let tx: &dyn ITransaction<()> =
        &AddHourlyEmployeeTransaction::new(2, "Alice", "Home", 10.0, db.clone());
    println!("Before: {:#?}", db);
    tx.execute(&mut ());
    println!("After: {:#?}", db);
}
