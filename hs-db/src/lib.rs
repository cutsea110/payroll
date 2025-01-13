// dao の具体的な実装
use log::trace;
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

// dao にのみ依存 (domain は当然 ok)
use dao::{DaoError, EmpDao};
use payroll_domain::{Emp, EmpId, MemberId, Paycheck};

// DB の実装 HashDB は EmpDao にのみ依存する かつ HashDB に依存するものはなにもない!! (main 以外には!)
#[derive(Debug, Clone)]
pub struct HashDB {
    // HashDB を DBMS として EmpDb がデータベースを表現
    emp_db: Rc<RefCell<EmpDb>>,
}
impl HashDB {
    pub fn new() -> Self {
        let emp_db = EmpDb {
            employees: HashMap::new(),
            union_members: HashMap::new(),
            paychecks: HashMap::new(),
        };
        Self {
            emp_db: Rc::new(RefCell::new(emp_db)),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EmpDb {
    employees: HashMap<EmpId, Emp>,
    union_members: HashMap<MemberId, EmpId>,
    paychecks: HashMap<EmpId, Vec<Paycheck>>,
}
// DB の実装ごとに EmpDao トレイトを実装する
impl EmpDao for HashDB {
    type Ctx<'a> = RefMut<'a, EmpDb>;

    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, DaoError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>,
    {
        trace!("HashDB::run_tx called");
        // RefCell の borrow_mut が RDB におけるトランザクションに相当
        f(self.emp_db.borrow_mut())
    }

    fn insert<'a>(&self, emp: Emp) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = EmpId, Err = DaoError> {
        trace!("HashDB::insert called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            let emp_id = emp.id();
            trace!(
                "HashDB::insert::with_tx called: emp_id={},emp={:?}",
                emp_id,
                emp
            );
            if tx.employees.contains_key(&emp_id) {
                return Err(DaoError::AlreadyExists(emp_id));
            }
            tx.employees.insert(emp_id, emp);
            Ok(emp_id)
        })
    }
    fn remove<'a>(&self, id: EmpId) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
        trace!("HashDB::remove called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            trace!("HashDB::remove::with_tx called: id={}", id);
            if tx.employees.remove(&id).is_some() {
                return Ok(());
            }
            Err(DaoError::NotFound(id))
        })
    }
    fn fetch<'a>(&self, id: EmpId) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Emp, Err = DaoError> {
        trace!("HashDB::fetch called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            trace!("HashDB::fetch::with_tx called: id={}", id);
            tx.employees.get(&id).cloned().ok_or(DaoError::NotFound(id))
        })
    }
    fn update<'a>(&self, emp: Emp) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
        trace!("HashDB::save called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            let emp_id = emp.id();
            trace!(
                "HashDB::save::with_tx called: emp_id={},emp={:?}",
                emp_id,
                emp
            );
            if tx.employees.contains_key(&emp_id) {
                tx.employees.insert(emp_id, emp);
                return Ok(());
            }
            Err(DaoError::NotFound(emp_id))
        })
    }
}
