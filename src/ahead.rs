use std::collections::VecDeque;

/// FIFO queue that can be indexed arbitrarily far ahead.
pub struct AheadQueue<T> {
    offset: usize,
    elements: VecDeque<T>,
}

impl<T> AheadQueue<T> {
    /// New empty queue.
    pub fn new() -> Self {
        AheadQueue {
            offset: 0,
            elements: VecDeque::new(),
        }
    }

    /// Number of elements so far popped from the queue.
    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl<T> AheadQueue<T>
where
    T: Default,
{
    pub fn front(&mut self) -> &mut T {
        if self.elements.is_empty() {
            self.elements.push_back(T::default());
        }
        &mut self.elements[0]
    }

    pub fn pop(&mut self) -> T {
        self.offset += 1;
        self.elements.pop_front().unwrap_or_default()
    }

    pub fn get(&mut self, index: usize) -> &mut T {
        while index >= self.offset + self.elements.len() {
            self.elements.push_back(T::default());
        }
        &mut self.elements[index - self.offset]
    }
}
