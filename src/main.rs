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

trait Transaction<Ctx> {
    fn execute(&self, ctx: &mut Ctx);
}

#[derive(Debug, Clone)]
struct AddSalariedEmployee<Db, Ctx>
where
    Db: EmployeeDao<Ctx>,
{
    db: Db,
    phantom: std::marker::PhantomData<Ctx>,

    id: EmployeeId,
    name: String,
    address: String,
    salary: f32,
}
impl<Db, Ctx> AddSalariedEmployee<Db, Ctx>
where
    Db: EmployeeDao<Ctx>,
{
    fn new(id: EmployeeId, name: &str, address: &str, salary: f32, db: Db) -> Self {
        Self {
            db,
            phantom: std::marker::PhantomData,

            id,
            name: name.to_string(),
            address: address.to_string(),
            salary,
        }
    }
}
impl<Db, Ctx> HaveEmployeeDao<Ctx> for AddSalariedEmployee<Db, Ctx>
where
    Db: EmployeeDao<Ctx>,
{
    fn dao(&self) -> Box<&impl EmployeeDao<Ctx>> {
        Box::new(&self.db)
    }
}
impl<Db, Ctx> Transaction<Ctx> for AddSalariedEmployee<Db, Ctx>
where
    Db: EmployeeDao<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx) {
        let emp = Employee::new(
            self.id,
            &self.name,
            &self.address,
            Rc::new(RefCell::new(SalariedClassification {
                salary: self.salary,
            })),
            Rc::new(RefCell::new(MonthlySchedule)),
        );
        let dao = self.dao();
        dao.insert(emp).run(ctx).expect("add salaried employee");
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
            let id = emp.id;
            self.employees.borrow_mut().insert(id, emp);
            Ok(id)
        })
    }
}

fn main() {
    let db = PayrollDatabase::new();
    let tx = AddSalariedEmployee::new(1, "Bob", "123 Main St", 1000.0, db.clone());
    println!("{:#?}", db);
    tx.execute(&mut ());
    println!("{:#?}", db);
}
