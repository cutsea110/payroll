mod domain {
    // なににも依存なし!

    #[derive(Debug, Clone)]
    pub struct Emp {
        id: i32,
        name: String,
    }

    impl Emp {
        pub fn new(id: i32, name: &str) -> Self {
            Self {
                id,
                name: name.to_string(),
            }
        }
        pub fn id(&self) -> i32 {
            self.id
        }
        pub fn set_name(&mut self, name: &str) {
            self.name = name.to_string();
        }
    }
}

mod dao {
    // domain にのみ依存
    use crate::domain::Emp;

    // Dao のインターフェース (TxAddEmpImp にはこちらにだけ依存させる)
    pub trait EmpDao {
        fn get(&self, id: i32) -> Option<Emp>;
        fn save(&self, emp: Emp);
    }

    pub trait HaveEmpDao {
        fn dao(&self) -> &impl EmpDao;
    }
}

mod usecase {
    mod add_emp {
        // dao にのみ依存 (domain は当然 ok)
        use crate::dao::{EmpDao, HaveEmpDao};
        use crate::domain::Emp;

        // ユースケース: AddEmp トランザクション(抽象レベルのビジネスロジック)
        pub trait TxAddEmpAbst: HaveEmpDao {
            fn get_id(&self) -> i32;
            fn get_name(&self) -> &str;
            fn execute(&self) {
                self.dao().save(Emp::new(self.get_id(), self.get_name()));
            }
        }

        // ユースケース: AddEmp トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct TxAddEmpImpl<T>
        where
            T: EmpDao,
        {
            id: i32,
            name: String,
            db: T,
        }
        impl<T> TxAddEmpImpl<T>
        where
            T: EmpDao,
        {
            pub fn new(id: i32, name: &str, dao: T) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for TxAddEmpImpl<T>
        where
            T: EmpDao,
        {
            fn dao(&self) -> &impl EmpDao {
                &self.db
            }
        }
        impl<T> TxAddEmpAbst for TxAddEmpImpl<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> i32 {
                self.id
            }
            fn get_name(&self) -> &str {
                &self.name
            }
        }
    }
    pub use add_emp::*;

    mod chg_name {
        // dao にのみ依存 (domain は当然 ok)
        use crate::dao::{EmpDao, HaveEmpDao};

        // ユースケース: AddEmp トランザクション(抽象レベルのビジネスロジック)
        pub trait TxChgEmpNameAbst: HaveEmpDao {
            fn get_id(&self) -> i32;
            fn get_new_name(&self) -> &str;
            fn execute(&self) {
                let mut emp = self.dao().get(self.get_id()).unwrap();
                emp.set_name(self.get_new_name());
                self.dao().save(emp);
            }
        }

        // ユースケース: AddEmp トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct TxChgEmpNameImpl<T>
        where
            T: EmpDao,
        {
            id: i32,
            new_name: String,
            db: T,
        }
        impl<T> TxChgEmpNameImpl<T>
        where
            T: EmpDao,
        {
            pub fn new(id: i32, new_name: &str, dao: T) -> Self {
                Self {
                    id,
                    new_name: new_name.to_string(),
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for TxChgEmpNameImpl<T>
        where
            T: EmpDao,
        {
            fn dao(&self) -> &impl EmpDao {
                &self.db
            }
        }
        impl<T> TxChgEmpNameAbst for TxChgEmpNameImpl<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> i32 {
                self.id
            }
            fn get_new_name(&self) -> &str {
                &self.new_name
            }
        }
    }
    pub use chg_name::*;
}

// 具体的な DB 実装
mod hash_db {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    // dao にのみ依存 (domain は当然 ok)
    use crate::dao::EmpDao;
    use crate::domain::Emp;

    // DB の実装 DbImpl は EmpDao にのみ依存する かつ DbImpl に依存するものはなにもない!! (main 以外には!)
    #[derive(Debug, Clone)]
    pub struct DbImpl {
        emps: Rc<RefCell<HashMap<i32, Emp>>>,
    }
    impl DbImpl {
        pub fn new() -> Self {
            Self {
                emps: Rc::new(RefCell::new(HashMap::new())),
            }
        }
    }
    // DB の実装ごとに EmpDao トレイトを実装する
    impl EmpDao for DbImpl {
        fn get(&self, id: i32) -> Option<Emp> {
            self.emps.borrow().get(&id).cloned()
        }
        fn save(&self, emp: Emp) {
            self.emps.borrow_mut().insert(emp.id(), emp);
        }
    }
}

fn main() {
    use crate::hash_db::DbImpl;
    use crate::usecase::{TxAddEmpAbst, TxAddEmpImpl, TxChgEmpNameAbst, TxChgEmpNameImpl};

    let db = DbImpl::new();
    // ここで main が DbImpl に依存しているだけで TxAddEmpImpl は DbImpl に依存していない
    let emp_dao = TxAddEmpImpl::new(1, "Alice", db.clone());
    emp_dao.execute();
    println!("db: {:#?}", emp_dao);

    let emp_dao = TxChgEmpNameImpl::new(1, "Bob", db);
    emp_dao.execute();
    println!("db: {:#?}", emp_dao);
}
