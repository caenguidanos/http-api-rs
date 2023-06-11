use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub typee: String,
    pub title: String,
    pub detail: String,
    pub status: u16,
}

impl ProblemDetails {
    pub fn from_400() -> Self {
        let mut problem_details = Self::default();
        problem_details
            .set_type("https://www.rfc-editor.org/rfc/rfc9110.html#name-400-bad-request")
            .set_status(400)
            .set_title("Bad Request");
        problem_details
    }

    pub fn from_401() -> Self {
        let mut problem_details = Self::default();
        problem_details
            .set_type("https://www.rfc-editor.org/rfc/rfc9110.html#name-401-unauthorized")
            .set_status(401)
            .set_title("Unauthorized");
        problem_details
    }

    pub fn from_403() -> Self {
        let mut problem_details = Self::default();
        problem_details
            .set_type("https://www.rfc-editor.org/rfc/rfc9110.html#name-403-forbidden")
            .set_status(403)
            .set_title("Forbidden");
        problem_details
    }

    pub fn from_503() -> Self {
        let mut problem_details = Self::default();
        problem_details
            .set_type("https://www.rfc-editor.org/rfc/rfc9110.html#name-503-service-unavailable")
            .set_status(503)
            .set_title("Service Unavailable");
        problem_details
    }

    pub fn set_type(&mut self, value: impl ToString) -> &mut Self {
        self.typee = value.to_string();
        self
    }

    pub fn set_title(&mut self, value: impl ToString) -> &mut Self {
        self.title = value.to_string();
        self
    }

    pub fn set_detail(&mut self, value: impl ToString) -> &mut Self {
        self.detail = value.to_string();
        self
    }

    pub fn set_status(&mut self, value: u16) -> &mut Self {
        self.status = value;
        self
    }
}
