use bytes::Bytes;
use bytes::BytesMut;
use std::cmp::min;
use std::collections::LinkedList;
use std::sync::Condvar;
use std::sync::Mutex;

#[derive(Default)]
struct BufferInner {
    elements: LinkedList<BytesMut>,
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

    pub fn write(&mut self, bytes: BytesMut) {
        let mut guard = self.inner.lock().unwrap();
        guard.elements.push_back(bytes);
        self.cond.notify_one();
    }

    pub fn eof(&mut self) {
        let mut guard = self.inner.lock().unwrap();
        guard.closed = true;
        self.cond.notify_one();
    }

    pub fn read(&mut self, size: usize) -> Result<Bytes, crate::Error> {
        let mut guard = self.inner.lock().unwrap();
        let mut bytes = BytesMut::with_capacity(size);
        while bytes.len() < size {
            if let Some(head) = guard.elements.front_mut() {
                let split_size = min(size - bytes.len(), head.len());
                bytes.extend(head.split_to(split_size));
                if head.is_empty() {
                    guard.elements.pop_front();
                }
                continue;
            }

            if !bytes.is_empty() {
                break;
            }

            if guard.closed {
                return Err(crate::Error::EOF);
            }

            guard = self.cond.wait(guard).unwrap();
        }
        Ok(bytes.freeze())
    }
}

#[cfg(test)]
mod tests {
    use bytes::BufMut;

    use super::*;

    #[test]
    fn test_buffer_read_write_at_capacity() {
        let mut b = Buffer::new();
        let mut to_write = BytesMut::with_capacity(1);
        to_write.put_u8(1);
        b.write(to_write);
        let read_bytes = b.read(1).unwrap();
        assert_eq!(read_bytes.as_ref(), &[1]);
    }

    #[test]
    fn test_buffer_read_write_exceeds_capacity() {
        let mut b = Buffer::new();
        let mut to_write = BytesMut::with_capacity(1);
        to_write.put_u8(1);
        to_write.put_u8(2);
        b.write(to_write);
        let read_bytes = b.read(1).unwrap();
        assert_eq!(read_bytes.as_ref(), &[1]);
        let read_bytes = b.read(1).unwrap();
        assert_eq!(read_bytes.as_ref(), &[2]);
    }

    #[test]
    fn test_buffer_read_write_multi_write() {
        let mut b = Buffer::new();
        let mut to_write = BytesMut::with_capacity(1);
        to_write.put_u8(1);
        b.write(to_write);
        let mut to_write = BytesMut::with_capacity(1);
        to_write.put_u8(2);
        b.write(to_write);
        let read_bytes = b.read(1).unwrap();
        assert_eq!(read_bytes.as_ref(), &[1]);
        let read_bytes = b.read(1).unwrap();
        assert_eq!(read_bytes.as_ref(), &[2]);
    }

    #[test]
    fn test_buffer_read_write_close() {
        let mut b = Buffer::new();
        let mut to_write = BytesMut::with_capacity(1);
        to_write.put_u8(1);
        b.write(to_write);
        b.eof();
        let read_bytes = b.read(1).unwrap();
        assert_eq!(read_bytes.as_ref(), &[1]);
        let eof_error = b.read(1).unwrap_err();
        assert_eq!(eof_error, crate::Error::EOF);
    }
}
