use jsonwebtoken::{decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
// use std::cell::LazyCell;
use std::sync::LazyLock;
use std::time::Duration;
const DEFAULT_SECRET: &str = "12345678";
//全局使用
static DEFAULT_JWT: LazyLock<JWT> = LazyLock::new(|| JWT::default());
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal
{
    pub id: String,
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims
{
    jti: String,
    sub: String,
    aud: String,
    iss: String,
    iat: u64,
    exp: u64,
}

#[derive(Debug)]
pub struct JwtConfig
{
    pub secret: Cow<'static, str>,
    pub issuer: String,
    pub audience: String,
    pub expiration: Duration,
}

impl Default for JwtConfig
{
    fn default() -> Self
    {
        Self
        {
            secret: Cow::Borrowed(DEFAULT_SECRET),
            issuer: "issuer".to_string(),
            audience: "audience".to_string(),
            expiration: Duration::from_secs(60*60)
        }
    }
}

#[derive(Debug, Clone)]
pub struct JWT
{
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,
    header: Header,
    validation: Validation,
    expiration: Duration,
    audience: String,
    issuer: String,
}

impl JWT
{
    pub fn new(config: JwtConfig) -> Self
    {
        let mut validation = Validation::new(Algorithm::HS512);
        validation.set_audience(&[&config.audience]);
        validation.set_issuer(&[&config.issuer]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);

        let secret = config.secret.as_bytes();

        Self
        {
            encode_secret: EncodingKey::from_secret(secret),
            decode_secret: DecodingKey::from_secret(secret),
            header: Header::new(Algorithm::HS512),
            validation,
            expiration: config.expiration,
            audience: config.audience,
            issuer: config.issuer,
        }
    }

    pub fn encode(&self, principal: Principal) -> anyhow::Result<String>
    {
        let current_timestamp = get_current_timestamp();
        let claims = Claims
        {
            jti: xid::new().to_string(),
            sub: format!("{}:{}",principal.id,principal.name),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_timestamp,
            exp: current_timestamp.saturating_add(self.expiration.as_secs()),
        };

        Ok( encode(&self.header, &claims, &self.encode_secret)? )
    }

    pub fn decode(&self, token: &str) -> anyhow::Result<Principal>
    {
        let claims: Claims = decode(token, &self.decode_secret, &self.validation)?.claims;

        let mut parts = claims.sub.splitn(2, ':');

        let principal = Principal
        {
            id: parts.next().unwrap().to_string(),
            name: parts.next().unwrap().to_string(),
        };

        Ok( principal )
    }
}

impl Default for JWT
{
    fn default() -> Self
    {
        Self::new(JwtConfig::default())
    }
}

pub fn get_jwt() -> &'static JWT
{
    &DEFAULT_JWT
}
