
extern crate rocket;

pub mod routes {
    pub mod api {
        pub mod login; 
        pub mod weather;
        pub mod search;
        pub mod register; 
        pub mod delete_user;
    }

}

pub mod models {
    pub mod user; // Eksponer user-modulet
}

pub mod security {
    pub mod security; // Eksponer security-modulet
}
