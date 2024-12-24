use colored::{ColoredString, Colorize};
use h3i::{
    actions::h3::{send_headers_frame, Action},
    config::Config,
};
use h3i::{
    client::{
        connection_summary::{ConnectionSummary, ExpectedFrames},
        sync_client, ClientError,
    },
    frame::ExpectedFrame,
};
use pin_project_lite::pin_project;
use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::task::{JoinHandle, JoinSet};

pub struct Section {
    name: String,
    cases: Vec<TestCase>,
}

impl Section {
    pub fn new(test_name: &str, cases: Vec<TestCase>) -> Self {
        let mut name = String::from("\n");
        name.push_str(test_name);

        Self { name, cases }
    }

    pub async fn run(self) {
        // TODO: this should have progress bars for each test, and the fraction should update on
        // each thing, similar to nextest ideally.
        // Only print the full report when all tests are done
        println!("{}", self.name.bold().underline());
        let mut s: JoinSet<ColoredString> = JoinSet::from_iter(self.cases);

        while let Some(Ok(finished)) = s.join_next().await {
            println!(" - {finished}");
        }
    }
}

pub enum Verifier {
    ExpectedFrames(Vec<ExpectedFrame>),
    Fn(Box<dyn Fn(&ConnectionSummary) -> bool + Send + Sync>),
}

#[derive(Debug)]
pub enum TestResult {
    Pass { test_name: String },
    // TODO: propagate the error somewhere
    Fail { test_name: String },
    // Error in the test code somewhere
    Error(String),
}

impl TestResult {
    fn print(&self) -> ColoredString {
        match self {
            Self::Pass { test_name } => test_name.green(),
            Self::Fail { test_name } => test_name.red(),
            Self::Error(error) => error.yellow().underline(),
        }
    }
}

pin_project! {
#[must_use = "test cases must be .awaited"]
    pub struct TestCase {
        name: String,
        verifier: Verifier,
        #[pin]
        handle: JoinHandle<Result<ConnectionSummary, ClientError>>
    }
}

impl TestCase {
    pub fn new(name: &str, config: Config, actions: Vec<Action>, verifier: Verifier) -> Self {
        let expected_frames = if let Verifier::ExpectedFrames(ref ef) = verifier {
            Some(ExpectedFrames::new(ef.clone()))
        } else {
            None
        };

        let task_handle = tokio::task::spawn_blocking(move || {
            sync_client::connect(config, &actions, expected_frames)
        });

        Self {
            name: name.to_owned(),
            handle: task_handle,
            verifier,
        }
    }
}

impl Future for TestCase {
    type Output = ColoredString;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        let handle = me.handle;
        let verifier = me.verifier;
        let name = me.name;

        let summary = match handle.poll(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(summary) => summary
                .expect("task handle panic")
                .expect("connection failure"),
        };

        let test_res = match verifier {
            Verifier::ExpectedFrames(_) => {
                let Some(missing) = summary.stream_map.missing_frames() else {
                    return Poll::Ready(TestResult::Error("no missing frames".to_string()).print());
                };

                missing.is_empty()
            }
            Verifier::Fn(f) => f(&summary),
        };

        let mapped = if test_res {
            TestResult::Pass {
                test_name: name.to_string(),
            }
        } else {
            TestResult::Fail {
                test_name: name.to_string(),
            }
        };

        let colorized = mapped.print();
        Poll::Ready(colorized)
    }
}

impl Debug for TestCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
