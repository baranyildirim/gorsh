use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use std::collections::LinkedList;
use std::sync::Condvar;
use std::sync::Mutex;

#[derive(Default)]
struct BufferInner {
    elements: LinkedList<Bytes>,
    closed: bool,
}

pub struct Buffer {
    cond: Condvar,
    inner: Mutex<BufferInner>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            cond: Condvar::new(),
            inner: Mutex::new(BufferInner::default()),
        }
    }

    pub fn write(&mut self, bytes: Bytes) {
        let mut guard = self.inner.lock().unwrap();
        guard.elements.push_back(bytes);
        self.cond.notify_one();
    }

    pub fn eof(&mut self) {
        let mut guard = self.inner.lock().unwrap();
        guard.closed = true;
        self.cond.notify_one();
    }

    pub fn read(&mut self) -> Result<BytesMut, crate::Error> {
        let mut guard = self.inner.lock().unwrap();
        let mut bytes = BytesMut::new();
        loop {
            if let Some(head) = &guard.elements.pop_front() {
                bytes.extend_from_slice(head.as_ref());
                continue;
            }

            if !bytes.is_empty() {
                return Ok(bytes);
            }

            if guard.closed {
                return Err(crate::Error::EOF);
            }
            guard = self.cond.wait(guard).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
