// TxFacotry の具体的な実装
mod tx_factory {
    use log::trace;

    // tx_app にのみ依存 (domain は当然 ok)
    use payroll_domain::EmpId;
    use tx_app::{Transaction, TxFactory};

    pub struct TxFactoryImpl<'a> {
        pub add_emp: &'a dyn Fn(EmpId, &str) -> Box<dyn Transaction>,
        pub chg_emp_name: &'a dyn Fn(EmpId, &str) -> Box<dyn Transaction>,
    }
    impl<'a> TxFactory for TxFactoryImpl<'a> {
        fn mk_add_emp_tx(&self, id: EmpId, name: &str) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_add_emp_tx called");
            (self.add_emp)(id, name)
        }
        fn mk_chg_emp_name_tx(&self, id: EmpId, new_name: &str) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_chg_emp_name_tx called");
            (self.chg_emp_name)(id, new_name)
        }
    }
}

// TxSource の具体的な実装
mod text_parser_tx_source {
    use log::{debug, trace};
    use std::{cell::RefCell, collections::VecDeque, rc::Rc};

    // tx_app にのみ依存
    use tx_app::{Tx, TxSource};

    pub struct TextParserTxSource {
        txs: Rc<RefCell<VecDeque<Tx>>>,
    }
    impl TextParserTxSource {
        pub fn new(input: &str) -> Self {
            Self {
                txs: Rc::new(RefCell::new(read_txs(input))),
            }
        }
    }

    impl TxSource for TextParserTxSource {
        fn get_tx_source(&self) -> Option<Tx> {
            trace!("TextParserTxSource::get_tx_source called");
            let tx = self.txs.borrow_mut().pop_front();
            debug!("tx_src={:?}", tx);
            tx
        }
    }

    mod parser {
        use log::{debug, trace};

        use parsec_rs::{char, int32, keyword, pred, spaces, string, Parser};
        use std::collections::VecDeque;

        use tx_app::Tx;

        pub fn read_txs(input: &str) -> VecDeque<Tx> {
            trace!("read_txs called");
            let txs = txs().parse(input).map(|p| p.0.into()).unwrap_or_default();
            debug!("txs={:?}", txs);
            txs
        }

        fn txs() -> impl Parser<Item = Vec<Tx>> {
            trace!("txs called");
            tx().many0()
        }

        fn tx() -> impl Parser<Item = Tx> {
            trace!("tx called");
            go_through().skip(add_emp().or(chg_emp_name()))
        }

        fn go_through() -> impl Parser<Item = ()> {
            let comment = char('#').skip(pred(|c| c != '\n').many0().with(char('\n')));
            let space_comment = spaces().skip(comment).map(|_| ());
            let ignore = space_comment.many1().map(|_| ()).or(spaces().map(|_| ()));

            spaces().skip(ignore).skip(spaces()).map(|_| ())
        }

        fn add_emp() -> impl Parser<Item = Tx> {
            let prefix = keyword("AddEmp").skip(spaces());
            let emp_id = int32().with(spaces());
            let name = string().with(spaces());

            prefix
                .skip(emp_id)
                .join(name)
                .map(|(id, name)| Tx::AddEmp(id, name))
        }

        fn chg_emp_name() -> impl Parser<Item = Tx> {
            let prefix = keyword("ChgEmp").skip(spaces());
            let emp_id = int32().with(spaces());
            let name = keyword("Name").skip(spaces()).skip(string());

            prefix
                .skip(emp_id)
                .join(name)
                .map(|(id, name)| Tx::ChgEmpName(id, name))
        }
    }
    pub use parser::*;
}

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
    use crate::text_parser_tx_source::TextParserTxSource;
    use crate::tx_factory::TxFactoryImpl;
    use tx_app::TxApp;
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
