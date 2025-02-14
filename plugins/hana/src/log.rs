

/// Append debug log entry to stdout
#[macro_export]
macro_rules! std_debug {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        kovi::log::debug!("{}", content);
    }};
}

/// Append info log entry to stdout
#[macro_export]
macro_rules! std_info {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        kovi::log::info!("{}", content);
    }};
}

/// Append warn log entry to stdout
#[macro_export]
macro_rules! std_warn {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        kovi::log::warn!("{}", content);
    }};
}

/// Append error log entry to stdout
#[macro_export]
macro_rules! std_error {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        kovi::log::error!("{}", content);
    }};
}

/// Append debug log entry to database.
#[macro_export]
macro_rules! db_debug {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        let time = $crate::util::cur_time_iso8601();
        $crate::store::db_write_bot_log(time, "DEBUG".to_string(), content).await;
    }};
}

/// Append info log entry to database.
#[macro_export]
macro_rules! db_info {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        let time = $crate::util::cur_time_iso8601();
        $crate::store::db_write_bot_log(time, "INFO".to_string(), content).await;
    }};
}

/// Append warn log entry to database.
#[macro_export]
macro_rules! db_warn {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        let time = $crate::util::cur_time_iso8601();
        $crate::store::db_write_bot_log(time, "WARN".to_string(), content).await;
    }};
}

/// Append error log entry to database.
#[macro_export]
macro_rules! db_error {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        let time = $crate::util::cur_time_iso8601();
        $crate::store::db_write_bot_log(time, "ERROR".to_string(), content).await;
    }};
}

/// Append debug log entry to stdout and database.
#[macro_export]
macro_rules! std_db_debug {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        let time = $crate::util::cur_time_iso8601();
        kovi::log::debug!("{}", content);
        $crate::store::db_write_bot_log(time, "DEBUG".to_string(), content).await;
    }};
}

/// Append info log entry to stdout and database.
#[macro_export]
macro_rules! std_db_info {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        let time = $crate::util::cur_time_iso8601();
        kovi::log::info!("{}", content);
        $crate::store::db_write_bot_log(time, "INFO".to_string(), content).await;
    }};
}

/// Append warn log entry to stdout and database.
#[macro_export]
macro_rules! std_db_warn {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        let time = $crate::util::cur_time_iso8601();
        kovi::log::warn!("{}", content);
        $crate::store::db_write_bot_log(time, "WARN".to_string(), content).await;
    }};
}

/// Append error log entry to stdout and database.
#[macro_export]
macro_rules! std_db_error {
    ($($t:tt)*) => {{
        let content = indoc::formatdoc!($($t)*);
        let time = $crate::util::cur_time_iso8601();
        kovi::log::error!("{}", content);
        $crate::store::db_write_bot_log(time, "ERROR".to_string(), content).await;
    }};
}