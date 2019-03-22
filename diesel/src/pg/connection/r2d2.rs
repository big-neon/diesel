
use diesel::r2d2::{ConnectionManager, Error};
use r2d2::ManageConnection;
use pg::PgConnection;


impl ManageConnection for ConnectionManager<PgConnection> {
    type Connection = T;
    type Error = LocalR2D2Error;

    fn connect(&self) -> Result<T, Self::Error> {
        T::establish(&self.database_url).map_err(Error::ConnectionError)
    }

    fn is_valid(&self, conn: &mut T) -> Result<(), Self::Error> {
        let tx_id = conn.execute("SELECT txid_current()")
            //.map(|_| ())
            .map_err(Error::QueryError)?;
        let tx_id2 = conn.execute("SELECT txid_current()")
            //.map(|_| ())
            .map_err(Error::QueryError)?;
        // If the transaction ids are the same, then there is
        // a transaction that has not been rolled back and it is
        // not safe to use as a new connection. It would be better to
        // do this call in `has_broken` because `is_valid` is only called when
        // a new transaction is requested, but `has_broken` is meant to be a quick
        // check
        if tx_id == tx_id2 {
            return Err(Error::InvalidConnectionError(
                "Connection still has an uncommitted transaction and should not be reused".to_string()))
        }
    }

    fn has_broken(&self, _conn: &mut T) -> bool {
        false
    }
}