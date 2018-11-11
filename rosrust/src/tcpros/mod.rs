pub use self::client::Client;
pub use self::error::Error;
pub use self::publisher::{Publisher, PublisherStream};
pub use self::service::Service;
pub use self::subscriber::Subscriber;
use rosmsg::RosMsg;

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use Clock;

mod client;
pub mod error;
mod header;
mod publisher;
mod service;
mod subscriber;
mod util;

pub type ServiceResult<T> = Result<T, String>;

pub trait Message: RosMsg + Send + 'static {
    fn msg_definition() -> String;
    fn md5sum() -> String;
    fn msg_type() -> String;
    fn set_header(&mut self, _clock: &Arc<Clock>, _seq: &Arc<AtomicUsize>) {}
}

pub trait ServicePair: Message {
    type Request: RosMsg + Send + 'static;
    type Response: RosMsg + Send + 'static;
}

#[derive(Clone, Debug)]
pub struct Topic {
    pub name: String,
    pub msg_type: String,
}
