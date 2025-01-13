mod domain {
    // なににも依存なし!

    pub type EmpId = i32;

    #[derive(Debug, Clone)]
    pub struct Emp {
        id: EmpId,
        name: String,
    }

    impl Emp {
        pub fn new(id: EmpId, name: &str) -> Self {
            Self {
                id,
                name: name.to_string(),
            }
        }
        pub fn id(&self) -> EmpId {
            self.id
        }
        pub fn set_name(&mut self, name: &str) {
            self.name = name.to_string();
        }
    }
}

mod dao {
    use thiserror::Error;

    // domain にのみ依存
    use crate::domain::{Emp, EmpId};

    #[derive(Debug, Clone, Error)]
    pub enum DaoError {
        #[error("emp_id={0} not found")]
        NotFound(EmpId),
    }

    // Dao のインターフェース (AddEmpTx にはこちらにだけ依存させる)
    pub trait EmpDao {
        type Ctx<'a>;

        fn run_tx<'a, F, T>(&'a self, f: F) -> Result<T, DaoError>
        where
            F: FnOnce(Self::Ctx<'a>) -> Result<T, DaoError>;

        fn get<'a>(&self, id: EmpId) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Emp, Err = DaoError>;
        fn save<'a>(&self, emp: Emp) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError>;
    }

    pub trait HaveEmpDao {
        type Ctx<'a>;

        fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>>;
    }
}

mod tx {
    // ユースケースのトランザクションのインターフェース
    mod interface {
        use thiserror::Error;

        // dao にのみ依存
        use crate::dao::DaoError;

        #[derive(Debug, Clone, Error)]
        pub enum UsecaseError {
            #[error("add employee failed: {0}")]
            AddEmpFailed(DaoError),
            #[error("change employee name failed: {0}")]
            ChgEmpNameFailed(DaoError),
        }

        mod add_emp {
            use log::{debug, trace};
            use tx_rs::Tx;

            // dao にのみ依存 (domain は当然 ok)
            use super::UsecaseError;
            use crate::dao::{EmpDao, HaveEmpDao};
            use crate::domain::{Emp, EmpId};

            // ユースケース: AddEmp トランザクション(抽象レベルのビジネスロジック)
            pub trait AddEmp: HaveEmpDao {
                fn get_id(&self) -> EmpId;
                fn get_name(&self) -> &str;
                fn execute<'a>(&self) -> Result<(), UsecaseError> {
                    trace!("AddEmp::execute called");
                    self.dao()
                        .run_tx(|mut ctx| {
                            trace!("AddEmp::run_tx called");
                            let emp = Emp::new(self.get_id(), self.get_name());
                            debug!("AddEmp::execute: emp={:?}", emp);
                            self.dao().save(emp).run(&mut ctx)
                        })
                        .map_err(UsecaseError::AddEmpFailed)
                }
            }
        }
        pub use add_emp::*;

        mod chg_name {
            use log::{debug, trace};
            use tx_rs::Tx;

            // dao にのみ依存 (domain は当然 ok)
            use super::UsecaseError;
            use crate::dao::{EmpDao, HaveEmpDao};
            use crate::domain::EmpId;

            // ユースケース: ChgEmpName トランザクション(抽象レベルのビジネスロジック)
            pub trait ChgEmpName: HaveEmpDao {
                fn get_id(&self) -> EmpId;
                fn get_new_name(&self) -> &str;
                fn execute<'a>(&self) -> Result<(), UsecaseError> {
                    trace!("ChgEmpName::execute called");
                    self.dao()
                        .run_tx(|mut ctx| {
                            trace!("ChgEmpName::run_tx called");
                            let mut emp = self.dao().get(self.get_id()).run(&mut ctx)?;
                            debug!("changing emp name: emp={:?}", emp);
                            emp.set_name(self.get_new_name());
                            debug!("changed emp name: emp={:?}", emp);
                            self.dao().save(emp).run(&mut ctx)
                        })
                        .map_err(UsecaseError::ChgEmpNameFailed)
                }
            }
        }
        pub use chg_name::*;
    }
    pub use interface::*;

    // ユースケースのトランザクションの実装
    mod tx_impl {
        mod add_emp_tx {
            use anyhow;
            use log::trace;

            // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
            use super::super::AddEmp;
            use crate::dao::{EmpDao, HaveEmpDao};
            use crate::domain::EmpId;
            use crate::tx_app::{Response, Transaction};

            // ユースケース: AddEmp トランザクションの実装 (struct)
            #[derive(Debug)]
            pub struct AddEmpTx<T>
            where
                T: EmpDao,
            {
                id: EmpId,
                name: String,
                db: T,
            }
            impl<T> AddEmpTx<T>
            where
                T: EmpDao,
            {
                pub fn new(id: EmpId, name: &str, dao: T) -> Self {
                    Self {
                        id,
                        name: name.to_string(),
                        db: dao,
                    }
                }
            }

            impl<T> HaveEmpDao for AddEmpTx<T>
            where
                T: EmpDao,
            {
                type Ctx<'a> = T::Ctx<'a>;

                fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                    &self.db
                }
            }
            impl<T> AddEmp for AddEmpTx<T>
            where
                T: EmpDao,
            {
                fn get_id(&self) -> EmpId {
                    self.id
                }
                fn get_name(&self) -> &str {
                    &self.name
                }
            }
            // 共通インターフェースの実装
            impl<T> Transaction for AddEmpTx<T>
            where
                T: EmpDao,
            {
                fn execute(&self) -> Result<Response, anyhow::Error> {
                    trace!("AddEmpTx::execute called");
                    AddEmp::execute(self)
                        .map(|_| Response::EmpId(self.id))
                        .map_err(Into::into)
                }
            }
        }
        pub use add_emp_tx::*;

        mod chg_name_tx {
            use anyhow;
            use log::trace;

            // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
            use super::super::ChgEmpName;
            use crate::dao::{EmpDao, HaveEmpDao};
            use crate::domain::EmpId;
            use crate::tx_app::{Response, Transaction};

            // ユースケース: ChgEmpName トランザクションの実装 (struct)
            #[derive(Debug)]
            pub struct ChgEmpNameTx<T>
            where
                T: EmpDao,
            {
                id: EmpId,
                new_name: String,
                db: T,
            }
            impl<T> ChgEmpNameTx<T>
            where
                T: EmpDao,
            {
                pub fn new(id: EmpId, new_name: &str, dao: T) -> Self {
                    Self {
                        id,
                        new_name: new_name.to_string(),
                        db: dao,
                    }
                }
            }

            impl<T> HaveEmpDao for ChgEmpNameTx<T>
            where
                T: EmpDao,
            {
                type Ctx<'a> = T::Ctx<'a>;

                fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                    &self.db
                }
            }
            impl<T> ChgEmpName for ChgEmpNameTx<T>
            where
                T: EmpDao,
            {
                fn get_id(&self) -> EmpId {
                    self.id
                }
                fn get_new_name(&self) -> &str {
                    &self.new_name
                }
            }
            // 共通インターフェースの実装
            impl<T> Transaction for ChgEmpNameTx<T>
            where
                T: EmpDao,
            {
                fn execute(&self) -> Result<Response, anyhow::Error> {
                    trace!("ChgEmpNameTx::execute called");
                    ChgEmpName::execute(self)
                        .map(|_| Response::Void)
                        .map_err(Into::into)
                }
            }
        }
        pub use chg_name_tx::*;
    }
    pub use tx_impl::*;
}

mod tx_app {
    use log::trace;

    pub struct TxApp<S, F>
    where
        S: TxSource,
        F: TxFactory,
    {
        tx_source: S,
        tx_factory: F,
    }
    impl<S, F> TxApp<S, F>
    where
        S: TxSource,
        F: TxFactory,
    {
        pub fn new(tx_source: S, tx_factory: F) -> Self {
            Self {
                tx_source,
                tx_factory,
            }
        }
        pub fn run(&self) -> Result<(), anyhow::Error> {
            trace!("TxApp::run called");
            while let Some(tx_src) = self.tx_source.get_tx_source() {
                trace!("get tx_source={:?}", tx_src);
                let tx = self.tx_factory.convert(tx_src);
                tx.execute()?;
            }
            Ok(())
        }
    }

    mod tx {
        use anyhow;
        // なににも依存しない (domain は当然 ok)
        use crate::domain::EmpId;

        // トランザクションのインターフェース
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Response {
            Void,
            EmpId(EmpId),
        }
        pub trait Transaction {
            fn execute(&self) -> Result<Response, anyhow::Error>;
        }
    }
    pub use tx::*;

    mod tx_source {
        // なににも依存しない (domain は当然 ok)
        use crate::domain::EmpId;

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Tx {
            AddEmp(EmpId, String),
            ChgEmpName(EmpId, String),
        }
        pub trait TxSource {
            fn get_tx_source(&self) -> Option<Tx>;
        }
    }
    pub use tx_source::*;

    mod tx_factory {
        use log::trace;

        // なににも依存しない (domain は当然 ok)
        use super::{Transaction, Tx};
        use crate::domain::EmpId;

        pub trait TxFactory {
            fn convert(&self, src: Tx) -> Box<dyn Transaction> {
                trace!("TxFactory::convert called");
                match src {
                    Tx::AddEmp(id, name) => {
                        trace!("convert Tx::AddEmp by mk_add_emp_tx called");
                        self.mk_add_emp_tx(id, &name)
                    }
                    Tx::ChgEmpName(id, new_name) => {
                        trace!("convert Tx::ChgEmpName by mk_chg_emp_name_tx called");
                        self.mk_chg_emp_name_tx(id, &new_name)
                    }
                }
            }

            fn mk_add_emp_tx(&self, id: EmpId, name: &str) -> Box<dyn Transaction>;
            fn mk_chg_emp_name_tx(&self, id: EmpId, new_name: &str) -> Box<dyn Transaction>;
        }
    }
    pub use tx_factory::*;
}

// TxFacotry の具体的な実装
mod tx_factory {
    use log::trace;

    // tx_app にのみ依存 (domain は当然 ok)
    use crate::domain::EmpId;
    use crate::tx_app::{Transaction, TxFactory};

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
    use crate::tx_app::{Tx, TxSource};

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

        use crate::tx_app::Tx;

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
    use crate::dao::{DaoError, EmpDao};
    use crate::domain::{Emp, EmpId};

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
    use crate::tx::{AddEmpTx, ChgEmpNameTx};
    use crate::tx_app::TxApp;
    use crate::tx_factory::TxFactoryImpl;

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
