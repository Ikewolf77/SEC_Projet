use sodiumoxide::crypto::pwhash::argon2id13;

//hash a password
pub fn hash(passwd: &str) -> String {
    sodiumoxide::init().unwrap();
    let hash = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();
    let texthash = std::str::from_utf8(&hash.0).unwrap().to_string();
    texthash
}

//compare an hash and a password
pub fn verify(hash: [u8; 128], passwd: &str) -> bool {
    sodiumoxide::init().unwrap();
    match argon2id13::HashedPassword::from_slice(&hash) {
        Some(hp) => argon2id13::pwhash_verify(&hp, passwd.as_bytes()),
        _ => false,
    }
}

//hash a password and transform the hash into bytes
pub fn padded_hash(pw: &str) -> [u8; 128] {
    let pw_hash = hash(pw);
    let mut padded = [0u8; 128];
    pw_hash.as_bytes().iter().enumerate().for_each(|(i, val)| {
        padded[i] = val.clone();
    });
    return padded;
}

#[cfg(test)]
mod test_student {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        case("myPassword"),
        case("AnotherOne"),
        case("r38zrw789FG/(wgF/(gae78FG08ae7F/aezF78aezf78'ze9PFHAJf"),
        case(""),
        ::trace
    )]
    pub fn test_hash(input: &str) {
        let hash = padded_hash(input);
        assert!(verify(hash, input));
    }
}
