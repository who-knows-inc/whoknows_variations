
extern crate rocket;

pub mod routes {
    pub mod api {
        pub mod login; // Eksponer login-modulet
    }
}

pub mod models {
    pub mod user; // Eksponer user-modulet
}

pub mod security {
    pub mod security; // Eksponer security-modulet
}
