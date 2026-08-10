//! All the data sent between egui and the backend

mod code;
pub mod input;
mod key;
pub mod output;
mod user_data;

pub use code::Code;
pub use key::{Key, KeyExt, NamedKey};
pub use user_data::UserData;
