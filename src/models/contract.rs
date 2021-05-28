use crate::schema::contract;
use crate::utils::now;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use std::error::Error;
use std::time;

#[derive(Queryable, Debug)]
pub struct Contract {
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub address: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name = "contract"]
pub struct NewContract {
    pub name: String,
    pub owner: String,
    pub address: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
}

impl NewContract {
    pub fn new(name: String, owner: String, address: String) -> Self {
        NewContract {
            name,
            owner,
            address,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            active: true,
        }
    }

    pub fn insert(&self, conn: &PgConnection) -> Contract {
        create(conn, self)
    }
}

pub fn create(conn: &PgConnection, new_contract: &NewContract) -> Contract {
    diesel::insert_into(contract::table)
        .values(new_contract)
        .get_result(conn)
        .expect("Error saving new contract")
}

pub fn read(conn: &PgConnection) -> Vec<Contract> {
    contract::table
        .load::<Contract>(conn)
        .expect("Error loading contract")
}

pub fn read_by_name(conn: &PgConnection, name: String) -> Vec<Contract> {
    contract::table
        .filter(contract::name.eq(name))
        .load::<Contract>(conn)
        .expect("Error loading contract")
}

pub fn read_by_address(conn: &PgConnection, address: String) -> Vec<Contract> {
    contract::table
        .filter(contract::address.eq(address))
        .load::<Contract>(conn)
        .expect("Error loading contract")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection_test;

    fn cleanup_table(conn: &PgConnection) {
        diesel::delete(contract::table).execute(conn).unwrap();
    }

    fn mock_struct() -> NewContract {
        NewContract::new(
            String::from("Token"),
            String::from("00a329c0648769a73afac7f9381e08fb43dbea72"),
            String::from("00a329c0648769a73afac7f9381e08fb43dbea76"),
        )
    }

    #[test]
    fn it_creates_a_contract() {
        let conn = establish_connection_test();

        create(&conn, &mock_struct());

        let contract = contract::table
            .load::<Contract>(&conn)
            .expect("Error loading contract");

        assert_eq!(1, contract.len());

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_contract() {
        let conn = establish_connection_test();

        let new_contract = mock_struct();

        let created_contract = diesel::insert_into(contract::table)
            .values(&new_contract)
            .get_result::<Contract>(&conn)
            .expect("Error saving new contract");

        let contract = read(&conn);

        assert!(0 < contract.len());

        let my_contract = contract.iter().find(|&x| x.name == new_contract.name);
        assert!(
            my_contract.is_some(),
            "Could not find the created contract in the database!"
        );

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_contract_by_name() {
        let conn = establish_connection_test();
        let name = String::from("NamedContract");

        // make 2 contracts, each with different categories
        let mut new_contract = mock_struct();
        create(&conn, &new_contract);

        new_contract.name = name.clone();
        create(&conn, &new_contract);

        let contract = read_by_name(&conn, name.clone());

        assert_eq!(1, contract.len());

        let my_contract = contract.iter().find(|x| x.name == name);
        assert!(
            my_contract.is_some(),
            "Could not find the created contract in the database!"
        );

        cleanup_table(&conn);
    }

    #[test]
    fn it_reads_a_contract_by_address() {
        let conn = establish_connection_test();
        let address =
            String::from("0cd1136c6702de4410d06d3ae80f592c9b2132ea232011bcc78fb53862cbd9ee");

        // make 2 contracts, each with different categories
        let mut new_contract = mock_struct();
        create(&conn, &new_contract);

        new_contract.address = address.clone();
        create(&conn, &new_contract);

        let contract = read_by_address(&conn, address.clone());

        assert_eq!(1, contract.len());

        let my_contract = contract.iter().find(|x| x.address == address);
        assert!(
            my_contract.is_some(),
            "Could not find the created contract in the database!"
        );

        cleanup_table(&conn);
    }
}
