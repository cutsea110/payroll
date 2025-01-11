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
        #[error("emp_id={0} save failed")]
        SaveFailed(EmpId),
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
    use thiserror::Error;

    use crate::dao::DaoError;

    #[derive(Debug, Clone, Error)]
    pub enum UsecaseError {
        #[error("add employee failed: {0}")]
        AddEmpFailed(DaoError),
        #[error("change employee name failed: {0}")]
        ChgEmpNameFailed(DaoError),
    }

    // ユースケースのトランザクションのインターフェース
    mod interface {
        use crate::domain::EmpId;
        use crate::tx::UsecaseError;

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Response {
            Void,
            EmpId(EmpId),
        }

        // トランザクションのインターフェース
        pub trait Transaction {
            fn execute(&self) -> Result<Response, UsecaseError>;
        }

        mod add_emp {
            use tx_rs::Tx;

            // dao にのみ依存 (domain は当然 ok)
            use crate::dao::{EmpDao, HaveEmpDao};
            use crate::domain::{Emp, EmpId};
            use crate::tx::UsecaseError;

            // ユースケース: AddEmp トランザクション(抽象レベルのビジネスロジック)
            pub trait AddEmp: HaveEmpDao {
                fn get_id(&self) -> EmpId;
                fn get_name(&self) -> &str;
                fn execute<'a>(&self) -> Result<(), UsecaseError> {
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
            use crate::dao::{EmpDao, HaveEmpDao};
            use crate::domain::EmpId;
            use crate::tx::UsecaseError;

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
            // dao にのみ依存 (domain は当然 ok)
            use crate::dao::{EmpDao, HaveEmpDao};
            use crate::domain::EmpId;
            use crate::tx::{AddEmp, Response, Transaction, UsecaseError};

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
                fn execute(&self) -> Result<Response, UsecaseError> {
                    AddEmp::execute(self).map(|_| Response::EmpId(self.id))
                }
            }
        }
        pub use add_emp_tx::*;

        mod chg_name_tx {
            // dao にのみ依存 (domain は当然 ok)
            use crate::dao::{EmpDao, HaveEmpDao};
            use crate::domain::EmpId;
            pub use crate::tx::{ChgEmpName, Response, Transaction, UsecaseError};

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
                fn execute(&self) -> Result<Response, UsecaseError> {
                    ChgEmpName::execute(self).map(|_| Response::Void)
                }
            }
        }
        pub use chg_name_tx::*;
    }
    pub use tx_impl::*;
}

mod tx_factory {
    // tx のインターフェースにのみ依存 (domain は当然 ok)
    // TODO: tx::Transaction は tx-app モジュールを作ってそちらに移動する
    use crate::domain::EmpId;
    use crate::tx::Transaction;

    pub trait TxFactory {
        fn mk_add_emp_tx(&self, id: EmpId, name: &str) -> Box<dyn Transaction>;
        fn mk_chg_emp_name_tx(&self, id: EmpId, new_name: &str) -> Box<dyn Transaction>;
    }

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
                tx.insert(emp_id, emp)
                    .map(|_| ())
                    .ok_or(DaoError::SaveFailed(emp_id))
            })
        }
    }
}

fn main() {
    // main hs_db と tx と tx-factory に依存
    use crate::hs_db::HashDB;
    use crate::tx::{AddEmpTx, ChgEmpNameTx};
    use crate::tx_factory::TxFactory;

    let db = HashDB::new();
    let tx_factory = tx_factory::TxFactoryImpl {
        add_emp: &|id, name| Box::new(AddEmpTx::new(id, name, db.clone())),
        chg_emp_name: &|id, new_name| Box::new(ChgEmpNameTx::new(id, new_name, db.clone())),
    };

    // ここで main が HashDB に依存しているだけで AddEmpTx/ChgEmpNameTx は具体的な DB 実装(HashDB)に依存していない
    let emp_dao = tx_factory.mk_add_emp_tx(1, "Alice");
    let _ = emp_dao.execute();
    println!("db: {:#?}", db);

    let emp_dao = tx_factory.mk_add_emp_tx(2, "Bob");
    let _ = emp_dao.execute();
    println!("db: {:#?}", db);

    let emp_dao = tx_factory.mk_chg_emp_name_tx(2, "Eve");
    let _ = emp_dao.execute();
    println!("db: {:#?}", db);
}
