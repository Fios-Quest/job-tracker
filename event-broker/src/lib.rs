use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug};
use uuid::Uuid;

/// This trait is used to represent a function that can handle an event
pub trait Handler<E>: 'static {
    fn handle(&self, event: E);
}

impl<E, F> Handler<E> for F
where
    F: Fn(E) + 'static,
{
    fn handle(&self, event: E) {
        (*self)(event);
    }
}

/// Because of the nature of the broker, these errors can usually be ignored.
#[derive(PartialEq, Debug)]
pub enum BrokerError {
    NoHandlerForEvent,
    NoHandlerForId,
}

impl fmt::Display for BrokerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for BrokerError {}

type Result<T> = std::result::Result<T, BrokerError>;

/// The event Broker allows you to register to receive notification of events, as well as emit
/// events.
#[derive(Default)]
pub struct Broker {
    handlers: HashMap<TypeId, HashMap<Uuid, Box<dyn Any>>>,
}

impl Broker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an event handler. It will be called when a matching event emitted.
    ///
    /// This method will error if there is no event handler configured for that event. If the
    /// event is "fire and forget" then the error can safely be ignored.
    ///
    /// ```
    /// # use event_broker::*;
    ///
    /// // This will handle messages of String
    /// let handler = |message: String| println!("{message}");
    ///
    /// // Create an event broker and register the handler
    /// let mut broker = Broker::new();
    /// let _handler_id = broker.register(Box::new(handler));
    ///
    /// // Emit the event
    /// assert!(broker.emit("Hello, Broker!".to_string()).is_ok());
    /// ```
    pub fn register<H, E>(&mut self, handler: H) -> Uuid
    where
        H: Handler<E>,
        E: Clone + Debug + Any,
    {
        let uuid = Uuid::new_v4();
        let type_id = TypeId::of::<E>();
        self.handlers
            .entry(type_id)
            .or_default()
            .insert(uuid, Box::new(handler));
        uuid
    }

    /// Unregister an existing event handler. It will no longer be
    ///
    /// ```
    /// # use event_broker::*;
    ///
    /// // This will handle messages of String
    /// let handler = |message: String| println!("{message}");
    ///
    /// // Create an event broker and register the handler
    /// let mut broker = Broker::new();
    /// let handler_id = broker.register(Box::new(handler));
    ///
    /// // Emit the event
    /// assert!(broker.emit("Hello, Broker!".to_string()).is_ok());
    ///
    /// // Remove the handler
    /// assert!(broker.unregister::<String>(handler_id).is_ok());
    ///
    /// // This time the emitter will error, though you can ignore this if you don't care if your
    /// // event is being listened for
    /// assert_eq!(broker.emit("Good bye, Broker!".to_string()), Err(BrokerError::NoHandlerForEvent));
    /// ```
    pub fn unregister<E>(&mut self, uuid: Uuid) -> Result<()>
    where
        E: Clone + Debug + Any,
    {
        let type_id = TypeId::of::<E>();

        self.handlers
            .get_mut(&type_id)
            .ok_or(BrokerError::NoHandlerForEvent)
            .map(|handlers| handlers.remove(&uuid))?
            .ok_or(BrokerError::NoHandlerForId)
            .map(|_| {})
    }

    /// Emit an event
    ///
    /// ```
    /// # use event_broker::*;
    /// // This will handle messages of String
    /// let handler = |message: String| println!("{message}");
    ///
    /// // Create an event broker and register the handler
    /// let mut broker = Broker::new();
    /// broker.register(Box::new(handler));
    /// broker.emit("Hello, Broker!".to_string())
    ///     .expect("You can usually ignore this error if your event is fire and forget");
    /// ```
    pub fn emit<E>(&mut self, event: E) -> Result<()>
    where
        E: Clone + Debug + Any,
    {
        let handlers = self
            .handlers
            .get_mut(&TypeId::of::<E>())
            .ok_or(BrokerError::NoHandlerForEvent)?;

        if handlers.is_empty() {
            return Err(BrokerError::NoHandlerForEvent);
        }

        handlers
            .values_mut()
            .filter_map(|handle| handle.downcast_mut::<Box<dyn Handler<E>>>())
            .for_each(|handle| handle.handle(event.clone()));

        Ok(())
    }
}
