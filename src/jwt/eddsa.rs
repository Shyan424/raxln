use std::sync::{Arc, OnceLock};

use jsonwebtoken::{EncodingKey, DecodingKey, encode, decode, Validation};
use ring::signature::KeyPair;
use serde::{Serialize, Deserialize};
use time::{OffsetDateTime, Duration};

use super::error::AuthError;


pub struct EdDsaJwt {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation
}

impl EdDsaJwt {
    pub fn new() -> Self {
        let doc = ring::signature::Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
        let encoding_key = EncodingKey::from_ed_der(doc.as_ref());

        let pair = ring::signature::Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
        let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

        let validation = Validation::new(jsonwebtoken::Algorithm::EdDSA);

        EdDsaJwt { encoding_key, decoding_key, validation }
    }

    pub fn default() -> Arc<EdDsaJwt> {
        static EDDSA: OnceLock<Arc<EdDsaJwt>> = OnceLock::new();
        Arc::clone(&EDDSA.get_or_init(|| Arc::new(EdDsaJwt::new())))
    }

    pub fn signature(&self, id: &String) -> Result<String, AuthError> {
        let iat = OffsetDateTime::now_utc();
        let exp = iat.saturating_add(Duration::minutes(30));
        let claims = Claims::new(id.clone(), iat, exp);

        encode(&jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA), &claims, &self.encoding_key)
            .map_err(|e| {
                match e.kind() {
                    
                    _ => AuthError::CreateError
                }
            })
    }

    pub fn validate(&self, token: &String) -> Result<Claims, AuthError> {
        let claims = decode::<Claims>(&token, &self.decoding_key, &self.validation)
            .map_err(|e| {
                match e.kind() {

                    _ => AuthError::ValidateError
                }
            })?.claims;
        
        Ok(claims)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    #[serde(with = "jwt_time")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_time")]
    exp: OffsetDateTime
}

impl Claims {
    fn new(id: String, iat: OffsetDateTime, exp: OffsetDateTime) -> Self {
        let iat = iat
            .date()
            .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();

        let exp = exp
            .date()
            .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
            .unwrap()
            .assume_utc();

        Claims { id, iat, exp }
    }
}

// https://serde.rs/custom-date-format.html
// https://github.com/Keats/jsonwebtoken/blob/master/examples/custom_time.rs
mod jwt_time {
    use serde::{Deserialize, Serializer, Deserializer};
    use time::OffsetDateTime;

        pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let timestamp = date.unix_timestamp();
            serializer.serialize_i64(timestamp)

        }
    
        pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
        where
            D: Deserializer<'de>,
        {
            // i64::deserialize 要把 serde::Deserialize 包進來
            OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
                .map_err(|_| serde::de::Error::custom("custom deserialize error"))

        }
}

#[cfg(test)]
mod test {
    use time::{OffsetDateTime, Duration};

    use crate::jwt::eddsa::EdDsaJwt;
    use crate::jwt::error::AuthError;

    use super::Claims;


    #[test]
    fn offset_date_serialize() {
        let iat = OffsetDateTime::now_utc();
        let exp = iat.saturating_add(Duration::minutes(20));
        let ser_claims = Claims::new(String::from("id"), iat, exp);
    
        let json = serde_json::to_string(&ser_claims).unwrap();

        println!("claims: {:?}", ser_claims);
        println!("serialize: {json}");

        let de_claims: Claims = serde_json::from_str(&json).unwrap();

        assert_eq!(de_claims.id, ser_claims.id);
        assert_eq!(de_claims.iat, ser_claims.iat);
        assert_eq!(de_claims.exp, ser_claims.exp);
    }

    #[test]
    fn jwt_test() {
        let jwt_coding = EdDsaJwt::default();

        let id = String::from("hi");
        let token = jwt_coding.signature(&id).unwrap();

        let claims = jwt_coding.validate(&token).unwrap();

        assert_eq!(id, claims.id);
    }

    #[test]
    fn jwt_validate_fail_test() {
        let jwt = EdDsaJwt::default();

        let id = String::from("id");
        let mut token = jwt.signature(&id).unwrap();
        token.push_str("string");

        let validate = jwt.validate(&token);
    
        match validate {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(AuthError::ValidateError, e)
        };
    }

}