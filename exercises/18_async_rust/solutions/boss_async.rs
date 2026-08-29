// Solution: Boss Battle — Async Task Coordinator
// ================================================

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
#[derive(Debug, Clone, PartialEq)]
struct TaskResult {
    task_name: String,
    value: String,
    polls_taken: u32,
}

// =================================================================
// Part 2: Task
// =================================================================
struct Task {
    name: String,
    remaining_polls: u32,
    total_polls: u32,
    result_value: String,
}

impl Task {
    fn new(name: &str, polls_needed: u32, result_value: &str) -> Self {
        Task {
            name: name.to_string(),
            remaining_polls: polls_needed,
            total_polls: polls_needed,
            result_value: result_value.to_string(),
        }
    }

    fn is_complete(&self) -> bool {
        self.remaining_polls == 0
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Unpin for Task {}

impl Future for Task {
    type Output = TaskResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.remaining_polls == 0 {
            Poll::Ready(TaskResult {
                task_name: this.name.clone(),
                value: this.result_value.clone(),
                polls_taken: this.total_polls,
            })
        } else {
            this.remaining_polls -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// =================================================================
// Part 3: Coordinator
// =================================================================
struct Coordinator {
    tasks: Vec<Task>,
    completed: Vec<TaskResult>,
}

impl Coordinator {
    fn new() -> Self {
        Coordinator {
            tasks: Vec::new(),
            completed: Vec::new(),
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn poll_all(&mut self) -> usize {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        let mut still_pending = Vec::new();

        // Drain tasks and poll each one
        for mut task in self.tasks.drain(..) {
            match Pin::new(&mut task).poll(&mut cx) {
                Poll::Ready(result) => {
                    self.completed.push(result);
                }
                Poll::Pending => {
                    still_pending.push(task);
                }
            }
        }

        let pending_count = still_pending.len();
        self.tasks = still_pending;
        pending_count
    }

    fn run_to_completion(&mut self) -> u32 {
        let mut rounds = 0;
        while !self.tasks.is_empty() {
            self.poll_all();
            rounds += 1;
        }
        rounds
    }

    fn results(&self) -> &[TaskResult] {
        &self.completed
    }

    fn pending_count(&self) -> usize {
        self.tasks.len()
    }

    fn completed_count(&self) -> usize {
        self.completed.len()
    }
}

// =================================================================
// Part 4: Async helper functions
// =================================================================

async fn run_single_task(name: &str, polls: u32, value: &str) -> TaskResult {
    let task = Task::new(name, polls, value);
    task.await
}

async fn run_sequential(
    name1: &str, polls1: u32, value1: &str,
    name2: &str, polls2: u32, value2: &str,
) -> (TaskResult, TaskResult) {
    let result1 = Task::new(name1, polls1, value1).await;
    let result2 = Task::new(name2, polls2, value2).await;
    (result1, result2)
}

fn main() {
    println!("=== Boss Battle: Async Task Coordinator ===");
    println!();

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
