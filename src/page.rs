use std::default::Default;

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

    pub(crate) fn to_query(&self) -> String {
        format!("page={}&per_page={}", self.page, self.per_page)
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
