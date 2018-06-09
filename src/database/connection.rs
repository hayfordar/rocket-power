use rocket::http::Status;
use rocket::request::{ FromRequest, self };
use rocket::{ Request, State, Outcome };

use database::MySQLPooledConnection;
use store::Store;

// Wrap pooled connection so we can implement FromRequest
pub struct Connection {
    pub MySQLPooledConnection;
}

impl<'a,'r> FromRequest<'a,'r> for Connection {
    type Error = ();

    fn from_request(request : &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Request guard on State for pulling connection from pool
        let state = request.guard::<State<Store>>()?;

        match state.sampleDbConn.get() {
            Ok(connection) => Outcome::Success(Connection(connection)),
            Err(_) => Outcome::Failure(Status::ServiceUnavailable, Self::Error)
        }
    }
}

impl Deref for Connection {
    type Target = MySQLPooledConnection;

    // Return wrapped connection value on deref
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}