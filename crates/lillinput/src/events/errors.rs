//! Errors related to events.

use std::io::Error as IoError;

use crate::controllers::errors::ControllerError;
use filedescriptor::Error as FileDescriptorError;
use input::event::gesture::GestureSwipeEvent;
use thiserror::Error;

/// Errors raised during `libinput` initialization.
#[derive(Error, Debug)]
pub enum LibinputError {
    /// Error while assigning seat to the libinput context.
    #[error("error while assigning seat to the libinput context")]
    SeatError,
}

/// Custom error issued during the main loop.
///
/// This custom error message captures the errors emitted during the main loop,
/// which wrap over:
/// * [`filedescriptor::Error`] (during [`filedescriptor::poll`]).
/// * [`std::io::Error`] (during [`input::Libinput::dispatch`]).
#[derive(Error, Debug)]
pub enum MainLoopError {
    /// Unknown error while dispatching libinput event.
    #[error("unknown error while dispatching libinput event")]
    DispatchError(#[from] IoError),

    /// Unknown error while polling for the file descriptor.
    #[error("unknown error while polling the file descriptor")]
    IOError(#[from] FileDescriptorError),
}

/// Errors raised during the processing of an event.
#[derive(Error, Debug)]
pub enum ProcessEventError {
    /// Unsupported swipe event.
    #[error("unsupported swipe event ({:?})", .0)]
    UnsupportedSwipeEvent(GestureSwipeEvent),

    /// Action controller was not able to process the event.
    #[error("acton controller was not able to process the event {0}")]
    ControllerError(#[from] ControllerError),
}
