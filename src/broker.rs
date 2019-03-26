use std::fmt;
use std::io::{self, Write as _};
use std::ops::Deref;

use parking_lot::Mutex;
use ref_cast::RefCast;

use crate::ahead::AheadQueue;

/// Mechanism to prevent interleaving of output of tasks while still allowing an
/// arbitrary number of tasks to make progress, even ones other than the task
/// currently printing output.
///
/// # Objective
///
///   - We have an ordered sequence of tasks 0..N.
///
///   - We want to receive all output from task 0, then all output from task 1,
///     etc in order. Task output must not interleave with other tasks and must
///     follow the task order.
///
///   - We want tasks to execute in parallel.
///
///   - We want all output to be printed as soon as possible, meaning real time
///     for exactly one task at a time and deferred until replacement of the
///     realtime task for other tasks.
///
pub struct Broker {
    inner: Mutex<Inner>,
}

struct Inner {
    /// Index of next started task.
    head: usize,
    pending: AheadQueue<Task>,
}

struct Task {
    output: Vec<u8>,
    done: bool,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            inner: Mutex::new(Inner {
                head: 0,
                pending: AheadQueue::new(),
            }),
        }
    }

    pub fn begin(&self) -> Handle {
        let mut inner = self.inner.lock();

        let index = inner.head;
        inner.head += 1;

        Handle {
            broker: self,
            index,
        }
    }
}

impl Inner {
    fn catch_up(&mut self) {
        let stderr = io::stderr();
        let mut stderr = stderr.lock();

        while self.pending.front().done {
            let task = self.pending.pop();
            let _ = stderr.write_all(&task.output);
        }

        let head = self.pending.front();
        let _ = stderr.write_all(&head.output);
        head.output.clear();
    }
}

impl Default for Task {
    fn default() -> Self {
        Task {
            output: Vec::new(),
            done: false,
        }
    }
}

pub struct Handle<'a> {
    broker: &'a Broker,
    index: usize,
}

impl<'a> Handle<'a> {
    pub fn write_fmt(&self, args: fmt::Arguments) {
        let mut inner = self.broker.inner.lock();

        if self.index == inner.pending.offset() {
            let _ = io::stderr().write_fmt(args);
        } else {
            let _ = inner.pending.get(self.index).output.write_fmt(args);
        }
    }
}

impl<'a> Drop for Handle<'a> {
    fn drop(&mut self) {
        let mut inner = self.broker.inner.lock();

        inner.pending.get(self.index).done = true;
        inner.catch_up();
    }
}

impl<'a> Deref for Handle<'a> {
    type Target = HasIndex;

    fn deref(&self) -> &Self::Target {
        HasIndex::ref_cast(&self.index)
    }
}

#[derive(RefCast)]
#[repr(transparent)]
pub struct HasIndex {
    pub index: usize,
}
