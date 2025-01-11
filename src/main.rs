use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
struct Emp {
    id: i32,
    name: String,
}

impl Emp {
    fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

// Dao のインターフェース (TxAddEmpImp にはこちらにだけ依存させる)
trait EmpDao {
    fn get(&self, id: i32) -> Option<Emp>;
    fn save(&self, emp: Emp);
}

trait HaveEmpDao {
    fn dao(&self) -> impl EmpDao;
}

// ユースケース: AddEmp トランザクション(抽象レベルのビジネスロジック)
trait TxAddEmp: HaveEmpDao {
    fn add_emp(&self, emp: Emp) {
        self.dao().save(emp);
    }
    fn get_emp(&self, id: i32) -> Option<Emp> {
        self.dao().get(id)
    }
    fn set_name(&self, id: i32, new_name: &str) {
        let mut emp = self.dao().get(id).unwrap();
        emp.set_name(new_name);
        self.dao().save(emp);
    }
}

// DB の実装 DbImpl は EmpDao にのみ依存する かつ DbImpl に依存するものはなにもない!! (main 以外には!)
#[derive(Debug, Clone)]
struct DbImpl {
    emps: Rc<RefCell<HashMap<i32, Emp>>>,
}
// DB の実装ごとに EmpDao トレイトを実装する
impl EmpDao for DbImpl {
    fn get(&self, id: i32) -> Option<Emp> {
        self.emps.borrow().get(&id).cloned()
    }
    fn save(&self, emp: Emp) {
        self.emps.borrow_mut().insert(emp.id, emp);
    }
}

// AddEmp トランザクションの実装
#[derive(Debug)]
struct TxAddEmpImpl<T>
where
    T: EmpDao,
{
    db: T,
}
impl<T> HaveEmpDao for TxAddEmpImpl<T>
where
    T: EmpDao + Clone,
{
    fn dao(&self) -> impl EmpDao {
        self.db.clone()
    }
}
impl<T> TxAddEmp for TxAddEmpImpl<T> where T: EmpDao + Clone {}

fn main() {
    let db = DbImpl {
        emps: Rc::new(RefCell::new(HashMap::new())),
    };
    // ここで main が DbImpl に依存しているだけで TxAddEmpImpl は DbImpl に依存していない
    let emp_dao = TxAddEmpImpl { db };
    let emp = Emp {
        id: 1,
        name: "John".to_string(),
    };
    emp_dao.add_emp(emp.clone());
    println!("db: {:#?}", emp_dao);

    let emp = emp_dao.get_emp(1);
    println!("emp: {:#?}", emp);

    let emp = emp_dao.get_emp(2);
    println!("emp: {:#?}", emp);

    emp_dao.set_name(1, "Smith");
    println!("emp: {:#?}", emp_dao);
}
