// TxSource の具体的な実装
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
