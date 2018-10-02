//! bender_mq is a rust library, that implements additional methods for the
//! amqp crate via traits  
//!
//! It can be loaded in a rust library via the public git mirror by putting this in your Cargo.toml:  
//! ```ignore
//! [dependencies]
//! bender_mq = { git = "ssh://git@code.hfbk.net:4242/bendercode/bender-mq.git" }
//! ```
//! To update this run
//! ```ignore
//! cargo clean
//! cargo update
//! ```
//!
//! ## Testing
//! The tests can be run with
//! ```ignore
//! cargo test
//! ```
//!
//! ## Documentation
//! If you want to view the documentation run
//! ```ignore
//! cargo doc --no-deps --open
//! ```
//! 
//! ## Installation
//! To run cargo, make sure you have rust installed. Go to [rustup.rs](http://rustup.rs) and follow the instructions there
//! 

extern crate bender_config;
extern crate bender_job;
extern crate amqp;

use bender_config::Config;
use bender_job::Job;
use amqp::{Basic, Session, Table, protocol};
pub use amqp::Channel;


type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;




/// A trait for Channel to make it easier to post info
pub trait BenderMQ{
    fn open_channel<S>(url: S) -> GenResult<Self> 
    where S: Into<String>, Self: std::marker::Sized;
    fn open_default_channel() -> GenResult<Self> where Self: std::marker::Sized;
    fn declare_topic_exchange(&mut self) -> GenResult<()>;
    fn declare_task_exchange(&mut self) -> GenResult<()>;
    fn post_to_info<S, U>(&mut self, routing_key: S, message: U) where S: Into<String>, U: Into<Vec<u8>>;
    fn post_task(&mut self, message: Vec<u8>);
    fn post_job(&mut self, job: Job) -> GenResult<String>;
}


impl BenderMQ for Channel{
    /// Open a AMPQ session and return a channel. The method can be used like this:
    /// ```
    /// extern crate bender_mq;
    /// use bender_mq::{Channel, BenderMQ};
    /// let url = "amqp://localhost//";
    /// let channel = Channel::open_channel(url).expect("Couldn't aquire connection.");
    /// ```
    fn open_channel<S>(url: S) -> GenResult<Self> where S: Into<String>{
        let url = url.into();
        let mut session = Session::open_url(url.as_str()).expect(format!("Error while opening a connection to {}", url).as_str());
        let channel = session.open_channel(1)?;
        Ok(channel)
    }

    /// Open a AMPQ session and return a channel to the default URK specified in\
    /// the config. The method can be used like this:
    /// ```
    /// extern crate bender_mq;
    /// use bender_mq::{Channel, BenderMQ};
    /// let channel = Channel::open_default_channel().expect("Couldn't aquire connection.");
    /// ```
    fn open_default_channel() -> GenResult<Self>{
        let p = Config::location();
        let config = Config::from_file(p).unwrap();
        let mut session = Session::open_url(config.rabbitmq.url.as_str()).expect(format!("Error while opening a connection to {}", config.rabbitmq.url).as_str());
        let channel = session.open_channel(1)?;
        Ok(channel)
    }


    /// Declare a topic exchange named `info-topic`. Messages to this exchange \
    /// may be posted using the `post_to_info()` and the `post_job()` methods.
    /// ```
    /// # extern crate bender_mq;
    /// # use bender_mq::{Channel, BenderMQ};
    /// let url = "amqp://localhost//";
    /// let mut channel = Channel::open_channel(url).expect("Couldn't aquire connection.");
    /// channel.declare_topic_exchange().expect("Declaration of topic exchange failed");
    /// ```
    fn declare_topic_exchange(&mut self) -> GenResult<()>{
        let exchange_name = "info-topic";
        let exchange_type = "topic";
        // exchange name, exchange type, passive, durable, auto_delete, internal, nowait, arguments
        self.exchange_declare(exchange_name, exchange_type, false, true, false, false, false, Table::new())?;
        Ok(())
    }

    /// Declare a direct exchange named `task`. Messages to this exchange \
    /// may be posted using the `post_task()` method.
    /// ```
    /// # extern crate bender_mq;
    /// # use bender_mq::{Channel, BenderMQ};
    /// let url = "amqp://localhost//";
    /// let mut channel = Channel::open_channel(url).expect("Couldn't aquire connection.");
    /// channel.declare_task_exchange().expect("Declaration of task exchange failed");
    /// ```
    /// Declare a direct exchange named task
    fn declare_task_exchange(&mut self) -> GenResult<()>{
        let exchange_name = "task";
        let exchange_type = "direct";
        // exchange name, exchange type, passive, durable, auto_delete, internal, nowait, arguments
        // posibble exchange types are: direct, fanout, topic, headers
        self.exchange_declare(exchange_name, exchange_type, false, true, false, false, false, Table::new())?;
        Ok(())
    }

    /// Post a message to `info-topic` exchange with a routing key of your choice
    fn post_to_info<S, U>(&mut self, routing_key: S, message: U) where S: Into<String>, U: Into<Vec<u8>>{
        let queue_name = "info";
        let exchange = "info-topic";
        let mandatory = true;
        let immediate = false;
        let routing_key = routing_key.into();
        let routing_key = routing_key.as_str();
        let properties = protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()};
        let message = message.into();
        // For some weird reason it works when the queue declare happens before AND after
        // queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
        self.queue_declare(queue_name, false, true, false, false, false, Table::new()).ok().expect("Queue Declare failed for post_to_info (1)");
        self.basic_publish(exchange, routing_key, mandatory, immediate, properties, message).ok().expect("Couldn't publish message to info-topic exchange");
        self.queue_declare(queue_name, false, true, false, false, false, Table::new()).ok().expect("Queue Declare failed for post_to_info (2)");
    }


    /// Post a message to `task` exchange with a routing key of your choice
    fn post_task(&mut self, message: Vec<u8>){
        let queue_name = "tasks";
        let exchange = "task";
        let routing_key = "tasks";
        let mandatory = true;
        let immediate = false;
        let properties = protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()};
        // For some weird reason it works when the queue declare happens before AND after
        // queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
        self.queue_declare(queue_name, false, true, false, false, false, Table::new()).ok().expect("Queue Declare failed for post_task (1)");
        self.basic_publish(exchange, routing_key, mandatory, immediate, properties, message).ok().expect("Couldn't publish message to task exchange");
        self.queue_declare(queue_name, false, true, false, false, false, Table::new()).ok().expect("Queue Declare failed for post_task (2)");
    }

    /// Serialize a job and post it to the the `topic-info` exchange using the \
    /// `post_to_info()` method. 
    fn post_job(&mut self, job: Job) -> GenResult<String>{
        match job.serialize(){
            Ok(json) => {
                self.post_to_info(job.id().as_str(), json.as_str());
                Ok(json)
            },
            Err(err) => Err(err)
        }
    }

}






#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
