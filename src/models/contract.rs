use crate::schema::contract;
use crate::utils::now;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Contract {
    pub id: i32,
    pub name: String,
    pub owner: String,
}

#[derive(Insertable)]
#[table_name = "contract"]
pub struct NewContract {
    pub name: String,
    pub owner: String,
}

impl NewContract {
    pub fn new(name: String, owner: String) -> Self {
        NewContract { name, owner }
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
}
