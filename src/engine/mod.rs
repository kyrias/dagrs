//! The Engine
//!
//! [`Dag`] consists of a series of executable tasks with dependencies. A Dag can be executed
//! alone as a job. We can get the execution result and execution status of dag.
//! [`Engine`] can manage multiple [`Dag`]. An Engine can consist of multiple Dags of different
//! types of tasks. For example, you can give a Dag in the form of a yaml configuration file,
//! then give a Dag in the form of a custom configuration file, and finally give it in a programmatic way.
//! [`Engine`] stores each Dag in the form of a key-value pair (<name:String,dag:Dag>), and the user
//! can specify which task to execute by giving the name of the Dag, or follow the order in which
//! the Dags are added to the Engine , executing each Dag in turn.

pub use dag::Dag;
use log::error;
use thiserror::Error;

mod dag;
mod graph;

/// Errors that may be raised by building and running dag jobs.
#[derive(Debug, Error)]
/// A synthesis of all possible errors.
pub enum DagError {
    /// Yaml file parsing error.
    #[error("Parsing error: {0}")]
    ParserError(String),
    /// Task dependency error.
    #[error("Task[{0}] dependency task not exist.")]
    RelyTaskIllegal(String),
    /// There are loops in task dependencies.
    #[error("Illegal directed a cyclic graph, loop Detect!")]
    LoopGraph,
    /// There are no tasks in the job.
    #[error("There are no tasks in the job.")]
    EmptyJob,
    /// Task error
    #[error("Task with ID {0} errored: {1}")]
    TaskError(usize, String),
}
