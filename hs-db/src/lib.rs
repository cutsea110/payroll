// dao の具体的な実装
use log::trace;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use dao::{DaoError, EmployeeDao};
use payroll_domain::{Employee, EmployeeId, MemberId, Paycheck};

#[derive(Debug, Clone)]
pub struct HashDB {
    // HashDB を DBMS として PayrollDb が DB(テーブルの集合) を表現
    payroll_db: Arc<Mutex<PayrollDb>>,
}
impl HashDB {
    pub fn new() -> Self {
        let db = PayrollDb {
            employees: HashMap::new(),
            union_members: HashMap::new(),
            paychecks: HashMap::new(),
        };
        Self {
            payroll_db: Arc::new(Mutex::new(db)),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PayrollDb {
    employees: HashMap<EmployeeId, Employee>,
    union_members: HashMap<MemberId, EmployeeId>,
    paychecks: HashMap<EmployeeId, Vec<Paycheck>>,
}
// DB の実装ごとに EmployeeDao トレイトを実装する
impl EmployeeDao for HashDB {
    type Ctx<'a> = MutexGuard<'a, PayrollDb>;

    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, DaoError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>,
    {
        trace!("run_tx called");
        // Mutex の lock が RDB におけるトランザクションに相当
        let locked = self.payroll_db.lock().unwrap();
        f(locked)
    }

    fn add<'a>(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = EmployeeId, Err = DaoError> {
        trace!("add called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            let emp_id = emp.id();
            trace!("add::with_tx called: emp_id={},emp={:?}", emp_id, emp);
            if tx.employees.contains_key(&emp_id) {
                return Err(DaoError::EmployeeAlreadyExists(emp_id));
            }
            tx.employees.insert(emp_id, emp);
            Ok(emp_id)
        })
    }
    fn delete<'a>(
        &self,
        id: EmployeeId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
        trace!("delete called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            trace!("delete::with_tx called: id={}", id);
            if tx.employees.remove(&id).is_some() {
                return Ok(());
            }
            Err(DaoError::EmployeeNotFound(id))
        })
    }
    fn fetch<'a>(
        &self,
        id: EmployeeId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Employee, Err = DaoError> {
        trace!("fetch called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            trace!("fetch::with_tx called: id={}", id);
            tx.employees
                .get(&id)
                .cloned()
                .ok_or(DaoError::EmployeeNotFound(id))
        })
    }
    fn fetch_all<'a>(
        &self,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Vec<(EmployeeId, Employee)>, Err = DaoError> {
        trace!("fetch_all called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            trace!("fetch_all::with_tx called");
            Ok(tx.employees.iter().map(|(k, v)| (*k, v.clone())).collect())
        })
    }
    fn update<'a>(
        &self,
        emp: Employee,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
        trace!("save called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            let emp_id = emp.id();
            trace!("save::with_tx called: emp_id={},emp={:?}", emp_id, emp);
            if tx.employees.contains_key(&emp_id) {
                tx.employees.insert(emp_id, emp);
                return Ok(());
            }
            Err(DaoError::EmployeeNotFound(emp_id))
        })
    }
    fn add_union_member<'a>(
        &self,
        member_id: MemberId,
        emp_id: EmployeeId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
        trace!("add_union_member called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            trace!(
                "add_union_member::with_tx called: member_id={},emp_id={}",
                member_id,
                emp_id
            );
            if tx.union_members.contains_key(&member_id) {
                return Err(DaoError::MemberAlreadyExists(member_id, emp_id));
            }
            tx.union_members.insert(member_id, emp_id);
            Ok(())
        })
    }
    fn delete_union_member<'a>(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
        trace!("delete_union_member called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            trace!(
                "delete_union_member::with_tx called: member_id={}",
                member_id
            );
            if tx.union_members.remove(&member_id).is_none() {
                return Err(DaoError::MemberNotFound(member_id));
            }
            Ok(())
        })
    }
    fn find_union_member<'a>(
        &self,
        member_id: MemberId,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = EmployeeId, Err = DaoError> {
        trace!("find_union_members called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            trace!(
                "find_union_members::with_tx called: member_id={}",
                member_id
            );
            tx.union_members
                .get(&member_id)
                .cloned()
                .ok_or(DaoError::MemberNotFound(member_id))
        })
    }
    fn record_paycheck<'a>(
        &self,
        emp_id: EmployeeId,
        pc: Paycheck,
    ) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
        trace!("record_paycheck called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            trace!(
                "record_paycheck::with_tx called: emp_id={},paycheck={:?}",
                emp_id,
                pc
            );
            tx.paychecks.entry(emp_id).or_insert_with(Vec::new).push(pc);
            Ok(())
        })
    }
}
