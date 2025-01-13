// dao の具体的な実装
mod hs_db {
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
}

fn main() -> Result<(), anyhow::Error> {
    use log::info;
    use std::fs;

    use crate::hs_db::HashDB;
    use text_parser_tx_source::TextParserTxSource;
    use tx_app::TxApp;
    use tx_factory::TxFactoryImpl;
    use tx_impl::{AddEmpTx, ChgEmpNameTx};

    info!("TxApp starting");
    env_logger::init();

    let db = HashDB::new();
    info!("DB initialized: {:?}", db);

    // テストスクリプトを読み込んでシナリオを実行
    let input = fs::read_to_string("script/test.scr")?;
    let tx_source = TextParserTxSource::new(&input);
    let tx_factory = TxFactoryImpl {
        add_emp: &|id, name| Box::new(AddEmpTx::new(id, name, db.clone())),
        chg_emp_name: &|id, new_name| Box::new(ChgEmpNameTx::new(id, new_name, db.clone())),
    };
    let tx_app = TxApp::new(tx_source, tx_factory);

    info!("TxApp starting");
    tx_app.run()?;
    info!("TxApp finished");

    println!("{:#?}", db);
    info!("TxApp finished");

    Ok(())
}
