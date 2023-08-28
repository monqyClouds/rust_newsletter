use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{s} is not a valid email."))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claim::{assert_err, assert_ok};

    #[test]
    fn empty_email_is_rejected() {
        let email = String::from("");
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = String::from("davincidomain.com");
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = String::from("@domain.com");
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn valid_email_is_parsed_successfully() {
        let email = String::from("davinci@domain.com");
        assert_ok!(SubscriberEmail::parse(email));
    }
}
