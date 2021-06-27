use lazy_static::lazy_static;
use read_input::prelude::*;
use regex::Regex;
use zxcvbn::zxcvbn;

lazy_static! {
    static ref EMAIL_RE: Regex =
        Regex::new(r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$").unwrap();
}

pub fn ask_for_email() -> String {
    input::<String>()
        .msg("Please enter your email address (also username):\n")
        .add_err_test(
            move |input| EMAIL_RE.is_match(&*input),
            "This does not look like a valid email, try again",
        )
        .get()
}

pub fn ask_for_pw() -> String {
    input::<String>()
        .repeat_msg("Please enter the password for this account")
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
