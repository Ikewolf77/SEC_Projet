use casbin::prelude::*;
use log::{error, info, trace, warn};

use super::UserDTO;

const CONFIG: &str = "accessControl/access_control.conf";
pub const POLICY: &str = "accessControl/access_control.csv";

pub const TEACHER_ACC: &str = "create_teacher_account";
pub const STUDENT_ACC: &str = "create_student_account";
pub const SHOW_GRADES: &str = "show_all_grades";
pub const ENTER_GRADE: &str = "enter_grade";

///Centralized access control mechanism
pub async fn auth(subject: &UserDTO, ressource: &str) -> bool {
    trace!("Auth");
    let e = Enforcer::new(CONFIG, POLICY)
        .await
        .expect("cannot read model or policy");
    if let Ok(authorized) = e.enforce((&subject.id, ressource)) {
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

/**
#[cfg(test)]
mod test_access_control {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        ressource,
        expected,
        case(UserDTO {id: "admin".to_owned(), email: "admin".to_owned()}, TEACHER_ACC, true),
        case(UserDTO {id: "admin".to_owned(), email: "admin".to_owned()}, STUDENT_ACC, true),
        case(UserDTO {id: "admin".to_owned(), email: "admin".to_owned()}, SHOW_GRADES, true),
        case(UserDTO {id: "admin".to_owned(), email: "admin".to_owned()}, ENTER_GRADE, true),
        case(UserDTO {id: "123".to_owned(), email: "123@123.ch".to_owned()}, TEACHER_ACC, false),
        case(UserDTO {id: "123".to_owned(), email: "123@123.ch".to_owned()}, STUDENT_ACC, false),
        case(UserDTO {id: "123".to_owned(), email: "123@123.ch".to_owned()}, SHOW_GRADES, false),
        case(UserDTO {id: "123".to_owned(), email: "123@123.ch".to_owned()}, ENTER_GRADE, false)
    )]
    pub fn access_control_test(input: UserDTO, ressource: &str, expected: bool) {
        assert_eq!(auth(&input, ressource), expected);
    }
}
**/
fn plop() {}
