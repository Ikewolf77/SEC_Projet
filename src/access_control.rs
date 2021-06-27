use casbin::prelude::*;
use log::{error, info, trace, warn};

extern crate strum;

use super::db::User;

const CONFIG: &str = "accessControl/access_control.conf";
const POLICY: &str = "accessControl/access_control.csv";

#[derive(Display, Debug)]
enum Role {
    #[strum(serialize = "admin")]
    Admin,
    #[strum(serialize = "teacher")]
    Teacher,
    Student,
}

///Centralized access control mechanism
pub async fn auth(subject: &User, ressource: &str) -> bool {
    trace!("Auth");
    let e = Enforcer::new(CONFIG, POLICY)
        .await
        .expect("cannot read model or policy");
    if let Ok(authorized) = e.enforce((&subject.email, ressource)) {
        if authorized {
            info!(
                "Authorisation success - Subject : {}, Ressource: {}",
                subject.email, ressource
            );
        } else {
            warn!(
                "Authorisation failure - Subject : {}, Ressource: {}",
                subject.email, ressource
            );
        }
        authorized
    } else {
        error!("Casbin model does not map request");
        panic!("Casbin model does not map request");
    }
}
