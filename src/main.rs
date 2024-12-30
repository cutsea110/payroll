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

trait HaveEmployeeBasicProps {
    fn get_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
}

trait AddEmployeeTransaction<Ctx>: HaveEmployeeBasicProps + HaveEmployeeDao<Ctx> {
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
    fn execute(&self, ctx: &mut Ctx) {
        let emp = Employee::new(
            self.get_id(),
            self.get_name(),
            self.get_address(),
            self.get_classification(),
            self.get_schedule(),
        );
        self.dao().insert(emp).run(ctx).expect("do transaction");
    }
}
impl<Ctx, T> Transaction<Ctx> for T
where
    T: AddEmployeeTransaction<Ctx>,
{
    fn execute(&self, ctx: &mut Ctx) {
        AddEmployeeTransaction::execute(self, ctx);
    }
}

trait AddSalariedEmployee<Ctx> {
    fn get_salary(&self) -> f32;
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        Rc::new(RefCell::new(SalariedClassification {
            salary: self.get_salary(),
        }))
    }
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
        Rc::new(RefCell::new(MonthlySchedule))
    }
}
impl<Ctx, T> AddEmployeeTransaction<Ctx> for T
where
    T: AddSalariedEmployee<Ctx> + HaveEmployeeBasicProps + HaveEmployeeDao<Ctx>,
{
    // override
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        AddSalariedEmployee::get_classification(self)
    }
    // override
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
        AddSalariedEmployee::get_schedule(self)
    }
}

#[derive(Debug, Clone)]
struct MockDb {
    employees: Rc<RefCell<HashMap<EmployeeId, Employee>>>,
}
impl EmployeeDao<()> for MockDb {
    fn insert(&self, emp: Employee) -> impl tx_rs::Tx<(), Item = EmployeeId, Err = DaoError> {
        tx_rs::with_tx(move |_| {
            let id = emp.id;
            self.employees.borrow_mut().insert(id, emp);
            Ok(id)
        })
    }
}

#[derive(Debug, Clone)]
struct AddSalariedEmployeeTxMock {
    db: MockDb,

    emp_id: EmployeeId,
    name: String,
    address: String,
    salary: f32,
}
impl HaveEmployeeDao<()> for AddSalariedEmployeeTxMock {
    fn dao(&self) -> Box<&impl EmployeeDao<()>> {
        Box::new(&self.db)
    }
}
impl HaveEmployeeBasicProps for AddSalariedEmployeeTxMock {
    fn get_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_address(&self) -> &str {
        &self.address
    }
}
impl AddSalariedEmployee<()> for AddSalariedEmployeeTxMock {
    fn get_salary(&self) -> f32 {
        self.salary
    }
}

fn main() {
    let db = MockDb {
        employees: Rc::new(RefCell::new(HashMap::new())),
    };
    let tx = AddSalariedEmployeeTxMock {
        db: db.clone(),
        emp_id: 1,
        name: "Bob".to_string(),
        address: "123 Main St.".to_string(),
        salary: 1000.0,
    };
    println!("BEFORE: {:#?}", db);
    Transaction::execute(&tx, &mut ());
    println!("AFTER: {:#?}", db);
}
