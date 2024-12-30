#[derive(Debug, Clone)]
struct Emp {
    name: String,
}

impl Emp {
    fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

trait ChgEmpTx {
    fn change(&self, emp: &mut Emp);
    fn execute(&self) -> Result<(), String> {
        let mut emp = Emp {
            name: "John".to_string(),
        };
        self.change(&mut emp);
        println!("emp: {:?}", emp);
        Ok(())
    }
}

trait ChgEmpNameTx {
    fn new_name(&self) -> &str;
    fn change(&self, emp: &mut Emp) {
        emp.set_name(self.new_name());
    }
}
// T が ChgEmpNameTx なら常に ChgEmpTx でもある (blanket impl)
impl<T> ChgEmpTx for T
where
    T: ChgEmpNameTx,
{
    fn change(&self, emp: &mut Emp) {
        ChgEmpNameTx::change(self, emp);
    }
}

// ChgEmpName を使って ChgEmpNameTx を実装
struct ChgEmpNameTxImpl {
    name: String,
}
impl ChgEmpNameTx for ChgEmpNameTxImpl {
    fn new_name(&self) -> &str {
        &self.name
    }
}
fn main() {
    let tx = ChgEmpNameTxImpl {
        name: "Doe".to_string(),
    };
    tx.execute().unwrap();
}
