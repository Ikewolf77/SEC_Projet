use casbin::prelude::*;
use log::{error, info, trace, warn};

use super::UserDTO;

const CONFIG: &str = "accessControl/access_control.conf";
const POLICY: &str = "accessControl/access_control.csv";

pub const TEACHER_ACC: &str = "create_teacher_account";
pub const STUDENT_ACC: &str = "create_account";
pub const SHOW_GRADES: &str = "show_all_grades";
pub const ENTER_GRADE: &str = "enter_grade";

///Centralized access control mechanism
pub async fn auth(subject: &UserDTO, ressource: &str) -> bool {
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
