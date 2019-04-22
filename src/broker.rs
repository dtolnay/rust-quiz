use std::fmt;
use std::io::{self, Write};

use parking_lot::Mutex;
use termcolor::{Buffer, BufferWriter, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    output: Buffer,
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
        let stderr = BufferWriter::stderr(ColorChoice::Auto);

        while self.pending.front().done {
            let task = self.pending.pop();
            let _ = stderr.print(&task.output);
        }

        let head = self.pending.front();
        let _ = stderr.print(&head.output);
        head.output.clear();
    }
}

impl Default for Task {
    fn default() -> Self {
        Task {
            output: BufferWriter::stderr(ColorChoice::Auto).buffer(),
            done: false,
        }
    }
}

#[readonly::make]
pub struct Handle<'a> {
    broker: &'a Broker,
    #[readonly]
    pub index: usize,
}

impl<'a> Handle<'a> {
    pub fn write_fmt(&self, args: fmt::Arguments) {
        let _ = self.apply(|w| w.write_fmt(args));
    }

    fn apply<T>(&self, f: impl FnOnce(&mut dyn WriteColor) -> T) -> T {
        let mut inner = self.broker.inner.lock();

        if self.index == inner.pending.offset() {
            f(&mut StandardStream::stderr(ColorChoice::Auto))
        } else {
            f(&mut inner.pending.get(self.index).output)
        }
    }
}

impl<'a> Write for Handle<'a> {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.apply(|w| w.write(b))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.apply(|w| w.flush())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.apply(|w| w.write_all(buf))
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> io::Result<()> {
        self.apply(|w| w.write_fmt(args))
    }
}

impl<'a> WriteColor for Handle<'a> {
    fn supports_color(&self) -> bool {
        self.apply(|w| w.supports_color())
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        self.apply(|w| w.set_color(spec))
    }

    fn reset(&mut self) -> io::Result<()> {
        self.apply(|w| w.reset())
    }
}

impl<'a> Drop for Handle<'a> {
    fn drop(&mut self) {
        let mut inner = self.broker.inner.lock();

        inner.pending.get(self.index).done = true;
        inner.catch_up();
    }
}
