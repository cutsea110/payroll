use std::{cell::RefCell, cell::RefMut, collections::HashMap, fmt::Debug, rc::Rc};

use dao::{DaoError, EmployeeDao};
use payroll_domain::{Employee, EmployeeId, MemberId, Paycheck};

#[derive(Debug, Clone)]
pub struct PayrollDatabase {
    employees: HashMap<EmployeeId, Employee>,
    union_members: HashMap<MemberId, EmployeeId>,
    paychecks: Rc<RefCell<HashMap<EmployeeId, Vec<Paycheck>>>>,
}
impl PayrollDatabase {
    pub fn new() -> Self {
        Self {
            employees: HashMap::new(),
            union_members: HashMap::new(),
            paychecks: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}
pub type PayrollDbCtx<'a> = RefMut<'a, PayrollDatabase>;

#[derive(Debug, Clone)]
pub struct PayrollDbDao;
impl<'a> EmployeeDao<PayrollDbCtx<'a>> for PayrollDbDao {
    fn insert(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = EmployeeId, Err = DaoError> {
        tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
            let emp_id = emp.emp_id();
            if tx.employees.contains_key(&emp_id) {
                Err(DaoError::AlreadyExists(emp_id))
            } else {
                tx.employees.insert(emp_id, emp);
                Ok(emp_id)
            }
        })
    }
    fn remove(
        &self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
        tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
            if tx.employees.contains_key(&emp_id) {
                tx.employees.remove(&emp_id);
                Ok(())
            } else {
                Err(DaoError::NotFound(emp_id))
            }
        })
    }
    fn fetch(
        &self,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = Employee, Err = DaoError> {
        tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
            tx.employees
                .get(&emp_id)
                .cloned()
                .ok_or(DaoError::NotFound(emp_id))
        })
    }
    fn fetch_all(
        &self,
    ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = Vec<(EmployeeId, Employee)>, Err = DaoError> {
        tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
            Ok(tx.employees.iter().map(|(k, v)| (*k, v.clone())).collect())
        })
    }
    fn update(&self, emp: Employee) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
        tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
            let emp_id = emp.emp_id();
            if tx.employees.contains_key(&emp_id) {
                tx.employees.insert(emp_id, emp);
                Ok(())
            } else {
                Err(DaoError::NotFound(emp_id))
            }
        })
    }
    fn add_union_member(
        &self,
        member_id: MemberId,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
        tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
            if tx.union_members.contains_key(&member_id) {
                return Err(DaoError::AlreadyUnionMember(member_id, emp_id));
            }
            tx.union_members.insert(member_id, emp_id);
            Ok(())
        })
    }
    fn remove_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
        tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
            if tx.union_members.remove(&member_id).is_none() {
                return Err(DaoError::NotYetUnionMember(member_id));
            }
            Ok(())
        })
    }
    fn find_union_member(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = EmployeeId, Err = DaoError> {
        tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
            tx.union_members
                .get(&member_id)
                .cloned()
                .ok_or(DaoError::NotYetUnionMember(member_id))
        })
    }
    fn record_paycheck(
        &self,
        emp_id: EmployeeId,
        pc: Paycheck,
    ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
        tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
            tx.paychecks
                .borrow_mut()
                .entry(emp_id)
                .or_insert(vec![])
                .push(pc);
            Ok(())
        })
    }
}
