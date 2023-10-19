use jwt_simple::prelude::{Ed25519KeyPair, EdDSAKeyPairLike, Ed25519PublicKey, Claims, Duration, EdDSAPublicKeyLike};
use tracing::{event, Level};

pub struct EdDSAJwt {
    encoding_key: Ed25519KeyPair,
    decoding_key: Ed25519PublicKey
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JwtClaims {
    i: String
}

impl EdDSAJwt {
    pub fn new() -> Self {
        let ed_pair = Ed25519KeyPair::generate();
        let ed_pk = ed_pair.public_key();

        EdDSAJwt { encoding_key: ed_pair, decoding_key: ed_pk }
    }

    pub fn create(&self, id: &String) -> Result<String, Error> {
        let claims = JwtClaims{
            i: id.to_string()
        };

        let claims = Claims::with_custom_claims(claims, Duration::from_mins(30));
        let sign_result = self.encoding_key.sign(claims);

        match sign_result {
            Ok(j) => Ok(j),
            Err(e) => {
                println!("get jwt error {e}");
                Err(Error::CreateError)
            }
        }
    }

    pub fn validate(&self, jwt: &String) -> Result<JwtClaims, Error> {
        let decoded = self.decoding_key.verify_token::<JwtClaims>(&jwt, None);

        match decoded {
            Ok(d) => {
                Ok(d.custom)
            },
            Err(e) => {
                event!(Level::ERROR, "validate error {e}");
                match e {
                    _ => Err(Error::ValidateError),
                }
            }
        }
    }

    pub fn validate_renew(&self, jwt: String) -> Result<String, Error> {
        let c = self.validate(&jwt);

        match c {
            Ok(claims) => self.create(&claims.i),
            Err(_) => Err(Error::ValidateError)
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    CreateError,
    ValidateError
}

#[cfg(test)]
mod test {

    use std::sync::{OnceLock, Arc};

    use crate::jwt::jwt::Error;

    use super::EdDSAJwt;    

    fn key() -> Arc<EdDSAJwt>{
        static ED: OnceLock<Arc<EdDSAJwt>> = OnceLock::new();
        Arc::clone(ED.get_or_init(|| Arc::new(EdDSAJwt::new())))
    }

    #[test]
    fn create_validate_jwt() {
        let id = "test";
        let key_pair = key();
        let jwt = key_pair.create(&id.to_string()).unwrap();
        let claims = key_pair.validate(&jwt).unwrap();

        println!("jwt {jwt}");

        assert_eq!(id.to_string(), claims.i);
    }

    #[test]
    fn renew_test() {
        let id = "id";

        let key = key();
        let jwt = key.create(&id.to_string()).unwrap();
        let renew_jwt = key.validate_renew(jwt).unwrap();
        let claims = key.validate(&renew_jwt).unwrap();

        assert_eq!(id, claims.i);
    }

    #[test]
    fn validate_error() {
        let key1 = EdDSAJwt::new();
        let key2 = EdDSAJwt::new();
        
        let id = String::from("id");

        let jwt = key1.create(&id).unwrap();
        let  validate_result = key2.validate(&jwt);

        if let Err(e) = validate_result {
            assert_eq!(Error::ValidateError, e);
        } else {
            assert_eq!(false, true);
        }
    }

}