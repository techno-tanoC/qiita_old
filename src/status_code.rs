pub enum StatusCode {
    Ok,
    Created,
    NoContent,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl StatusCode {
    pub fn from_code(code: i16) -> Option<Self> {
        use StatusCode::*;

        match code {
            200 => Some(Ok),
            201 => Some(Created),
            204 => Some(NoContent),
            400 => Some(BadRequest),
            401 => Some(Unauthorized),
            403 => Some(Forbidden),
            404 => Some(NotFound),
            500 => Some(InternalServerError),
            _ => None,
        }
    }

    pub fn to_code(&self) -> u16 {
        use StatusCode::*;

        match self {
            Ok => 200,
            Created => 201,
            NoContent => 204,
            BadRequest => 400,
            Unauthorized => 401,
            Forbidden => 403,
            NotFound => 404,
            InternalServerError => 500,
        }
    }
}
