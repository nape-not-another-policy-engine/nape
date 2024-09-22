use std::cell::RefCell;
use std::rc::Rc;
use clap::{ArgMatches, Command};
use nape_kernel::error::{Error, Kind};
use crate::io_adapter::clap::command_handler_boundary::CommandHandlerBoundary;
use crate::io_adapter::clap::command_handlers::collect::collect_command_handler::CollectCommandHandler;

/// Test when a subcommand matches: This test will ensure that the handle method behaves correctly when one of the subcommands' names matches the arguments. The expected result is the result of the subcommand's handle method.
#[test]
fn collect_command_handle_success() {
    let mock_handler = MockCommandHandler::new("test");
    let mock_handler_2 = MockCommandHandler::new("test-2");
    let collect_handler = CollectCommandHandler::new(vec![Box::new(mock_handler.clone()), Box::new(mock_handler_2.clone())]);

    let matches = Command::new("collect")
        .subcommand(Command::new("test"))
        .subcommand(Command::new("test-2"))
        .get_matches_from(vec![ "collect", "test"]);

    let result = collect_handler.handle(&matches);

    assert!(result.is_ok());
    assert_eq!(*mock_handler.was_called.borrow(), true);
}

/// Test when no subcommand matches: This test will ensure that the handle method behaves correctly when none of the subcommands' names match the arguments. The expected result is Ok(()).
#[test]
fn no_subcommand_matches() {
    let mock_handler = MockCommandHandler::new("test");
    let collect_handler = CollectCommandHandler::new(vec![Box::new(mock_handler.clone())]);

    let matches = Command::new("collect")
        .subcommand(Command::new("test"))
        .get_matches_from(vec!["collect"]);

    let result = collect_handler.handle(&matches);

    assert!(result.is_ok());
    assert_eq!(*mock_handler.was_called.borrow(), false);
}

/// Test when a subcommand's handle method returns an error: This test will ensure that the handle method correctly propagates errors from a subcommand's handle method.
#[test]
fn test_subcommand_handle_returns_error() {
    let mock_handler = MockCommandHandlerError::new("test");
    let collect_handler = CollectCommandHandler::new(vec![Box::new(mock_handler.clone())]);

    let matches = Command::new("collect")
        .subcommand(Command::new("test"))
        .get_matches_from(vec!["collect", "test"]);

    let result = collect_handler.handle(&matches);

    assert!(result.is_err());

}



/*** Mock Command Handler for testing ***/
#[derive(Clone)]
struct MockCommandHandler {
    name: &'static str,
    was_called: Rc<RefCell<bool>>
}
impl MockCommandHandler {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            was_called: Rc::new(RefCell::new(false)),
        }
    }
}
impl CommandHandlerBoundary for MockCommandHandler {
    fn name(&self) -> &str {
        self.name
    }

    fn handle(&self, _args: &ArgMatches) -> Result<(), Error> {
        *self.was_called.borrow_mut() = true;
        Ok(())
    }
}

/*** Mock Command Error Handler for testing ***/
#[derive(Clone)]
struct MockCommandHandlerError {
    name: &'static str,
}
impl MockCommandHandlerError {
    fn new(name: &'static str) -> Self {
        Self { name }
    }
}
impl CommandHandlerBoundary for MockCommandHandlerError {
    fn name(&self) -> &str {
        self.name
    }

    fn handle(&self, _args: &ArgMatches) -> Result<(), Error> {
       Err(Error::for_system(Kind::GatewayError, "Error Occurred".to_string()))
    }
}