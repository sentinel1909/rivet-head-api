// src/lib/routes/mod.rs

pub mod diary;
pub mod health_check;
pub mod info;
pub mod not_found;

pub use diary::*;
pub use health_check::*;
pub use info::*;
pub use not_found::*;
