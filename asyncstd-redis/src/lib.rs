use async_std;
pub mod connectionpool;
pub mod connect;

pub enum Value {
    ///A Redis `OK` response.
    Ok,
    ///Nil Response.
    Nil,
    ///Array response.
    Array(Vec<Value>),
    ///Integer response.
    Integer(isize),
    ///String response. This cannot be a `String` type, because Redis strings need not be valid UTF-8, unlike Rust.
    String(Vec<u8>),
}