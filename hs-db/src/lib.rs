// dao の具体的な実装
use log::trace;
use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

// dao にのみ依存 (domain は当然 ok)
use dao::{DaoError, EmpDao};
use payroll_domain::{Emp, EmpId};

// DB の実装 HashDB は EmpDao にのみ依存する かつ HashDB に依存するものはなにもない!! (main 以外には!)
#[derive(Debug, Clone)]
pub struct HashDB {
    emps: Rc<RefCell<HashMap<EmpId, Emp>>>,
}
impl HashDB {
    pub fn new() -> Self {
        Self {
            emps: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}
// DB の実装ごとに EmpDao トレイトを実装する
impl EmpDao for HashDB {
    type Ctx<'a> = RefMut<'a, HashMap<EmpId, Emp>>;

    fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, DaoError>
    where
        F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>,
    {
        trace!("HashDB::run_tx called");
        // RefCell の borrow_mut が RDB におけるトランザクションに相当
        f(self.emps.borrow_mut())
    }

    fn get<'a>(&self, id: EmpId) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Emp, Err = DaoError> {
        trace!("HashDB::get called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            trace!("HashDB::get::with_tx called: id={}", id);
            tx.get(&id).cloned().ok_or(DaoError::NotFound(id))
        })
    }
    fn save<'a>(&self, emp: Emp) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
        trace!("HashDB::save called");
        tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
            let emp_id = emp.id();
            trace!(
                "HashDB::save::with_tx called: emp_id={},emp={:?}",
                emp_id,
                emp
            );
            tx.insert(emp_id, emp);
            Ok(())
        })
    }
}
