pub mod mysql {
    pub const VALIDATION_QUERY: &str = "/* ping */ SELECT 1";
}

pub mod postgres {
    pub const VALIDATION_QUERY: &str = "SELECT 1";
}

pub mod sqlite {
    pub const VALIDATION_QUERY: &str = "SELECT 1";
}

pub trait ValidationQuery {
    fn name() -> &'static str;
    fn validation_query() -> &'static str;
}

#[cfg(feature = "diesel")]
mod diesel {
    use super::ValidationQuery;

    #[cfg(feature = "diesel-mysql")]
    impl ValidationQuery for diesel::mysql::Mysql {
        fn name() -> &'static str {
            "diesel-mysql"
        }

        fn validation_query() -> &'static str {
            super::mysql::VALIDATION_QUERY
        }
    }

    #[cfg(feature = "diesel-postgres")]
    impl ValidationQuery for diesel::pg::Pg {
        fn name() -> &'static str {
            "diesel-postgres"
        }

        fn validation_query() -> &'static str {
            super::postgres::VALIDATION_QUERY
        }
    }

    #[cfg(feature = "diesel-sqlite")]
    impl ValidationQuery for diesel::sqlite::Sqlite {
        fn name() -> &'static str {
            "diesel-sqlite"
        }

        fn validation_query() -> &'static str {
            super::sqlite::VALIDATION_QUERY
        }
    }
}

#[cfg(feature = "sqlx")]
mod sqlx {
    use super::ValidationQuery;

    #[cfg(feature = "sqlx-mysql")]
    impl ValidationQuery for sqlx::mysql::MySql {
        fn name() -> &'static str {
            "sqlx-mysql"
        }

        fn validation_query() -> &'static str {
            super::mysql::VALIDATION_QUERY
        }
    }

    #[cfg(feature = "sqlx-postgres")]
    impl ValidationQuery for sqlx::postgres::Postgres {
        fn name() -> &'static str {
            "sqlx-postgres"
        }

        fn validation_query() -> &'static str {
            super::postgres::VALIDATION_QUERY
        }
    }

    #[cfg(feature = "sqlx-sqlite")]
    impl ValidationQuery for sqlx::sqlite::Sqlite {
        fn name() -> &'static str {
            "sqlx-sqlite"
        }

        fn validation_query() -> &'static str {
            super::sqlite::VALIDATION_QUERY
        }
    }
}
