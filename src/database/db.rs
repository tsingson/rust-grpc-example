pub struct DB {
    pub url: &'static str,
}

impl DB {
    pub fn url() -> Self {
        DB { url: "postgres://postgres:postgresPWD@localhost:5445/demo" }
    }
}

