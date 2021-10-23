use chrono::TimeZone;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct JWT {
    secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // who, e.g. "Shortcuts on iPhone"
    exp: usize,
}

pub struct TokenInfo {
    pub subject: String,
    pub until: chrono::DateTime<chrono::Utc>,
}

impl JWT {
    pub fn new(secret: String) -> Result<JWT, Box<dyn Error>> {
        Ok(JWT { secret })
    }

    pub fn generate(
        &self,
        subject: String,
        until: chrono::DateTime<chrono::Utc>,
    ) -> Result<String, Box<dyn Error>> {
        let token = encode(
            &Header::default(),
            &Claims {
                sub: subject,
                exp: until.timestamp() as usize,
            },
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        Ok(token)
    }

    pub fn is_valid(&self, token: String) -> Result<bool, Box<dyn Error>> {
        let token = self.get_info(token)?;
        Ok(chrono::Utc::now() < token.until)
    }

    pub fn get_info(&self, token: String) -> Result<TokenInfo, Box<dyn Error>> {
        let mut validation = Validation::default();
        validation.validate_exp = false;

        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        )?;

        Ok(TokenInfo {
            subject: token.claims.sub,
            until: chrono::Utc.timestamp(token.claims.exp as i64, 0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_and_validate() {
        struct TestCase {
            subject: String,
            until: String,
            expected_result: bool,
        }
        let tests: Vec<TestCase> = vec![
            TestCase {
                subject: String::from("test 1"),
                until: String::from("Fri, 14 Jul 2017 02:40:00 +0000"),
                expected_result: false,
            },
            TestCase {
                subject: String::from("test 2"),
                until: String::from("Wed, 14 Jul 2027 02:40:00 +0000"),
                expected_result: true,
            },
        ];

        let jwt = JWT::new(String::from("secret token!!1")).unwrap();
        for test in tests {
            let until = chrono::DateTime::parse_from_rfc2822(&test.until)
                .unwrap()
                .with_timezone(&chrono::Utc);

            let token = jwt.generate(test.subject, until).unwrap();
            assert_ne!(token, "");

            let result = jwt.is_valid(token).unwrap();
            assert_eq!(result, test.expected_result)
        }
    }
}
