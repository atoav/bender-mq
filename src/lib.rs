//! bender_mq is a rust library, that implements additional methods for the
//! amqp crate via traits  
//!
//! It can be loaded in a rust library via the public git mirror by putting this in your Cargo.toml:  
//! ```text
//! [dependencies]
//! bender_mq = { git = "https://github.com/atoav/bender-mq.git" }
//! ```
//! To update this run
//! ```text
//! cargo clean
//! cargo update
//! ```
//!
//! ## Testing
//! The tests can be run with
//! ```text
//! cargo test
//! ```
//!
//! ## Documentation
//! If you want to view the documentation run
//! ```text
//! cargo doc --no-deps --open
//! ```
//! 
//! ## Installation
//! This is a library and will not be directly used. No need to install anything here
//! 
//! ## API Methods
//! Check out the examples at the [BenderMQ](trait.BenderMQ.html) trait definition


extern crate bender_config;
extern crate bender_job;
extern crate amqp;

use bender_job::task::Task;
use bender_config::Config;
use bender_job::Job;
use amqp::{Basic, Session, Table, protocol};
pub use amqp::Channel;


type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;




/// A trait for Channel to make it easier to post info
pub trait BenderMQ{
    /// Open a AMPQ session and return a channel. The method can be used like this:
    /// ```
    /// extern crate bender_mq;
    /// use bender_mq::{Channel, BenderMQ};
    /// let url = "amqp://localhost//";
    /// let channel = Channel::open_channel(url).expect("Couldn't aquire connection.");
    /// ```
    fn open_channel<S>(url: S) -> GenResult<Self> 
    where S: Into<String>, Self: std::marker::Sized;

    /// Open a AMPQ session and return a channel to the default URK specified in\
    /// the config. The method can be used like this:
    /// ```
    /// extern crate bender_mq;
    /// use bender_mq::{Channel, BenderMQ};
    /// let channel = Channel::open_default_channel().expect("Couldn't aquire connection.");
    /// ```
    fn open_default_channel() -> GenResult<Self> where Self: std::marker::Sized;

    /// Declare a topic exchange named `info-topic`. Messages to this exchange \
    /// may be posted using the `post_to_info()` and the `post_job()` methods.
    /// ```
    /// # extern crate bender_mq;
    /// # use bender_mq::{Channel, BenderMQ};
    /// let mut channel = Channel::open_default_channel().expect("Couldn't aquire connection.");
    /// channel.declare_topic_exchange().expect("Declaration of topic exchange failed");
    /// ```
    fn declare_topic_exchange(&mut self) -> GenResult<()>;

    /// Declare a direct exchange named `job`. Messages to this exchange \
    /// may be posted using the `post_job()` method.
    /// ```
    /// # extern crate bender_mq;
    /// # use bender_mq::{Channel, BenderMQ};
    /// let mut channel = Channel::open_default_channel().expect("Couldn't aquire connection.");
    /// channel.declare_job_exchange().expect("Declaration of job exchange failed");
    /// ```
    fn declare_job_exchange(&mut self) -> GenResult<()>;

    /// Declare a direct exchange named `work`. Messages to this exchange \
    /// may be posted using the `post_task()` or `post_to_work()` methods.
    /// ```
    /// # extern crate bender_mq;
    /// # use bender_mq::{Channel, BenderMQ};
    /// let mut channel = Channel::open_default_channel().expect("Couldn't aquire connection.");
    /// channel.declare_work_exchange().expect("Declaration of work exchange failed");
    /// ```
    fn declare_work_exchange(&mut self) -> GenResult<()>;

    /// Declare a direct exchange named `worker-topic`. Messages to this exchange \
    /// may be posted using the `worker_post()` method.
    /// ```
    /// # extern crate bender_mq;
    /// # use bender_mq::{Channel, BenderMQ};
    /// let mut channel = Channel::open_default_channel().expect("Couldn't aquire connection.");
    /// channel.declare_worker_exchange().expect("Declaration of worker-topic exchange failed");
    /// ```
    fn declare_worker_exchange(&mut self) -> GenResult<()>;

    /// Declare a queue named `info`. This queue will be bound to the exchange \
    /// named `info-topic`.
    fn create_info_queue(&mut self) -> GenResult<()>;

    /// Declare a queue named `job`. This queue will be bound to the exchange \
    /// named `job`.
    fn create_job_queue(&mut self) -> GenResult<()>;

    /// Declare a queue named `work`. This queue will be bound to the exchange \
    /// named `work`.
    fn create_work_queue(&mut self) -> GenResult<()>;

    /// Declare a queue named `worker`. This queue will be bound to the exchange \
    /// named `worker-topic`.
    fn create_worker_queue(&mut self) -> GenResult<()>;

    /// Post a routed message to `info-topic` exchange with a routing key of your choice
    fn post_to_info<S, U>(&mut self, routing_key: S, message: U) where S: Into<String>, U: Into<Vec<u8>>;
    
    /// Post a direct message to `job` exchange
    fn post_to_job<U>(&mut self, message: U) where U: Into<Vec<u8>>;

    /// Post a direct message to `work` exchange
    fn post_to_work<U>(&mut self, message: U) where U: Into<Vec<u8>>;

    /// Post a routed message to `worker-topic` exchange with a routing key of your choice
    fn worker_post<S, U>(&mut self, routing_key: S, message: U) where S: Into<String>, U: Into<Vec<u8>>;

    /// Serialize a job and post it to the the `job` exchange using the \
    /// `post_to_job()` method. Get the serialized json back for debouncing
    fn post_job(&mut self, job: &Job) -> GenResult<String>;

    /// Serialize a job and post it to the the `topic-info` exchange using the \
    /// `post_to_info()` method. Get the serialized json back for debouncing
    fn post_job_info(&mut self, job: &Job) -> GenResult<String>;

    /// Serialize a task and post it to the the `task` exchange using the \
    /// `post_to_work()` method. Get the serialized json back for debouncing
    fn post_task(&mut self, task: &Task) -> GenResult<String>;

    /// Serialize a task and post it to the the `topic-info` exchange using the \
    /// `post_to_info()` method. Get the serialized json back for debouncing
    fn post_task_info<S>(&mut self, task: &Task, routing_key: S) -> GenResult<String> where S: Into<String>;
}


impl BenderMQ for Channel{
    /// Open a AMPQ session and return a channel.
    fn open_channel<S>(url: S) -> GenResult<Self> where S: Into<String>{
        let url = url.into();
        let mut session = Session::open_url(url.as_str()).unwrap_or_else(|_| panic!("Error while opening a connection to {}", url));
        let channel = session.open_channel(1)?;
        Ok(channel)
    }

    /// Open a AMPQ session and return a channel to the default URK specified in\
    /// the config.
    fn open_default_channel() -> GenResult<Self>{
        let config = Config::get();
        let mut session = Session::open_url(config.rabbitmq.url.as_str()).unwrap_or_else(|_| panic!("Error while opening a connection to {}", config.rabbitmq.url));
        let channel = session.open_channel(1)?;
        Ok(channel)
    }


    /// Declare a topic exchange named `info-topic`. Messages to this exchange \
    /// may be posted using the `post_to_info()` and the `post_job()` methods.
    fn declare_topic_exchange(&mut self) -> GenResult<()>{
        let exchange_name = "info-topic";
        let exchange_type = "topic";
        // exchange name, exchange type, passive, durable, auto_delete, internal, nowait, arguments
        self.exchange_declare(exchange_name, exchange_type, false, true, false, false, false, Table::new())?;
        Ok(())
    }

    /// Create a info queue that is bound to the info-topic exchange
    fn create_info_queue(&mut self) -> GenResult<()>{
        let queue_name = "info";
        let exchange_name = "info-topic";
        //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
        self.queue_declare(queue_name, false, true, false, false, false, Table::new())?;
        // queue: S, exchange: S, routing_key: S, nowait: bool,a rguments: Table
        self.queue_bind(queue_name, exchange_name, "#", false, Table::new())?;
        Ok(())
    }

    /// Declare a direct exchange named `job`. Messages to this exchange \
    /// may be posted using the `post_job()` method.
    fn declare_job_exchange(&mut self) -> GenResult<()>{
        let exchange_name = "job";
        let exchange_type = "direct";
        // exchange name, exchange type, passive, durable, auto_delete, internal, nowait, arguments
        // posibble exchange types are: direct, fanout, topic, headers
        self.exchange_declare(exchange_name, exchange_type, false, true, false, false, false, Table::new())?;
        Ok(())
    }

    /// Create a Job queue that is bound to the job exchange
    fn create_job_queue(&mut self) -> GenResult<()>{
        let queue_name = "job";
        // let exchange_name = "job";
        //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
        self.queue_declare(queue_name, false, true, false, false, false, Table::new())?;
        // queue: S, exchange: S, routing_key: S, nowait: bool,a rguments: Table
        // self.queue_bind(queue_name, exchange_name, "#", false, Table::new())?;
        Ok(())
    }

    /// Declare a direct exchange named `work`. Messages to this exchange \
    /// may be posted using the `post_work()` method.
    fn declare_work_exchange(&mut self) -> GenResult<()>{
        let exchange_name = "work";
        let exchange_type = "direct";
        // exchange name, exchange type, passive, durable, auto_delete, internal, nowait, arguments
        // posibble exchange types are: direct, fanout, topic, headers
        self.exchange_declare(exchange_name, exchange_type, false, true, false, false, false, Table::new())?;
        Ok(())
    }

    // Declare a topic exchange named `worker`. Messages to this exchange \
    /// may be posted using the `worker_post()` method.
    fn declare_worker_exchange(&mut self) -> GenResult<()>{
        let exchange_name = "worker-topic";
        let exchange_type = "topic";
        // exchange name, exchange type, passive, durable, auto_delete, internal, nowait, arguments
        // posibble exchange types are: direct, fanout, topic, headers
        self.exchange_declare(exchange_name, exchange_type, false, true, false, false, false, Table::new())?;
        Ok(())
    }

    /// Create a Work queue that is bound to the work exchange
    fn create_work_queue(&mut self) -> GenResult<()>{
        let queue_name = "work";
        // let exchange_name = "work";
        //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
        self.queue_declare(queue_name, false, true, false, false, false, Table::new())?;
        // queue: S, exchange: S, routing_key: S, nowait: bool,a rguments: Table
        // self.queue_bind(queue_name, exchange_name, "#", false, Table::new())?;
        Ok(())
    }

    /// Create a worker queue that is bound to the info-topic exchange
    fn create_worker_queue(&mut self) -> GenResult<()>{
        let queue_name = "worker";
        let exchange_name = "worker-topic";
        //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
        self.queue_declare(queue_name, false, true, false, false, false, Table::new())?;
        // queue: S, exchange: S, routing_key: S, nowait: bool,a rguments: Table
        self.queue_bind(queue_name, exchange_name, "#", false, Table::new())?;
        Ok(())
    }

    /// Post a message to `info-topic` exchange with a routing key of your choice
    fn post_to_info<S, U>(&mut self, routing_key: S, message: U) where S: Into<String>, U: Into<Vec<u8>>{
        // let queue_name = "info";
        let exchange = "info-topic";
        let mandatory = true;
        let immediate = false;
        let routing_key = routing_key.into();
        let routing_key = routing_key.as_str();
        let properties = protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()};
        let message = message.into();
        if let Err(err) = self.basic_publish(exchange, routing_key, mandatory, immediate, properties, message) { 
            println!("Error: Couldn't publish message to info-topic exchange: {}", err) 
        }
    }


    /// Post a direct message to `job` exchange
    fn post_to_job<U>(&mut self, message: U) where U: Into<Vec<u8>>{
        let mandatory = true;
        let immediate = false;
        let routing_key = "job".to_string();
        let properties = protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()};
        let message = message.into();
        if let Err(err) = self.basic_publish("", routing_key.as_str(), mandatory, immediate, properties, message) { 
            println!("Error: Couldn't publish message to job exchange: {}", err) 
        }
    }

    /// Post a direct message to `work` exchange
    fn post_to_work<U>(&mut self, message: U) where U: Into<Vec<u8>>{
        let mandatory = true;
        let immediate = false;
        let routing_key = "work".to_string();
        let properties = protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()};
        let message = message.into();
        if let Err(err) = self.basic_publish("", routing_key.as_str(), mandatory, immediate, properties, message) { 
            println!("Error: Couldn't publish message to info-topic exchange: {}", err) 
        }
    }

    // Post a message to `worker-topic` exchange with a routing key of your choice
    fn worker_post<S, U>(&mut self, routing_key: S, message: U) where S: Into<String>, U: Into<Vec<u8>>{
        // let queue_name = "worker";
        let exchange = "worker-topic";
        let mandatory = true;
        let immediate = false;
        let routing_key = routing_key.into();
        let routing_key = routing_key.as_str();
        let properties = protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()};
        let message = message.into();
        if let Err(err) = self.basic_publish(exchange, routing_key, mandatory, immediate, properties, message) { 
            println!("Error: Couldn't publish message to info-topic exchange: {}", err) 
        }
    }


    /// Serialize a job and post it to the the `job` exchange using the \
    /// `post_to_job()` method. Get the serialized json back for debouncing
    fn post_job(&mut self,job: &Job) -> GenResult<String>{
        match job.serialize(){
            Ok(json) => {
                self.post_to_job(json.as_str());
                Ok(json)
            },
            Err(err) => Err(err)
        }
    }

    /// Serialize a job and post it to the the `topic-info` exchange using the \
    /// `post_to_info()` method. Get the serialized json back for debouncing
    fn post_job_info(&mut self, job: &Job) -> GenResult<String>{
        match job.serialize(){
            Ok(json) => {
                self.post_to_info(job.id().as_str(), json.as_str());
                Ok(json)
            },
            Err(err) => Err(err)
        }
    }

    /// Serialize a task and post it to the the `task` exchange using the \
    /// `post_to_task()` method. Get the serialized json back for debouncing
    fn post_task(&mut self, task: &Task) -> GenResult<String>{
        match task.serialize(){
            Ok(json) => {
                self.post_to_work(json.as_str());
                Ok(json)
            },
            Err(err) => Err(err)
        }
    }


    fn post_task_info<S>(&mut self, task: &Task, routing_key: S) -> GenResult<String> where S: Into<String>{
        let routing_key = routing_key.into();
        match task.serialize(){
            Ok(json) => {
                self.post_to_info(routing_key.as_str(), json.as_str());
                Ok(json)
            },
            Err(err) => Err(err)
        }
    }

}