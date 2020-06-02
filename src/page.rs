use serde::Serialize;
use std::default::Default;

#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub(crate) page: u16,
    pub(crate) per_page: u16,
}

impl Page {
    pub fn new(page: u16, per_page: u16) -> Option<Page> {
        if per_page <= 100 {
            Some(Page { page, per_page })
        }
        else {
            None
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Page {
            page: 1,
            per_page: 20,
        }
    }
}
