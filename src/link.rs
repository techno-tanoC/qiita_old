pub struct Link {
    total_count: u16,
    first: Option<String>,
    prev: Option<String>,
    next: Option<String>,
    last: Option<String>,
}

impl Link {
    pub fn new(total_count: u16, first: Option<String>, prev: Option<String>, next: Option<String>, last: Option<String>) -> Self {
        Link {
            total_count,
            first,
            prev,
            next,
            last,
        }
    }

    pub fn total_count(&self) -> u16 {
        self.total_count
    }

    pub fn first(&self) -> Option<&str> {
        match self.first {
            Some(ref s) => Some(s),
            None => None,
        }
    }

    pub fn prev(&self) -> Option<&str> {
        match self.prev {
            Some(ref s) => Some(s),
            None => None,
        }
    }

    pub fn next(&self) -> Option<&str> {
        match self.next {
            Some(ref s) => Some(s),
            None => None,
        }
    }

    pub fn last(&self) -> Option<&str> {
        match self.last {
            Some(ref s) => Some(s),
            None => None,
        }
    }
}
