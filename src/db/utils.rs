use lazy_static::lazy_static;
use read_input::prelude::*;
use regex::Regex;
use zxcvbn::zxcvbn;

lazy_static! {
    static ref EMAIL_RE: Regex =
        Regex::new(r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$").unwrap();
    static ref NAME_RE: Regex = Regex::new(r"^[a-zA-Zàâçéèêëîïôûùüÿñæœ .-]{2,20}$").unwrap();
}

pub fn ask_for_name() -> String {
    input::<String>()
        .msg("Please enter their name:\n")
        .add_err_test(
            move |input| NAME_RE.is_match(&*input),
            "This does not look like a valid name, try again",
        )
        .get()
}

pub fn ask_for_email(is_login: bool) -> String {
    input::<String>()
        .msg("Please enter an email address:\n")
        .add_err_test(
            move |input| (EMAIL_RE.is_match(&*input) || (input.eq("admin") && is_login)),
            "This does not look like a valid email, try again",
        )
        .get()
}

pub fn ask_for_pw(is_new: bool) -> String {
    let msg = if is_new {
        "Please enter your new password:\n"
    } else {
        "Please enter the password for this account:\n"
    };

    input::<String>()
        .repeat_msg(msg)
        .add_err_test(move |input| !(input.len() < 8), "\nPassword too short\n")
        .add_err_test(move |input| !(input.len() > 64), "\nPassword too long\n")
        .add_err_test(
            move |input| {
                let estimate = zxcvbn(input, &[]).unwrap();
                if estimate.score() <= 2 {
                    let feedback = estimate.feedback().as_ref().unwrap();

                    if let Some(warning) = feedback.warning() {
                        println!("\nWarning : {}", warning);
                    }
                    println!("\nSuggestions : ");
                    for suggestion in feedback.suggestions() {
                        println!("{}", suggestion);
                    }
                    return false;
                } else {
                    return true;
                }
            },
            "\nTry again\n",
        )
        .get()
}
