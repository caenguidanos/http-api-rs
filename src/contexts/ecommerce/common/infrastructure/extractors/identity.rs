use std::collections::HashSet;
use std::sync::OnceLock;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::contexts::ecommerce::common;
use crate::libs;

static WELL_KNOWN: OnceLock<String> = OnceLock::new();
static WELL_KNOWN_TTL: OnceLock<chrono::DateTime<chrono::offset::Utc>> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityClaims {
    pub sub: Option<String>,
    pub permissions: Option<HashSet<String>>,
}

impl IdentityClaims {
    pub fn check_permission(&self, permission: common::domain::Permissions) -> Result<(), common::domain::Error> {
        let Some(permissions) = self.permissions.clone() else {
            tracing::error!("not found required permission");
            return Err(common::domain::Error::InvalidPermission);
        };

        if !permissions.contains(permission.to_string().as_str()) {
            tracing::error!("not found required permission");
            return Err(common::domain::Error::InvalidPermission);
        }

        Ok(())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for IdentityClaims
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header = parts.headers.get("authorization");

        let Some(header) = header else {
            tracing::error!("not found token");
            let problem_details = libs::problem_details::ProblemDetails::from_401();
            return Err(libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response());
        };

        let Ok(header_as_str) = header.to_str() else {
            tracing::error!("malformed token {:?}", header);
            let problem_details = libs::problem_details::ProblemDetails::from_401();
            return Err(libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response());
        };

        let header_parts: Vec<&str> = header_as_str.split(' ').collect();

        if header_parts.len() != 2 {
            tracing::error!("malformed token {:?}", header);
            let problem_details = libs::problem_details::ProblemDetails::from_401();
            return Err(
                libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response(),
            );
        }

        if header_parts.first().copied() != Some("Bearer") {
            tracing::error!("malformed token {:?}", header);
            let problem_details = libs::problem_details::ProblemDetails::from_401();
            return Err(
                libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response(),
            );
        }

        tracing::debug!("header_parts={:?}", header_parts);

        let Some(header_token) = header_parts.get(1).copied() else {
            tracing::error!("impossible to read token {:?}", header);
            let problem_details = libs::problem_details::ProblemDetails::from_401();
            return Err(libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response());
        };

        tracing::debug!("header_token={:?}", header_token);

        let Ok(decoded_header_token) = jsonwebtoken::decode_header(header_token) else {
            tracing::error!("impossible to decode token {:?}", header);
            let problem_details = libs::problem_details::ProblemDetails::from_401();
            return Err(libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response());
        };

        tracing::debug!("decoded_header_token={:?}", decoded_header_token);

        let Some(decoded_header_token_kid) = decoded_header_token.kid else {
            tracing::error!("impossible to extract KID {:?}", header);
            let problem_details = libs::problem_details::ProblemDetails::from_401();
            return Err(libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response());
        };

        tracing::debug!("decoded_header_token_kid={:?}", decoded_header_token_kid);

        let jwk = JwkWellKnown::new().await;

        let Ok(jwks_content) = serde_json::from_str::<jsonwebtoken::jwk::JwkSet>(&jwk.content) else {
            tracing::error!("impossible to extract well-known {:?}", header);
            let problem_details = libs::problem_details::ProblemDetails::from_401();
            return Err(libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response());
        };

        tracing::debug!("jwks_content={:?}", jwks_content);

        let Some(jwk) = jwks_content.find(&decoded_header_token_kid) else {
            tracing::error!("impossible to find KID {:?}", header);
            let problem_details = libs::problem_details::ProblemDetails::from_401();
            return Err(libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response());
        };

        tracing::debug!("jwk={:?}", jwk);

        return match jwk.clone().algorithm {
            jsonwebtoken::jwk::AlgorithmParameters::RSA(ref rsa) => {
                tracing::debug!("rsa={:?}", rsa);

                let Ok(identity_provider_domain) = std::env::var("OAUTH_DOMAIN") else {
                    tracing::error!("not found identity provider domain");
                    let problem_details = libs::problem_details::ProblemDetails::from_401();
                    return Err(libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response());
                };

                tracing::debug!("identity_provider_domain={:?}", identity_provider_domain);

                let Ok(identity_provider_audience) = std::env::var("OAUTH_AUDIENCE") else {
                    tracing::error!("not found identity provider audience");
                    let problem_details = libs::problem_details::ProblemDetails::from_401();
                    return Err(libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response());
                };

                tracing::debug!("identity_provider_audience={:?}", identity_provider_audience);

                let identity_provider_issuer = &format!("{identity_provider_domain}/");

                tracing::debug!("identity_provider_issuer={:?}", identity_provider_issuer);

                let mut rs256_validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
                rs256_validation.set_audience(&[identity_provider_audience]);
                rs256_validation.set_issuer(&[identity_provider_issuer]);

                let Ok(decoding_key) = jsonwebtoken::DecodingKey::from_rsa_components(&rsa.n, &rsa.e) else {
                    tracing::error!("impossible to decode rsa key {:?}", header);
                    let problem_details = libs::problem_details::ProblemDetails::from_401();
                    return Err(libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details).into_response());
                };

                return match jsonwebtoken::decode::<IdentityClaims>(header_token, &decoding_key, &rs256_validation) {
                    Ok(token_data) => {
                        tracing::debug!("claims={:?}", token_data.claims);
                        Ok(token_data.claims)
                    }
                    Err(error) => {
                        tracing::error!("impossible to decode token data: {:?}", error);
                        let problem_details = libs::problem_details::ProblemDetails::from_401();
                        Err(
                            libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details)
                                .into_response(),
                        )
                    }
                };
            }
            _ => {
                tracing::error!("invalid algorithm {:?}", header);
                let problem_details = libs::problem_details::ProblemDetails::from_401();
                Err(
                    libs::encoding::JsonResponse::with_status(StatusCode::UNAUTHORIZED, problem_details)
                        .into_response(),
                )
            }
        };
    }
}

struct JwkWellKnown {
    pub content: String,
}

#[async_trait]
trait JwkRetriever {
    async fn new() -> Self;
    async fn fetch() -> String;
}

#[async_trait]
impl JwkRetriever for JwkWellKnown {
    async fn new() -> Self {
        if WELL_KNOWN_TTL.get().is_none() {
            return Self {
                content: Self::fetch().await,
            };
        }

        // only fetch new jwk each 12h to prevent DDOS negation service from the identity provider
        let now = chrono::offset::Utc::now();

        // the last reading was more than 12 hours ago
        if (*WELL_KNOWN_TTL.get().unwrap_or(&now) + chrono::Duration::hours(12)) < now {
            return Self {
                content: Self::fetch().await,
            };
        }

        tracing::debug!("fetching cached jwks");

        Self {
            content: WELL_KNOWN.get().unwrap_or(&String::new()).clone(),
        }
    }

    #[cfg(test)]
    async fn fetch() -> String {
        tracing::debug!("generate new jwks from local pem");

        let jwks = include_str!("../../../../../../static/oauth2/jwks_from_public_pem.json");
        jwks.to_string()
    }

    #[cfg(not(test))]
    async fn fetch() -> String {
        tracing::debug!("generate new jwks from remote well-known");

        let oauth_domain = std::env::var("OAUTH_DOMAIN").expect("OAUTH_DOMAIN");

        if let Ok(response) = reqwest::get(format!("{oauth_domain}/.well-known/jwks.json")).await {
            if let Ok(text) = response.text().await {
                if let Ok(_) = WELL_KNOWN.set(text.clone()) {
                    if let Ok(_) = WELL_KNOWN_TTL.set(chrono::offset::Utc::now()) {
                        return text;
                    }
                }
            }
        }

        String::new()
    }
}

#[cfg(test)]
pub mod fixture {
    use std::collections::HashSet;

    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct ClaimsFixture {
        aud: Option<String>,
        exp: Option<usize>,
        iat: Option<usize>,
        iss: Option<String>,
        nbf: Option<usize>,
        sub: Option<String>,
        permissions: Option<HashSet<String>>,
    }

    pub fn encode_jwt(perm: &[&str]) -> String {
        let exp = chrono::offset::Utc::now().timestamp_millis() + 60 * 1000;
        let iat = chrono::offset::Utc::now().timestamp_millis() - 60 * 1000;

        let aud = "https://random.test.com";
        let dom = "https://random.test.com";

        std::env::set_var("OAUTH_AUDIENCE", aud);
        std::env::set_var("OAUTH_DOMAIN", dom);

        let mut permissions = HashSet::new();
        for permission in perm {
            permissions.insert(permission.to_string());
        }

        let mut header = Header::new(Algorithm::RS256);
        // must match with JWK generated from local `test_public.pem` file
        header.kid = Some("example_kid".to_string());

        let claims = ClaimsFixture {
            aud: Some(aud.to_string()),
            exp: Some(exp as usize),
            iat: Some(iat as usize),
            iss: Some(format!("{dom}/")),
            nbf: None,
            sub: None,
            permissions: Some(permissions),
        };

        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_rsa_pem(include_bytes!("../../../../../../static/oauth2/test_private.pem")).unwrap(),
        )
        .unwrap();

        format!("Bearer {token}")
    }
}
