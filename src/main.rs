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
                    println!(
                        "AddEmp::execute for id={}, name={}",
                        self.get_id(),
                        self.get_name()
                    );
                    self.dao()
                        .run_tx(|mut ctx| {
                            let emp = Emp::new(self.get_id(), self.get_name());
                            self.dao().save(emp).run(&mut ctx)
                        })
                        .map_err(UsecaseError::AddEmpFailed)
                }
            }
        }
        pub use add_emp::*;

        mod chg_name {
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
                    self.dao()
                        .run_tx(|mut ctx| {
                            let mut emp = self.dao().get(self.get_id()).run(&mut ctx)?;
                            emp.set_name(self.get_new_name());
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
                    AddEmp::execute(self)
                        .map(|_| Response::EmpId(self.id))
                        .map_err(|e| e.into())
                }
            }
        }
        pub use add_emp_tx::*;

        mod chg_name_tx {
            use anyhow;

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
                    ChgEmpName::execute(self)
                        .map(|_| Response::Void)
                        .map_err(|e| e.into())
                }
            }
        }
        pub use chg_name_tx::*;
    }
    pub use tx_impl::*;
}

mod tx_app {
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
        // なににも依存しない (domain は当然 ok)
        use crate::domain::EmpId;
        use crate::tx_app::{Transaction, Tx};

        pub trait TxFactory {
            fn convert(&self, src: Tx) -> Box<dyn Transaction> {
                match src {
                    Tx::AddEmp(id, name) => self.mk_add_emp_tx(id, &name),
                    Tx::ChgEmpName(id, new_name) => self.mk_chg_emp_name_tx(id, &new_name),
                }
            }

            fn mk_add_emp_tx(&self, id: EmpId, name: &str) -> Box<dyn Transaction>;
            fn mk_chg_emp_name_tx(&self, id: EmpId, new_name: &str) -> Box<dyn Transaction>;
        }
    }
    pub use tx_factory::*;

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
            while let Some(cmd) = self.tx_source.get_tx_source() {
                let tx = self.tx_factory.convert(cmd);
                tx.execute()?;
            }
            Ok(())
        }
    }
}

mod tx_factory {
    // tx_app にのみ依存 (domain は当然 ok)
    use crate::domain::EmpId;
    use crate::tx_app::{Transaction, TxFactory};

    pub struct TxFactoryImpl<'a> {
        pub add_emp: &'a dyn Fn(EmpId, &str) -> Box<dyn Transaction>,
        pub chg_emp_name: &'a dyn Fn(EmpId, &str) -> Box<dyn Transaction>,
    }
    impl<'a> TxFactory for TxFactoryImpl<'a> {
        fn mk_add_emp_tx(&self, id: EmpId, name: &str) -> Box<dyn Transaction> {
            (self.add_emp)(id, name)
        }
        fn mk_chg_emp_name_tx(&self, id: EmpId, new_name: &str) -> Box<dyn Transaction> {
            (self.chg_emp_name)(id, new_name)
        }
    }
}

mod text_parser_tx_source {
    use std::{cell::RefCell, collections::VecDeque, rc::Rc};

    // tx_app にのみ依存
    use crate::tx_app::{Tx, TxSource};

    pub struct TextParserTxSource {
        txs: Rc<RefCell<VecDeque<Tx>>>,
    }
    impl TextParserTxSource {
        pub fn new(_input: &str) -> Self {
            Self {
                // 今はテスト用の実装になっている
                txs: Rc::new(RefCell::new(VecDeque::from(vec![
                    Tx::AddEmp(1, "Alice".to_string()),
                    Tx::AddEmp(2, "Bob".to_string()),
                    Tx::ChgEmpName(2, "Eve".to_string()),
                ]))),
            }
        }
    }

    impl TxSource for TextParserTxSource {
        fn get_tx_source(&self) -> Option<Tx> {
            self.txs.borrow_mut().pop_front()
        }
    }
}

// 具体的な DB 実装
mod hs_db {
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
            f(self.emps.borrow_mut())
        }

        fn get<'a>(&self, id: EmpId) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = Emp, Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
                tx.get(&id).cloned().ok_or(DaoError::NotFound(id))
            })
        }
        fn save<'a>(&self, emp: Emp) -> impl tx_rs::Tx<Self::Ctx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut Self::Ctx<'a>| {
                let emp_id = emp.id();
                tx.insert(emp_id, emp);
                Ok(())
            })
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    use crate::hs_db::HashDB;
    use crate::text_parser_tx_source::TextParserTxSource;
    use crate::tx::{AddEmpTx, ChgEmpNameTx};
    use crate::tx_app::TxApp;
    use crate::tx_factory::TxFactoryImpl;

    let db = HashDB::new();

    let tx_factory = TxFactoryImpl {
        add_emp: &|id, name| Box::new(AddEmpTx::new(id, name, db.clone())),
        chg_emp_name: &|id, new_name| Box::new(ChgEmpNameTx::new(id, new_name, db.clone())),
    };
    let tx_source = TextParserTxSource::new("no input yet");
    let tx_app = TxApp::new(tx_source, tx_factory);

    tx_app.run()?;
    println!("{:#?}", db);

    Ok(())
}
