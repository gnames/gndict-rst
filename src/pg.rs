use crate::conf::Conf;
use postgres::{Client, NoTls, RowIter};
use std::error;

pub struct Db {
    client: Client,
}

impl Db {
    pub fn new(cnf: Conf) -> Result<Self, Box<dyn error::Error>> {
        let url = format!(
            "postgresql://{}:{}@{}:5432/{}",
            cnf.pg_user, cnf.pg_pass, cnf.pg_host, cnf.pg_db
        );
        let client = Client::connect(&url, NoTls)?;
        Ok(Db { client })
    }

    pub fn get_data(&mut self, query: &str) -> Result<RowIter<'_>, Box<dyn error::Error>> {
        let res = self.client.query_raw(query, std::iter::empty())?;
        Ok(res)
    }
}
