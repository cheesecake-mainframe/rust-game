// Boss Battle: Async Task Coordinator (BossBattle)
// ==================================================
//
// Build an async task coordinator that manages multiple async operations!
//
// This is a boss battle combining concepts from the entire async module:
//   - Custom Future implementations
//   - Manual polling with Poll::Ready/Pending
//   - Future combinators
//   - Async function composition
//   - Pin and Context usage
//
// You'll build a mini async task system from scratch using only std.
//
// The system has these components:
//   1. `Task` — a named future that tracks its own state
//   2. `TaskResult` — the outcome of a completed task
//   3. `Coordinator` — manages and polls multiple tasks
//   4. Async helper functions for building task pipelines

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

fn noop_waker() -> Waker {
    fn noop(_: *const ()) {}
    fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    unsafe { Waker::from_raw(raw) }
}

// =================================================================
// Part 1: TaskResult
// =================================================================
// Represents the result of a completed task.
#[derive(Debug, Clone, PartialEq)]
struct TaskResult {
    task_name: String,
    value: String,
    polls_taken: u32,
}

// =================================================================
// Part 2: Task
// =================================================================
// A future that simulates work by requiring multiple polls to complete.
// Each poll decrements `remaining_polls`. When it reaches 0, the task
// is "done" and returns a TaskResult.
//
// Fields:
//   name: String        — the task's name
//   remaining_polls: u32 — how many more polls until ready
//   total_polls: u32     — the original number of polls (for the result)
//   result_value: String — the value to include in TaskResult

struct Task {
    name: String,
    remaining_polls: u32,
    total_polls: u32,
    result_value: String,
}

impl Task {
    // TODO: Implement new
    fn new(name: &str, polls_needed: u32, result_value: &str) -> Self {
        todo!()
    }

    // TODO: Implement a helper to check if the task is complete
    fn is_complete(&self) -> bool {
        todo!()
    }

    // TODO: Implement a helper to get the task name
    fn name(&self) -> &str {
        todo!()
    }
}

impl Unpin for Task {}

// TODO: Implement Future for Task
// - If remaining_polls == 0, return Poll::Ready with a TaskResult
// - Otherwise, decrement remaining_polls, wake the waker, return Pending
impl Future for Task {
    type Output = TaskResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

// =================================================================
// Part 3: Coordinator
// =================================================================
// Manages a collection of tasks and polls them to completion.
// The coordinator keeps track of completed results and provides
// methods to drive tasks forward.

struct Coordinator {
    tasks: Vec<Task>,
    completed: Vec<TaskResult>,
}

impl Coordinator {
    // TODO: Create a new empty coordinator
    fn new() -> Self {
        todo!()
    }

    // TODO: Add a task to the coordinator
    fn add_task(&mut self, task: Task) {
        todo!()
    }

    // TODO: Poll all pending tasks once. Move completed tasks to the
    // `completed` list. Return the number of tasks still pending.
    //
    // Hint: Use Vec::retain or iterate and partition.
    // You'll need to create a waker and context to poll each task.
    fn poll_all(&mut self) -> usize {
        todo!()
    }

    // TODO: Keep polling until all tasks are complete.
    // Returns the total number of poll rounds needed.
    fn run_to_completion(&mut self) -> u32 {
        todo!()
    }

    // TODO: Return a reference to completed results
    fn results(&self) -> &[TaskResult] {
        todo!()
    }

    // TODO: Return how many tasks are still pending
    fn pending_count(&self) -> usize {
        todo!()
    }

    // TODO: Return how many tasks have completed
    fn completed_count(&self) -> usize {
        todo!()
    }
}

// =================================================================
// Part 4: Async helper functions
// =================================================================

// TODO: An async function that creates and awaits a Task, returning its result.
async fn run_single_task(name: &str, polls: u32, value: &str) -> TaskResult {
    todo!()
}

// TODO: An async function that runs two tasks sequentially (one after the other)
// and returns both results as a tuple.
async fn run_sequential(
    name1: &str, polls1: u32, value1: &str,
    name2: &str, polls2: u32, value2: &str,
) -> (TaskResult, TaskResult) {
    todo!()
}

fn main() {
    println!("=== Boss Battle: Async Task Coordinator ===");
    println!();

    // Demonstrate the coordinator
    let mut coord = Coordinator::new();
    coord.add_task(Task::new("download", 3, "file.txt"));
    coord.add_task(Task::new("compile", 2, "program.exe"));
    coord.add_task(Task::new("test", 1, "all passed"));

    println!("Starting with {} tasks", coord.pending_count());

    let rounds = coord.run_to_completion();
    println!("Completed in {} polling rounds", rounds);
    println!("Results:");
    for result in coord.results() {
        println!("  {} -> {} (took {} polls)",
            result.task_name, result.value, result.polls_taken);
    }

    println!();
    println!("Boss battle complete!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn poll_once<F: Future + Unpin>(future: &mut F) -> Poll<F::Output> {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        Pin::new(future).poll(&mut cx)
    }

    // --- Task tests ---

    #[test]
    fn test_task_immediate() {
        let mut task = Task::new("instant", 0, "done");
        let result = poll_once(&mut task);
        assert_eq!(result, Poll::Ready(TaskResult {
            task_name: "instant".to_string(),
            value: "done".to_string(),
            polls_taken: 0,
        }));
    }

    #[test]
    fn test_task_needs_multiple_polls() {
        let mut task = Task::new("slow", 3, "finished");

        assert_eq!(poll_once(&mut task), Poll::Pending);
        assert_eq!(poll_once(&mut task), Poll::Pending);
        assert_eq!(poll_once(&mut task), Poll::Pending);

        let result = poll_once(&mut task);
        assert_eq!(result, Poll::Ready(TaskResult {
            task_name: "slow".to_string(),
            value: "finished".to_string(),
            polls_taken: 3,
        }));
    }

    #[test]
    fn test_task_is_complete() {
        let mut task = Task::new("check", 1, "ok");
        assert!(!task.is_complete());
        poll_once(&mut task); // Pending, decrements to 0
        assert!(task.is_complete());
    }

    #[test]
    fn test_task_name() {
        let task = Task::new("my_task", 5, "value");
        assert_eq!(task.name(), "my_task");
    }

    // --- Coordinator tests ---

    #[test]
    fn test_coordinator_new() {
        let coord = Coordinator::new();
        assert_eq!(coord.pending_count(), 0);
        assert_eq!(coord.completed_count(), 0);
    }

    #[test]
    fn test_coordinator_add_task() {
        let mut coord = Coordinator::new();
        coord.add_task(Task::new("a", 1, "x"));
        coord.add_task(Task::new("b", 2, "y"));
        assert_eq!(coord.pending_count(), 2);
        assert_eq!(coord.completed_count(), 0);
    }

    #[test]
    fn test_coordinator_poll_all() {
        let mut coord = Coordinator::new();
        coord.add_task(Task::new("fast", 0, "quick"));
        coord.add_task(Task::new("slow", 2, "steady"));

        // First poll: "fast" completes (0 polls needed), "slow" goes Pending
        let pending = coord.poll_all();
        assert_eq!(pending, 1);
        assert_eq!(coord.completed_count(), 1);

        // Second poll: "slow" goes Pending (1 remaining)
        let pending = coord.poll_all();
        assert_eq!(pending, 1);

        // Third poll: "slow" completes
        let pending = coord.poll_all();
        assert_eq!(pending, 0);
        assert_eq!(coord.completed_count(), 2);
    }

    #[test]
    fn test_coordinator_run_to_completion() {
        let mut coord = Coordinator::new();
        coord.add_task(Task::new("a", 1, "x"));
        coord.add_task(Task::new("b", 3, "y"));
        coord.add_task(Task::new("c", 2, "z"));

        let rounds = coord.run_to_completion();
        // Task "b" needs the most polls (3), plus 1 final round to confirm Ready
        // So it takes 4 rounds total
        assert!(rounds >= 4);
        assert_eq!(coord.completed_count(), 3);
        assert_eq!(coord.pending_count(), 0);
    }

    #[test]
    fn test_coordinator_results() {
        let mut coord = Coordinator::new();
        coord.add_task(Task::new("only", 1, "result"));
        coord.run_to_completion();

        let results = coord.results();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].task_name, "only");
        assert_eq!(results[0].value, "result");
        assert_eq!(results[0].polls_taken, 1);
    }

    #[test]
    fn test_coordinator_empty() {
        let mut coord = Coordinator::new();
        let rounds = coord.run_to_completion();
        assert_eq!(rounds, 0);
        assert!(coord.results().is_empty());
    }

    // --- Async function tests ---

    #[test]
    fn test_run_single_task() {
        // `async fn` bodies are `!Unpin` (they may hold a reference across an
        // await point, so moving one after polling would dangle). `Box::pin`
        // pins it on the heap, and `Pin<Box<F>>` *is* `Unpin`.
        let mut future = Box::pin(run_single_task("test", 0, "ok"));
        let result = poll_once(&mut future);
        assert_eq!(result, Poll::Ready(TaskResult {
            task_name: "test".to_string(),
            value: "ok".to_string(),
            polls_taken: 0,
        }));
    }

    #[test]
    fn test_run_sequential() {
        // `async fn` bodies are `!Unpin` (they may hold a reference across an
        // await point, so moving one after polling would dangle). `Box::pin`
        // pins it on the heap, and `Pin<Box<F>>` *is* `Unpin`.
        let mut future = Box::pin(run_sequential(
            "first", 0, "a",
            "second", 0, "b",
        ));
        let result = poll_once(&mut future);
        assert_eq!(result, Poll::Ready((
            TaskResult {
                task_name: "first".to_string(),
                value: "a".to_string(),
                polls_taken: 0,
            },
            TaskResult {
                task_name: "second".to_string(),
                value: "b".to_string(),
                polls_taken: 0,
            },
        )));
    }
}
