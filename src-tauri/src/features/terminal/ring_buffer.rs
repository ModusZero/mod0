/// Búfer circular de bytes para repetir la salida de una terminal al re-adjuntarla.
pub struct RingBuffer {
    data: Vec<u8>,
    capacity: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity.min(64 * 1024)),
            capacity,
        }
    }

    pub fn push(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);

        if self.data.len() > self.capacity {
            let mut cut = self.data.len() - self.capacity;

            // Intenta cortar en un salto de línea para no partir secuencias de escape.
            if let Some(pos) = self.data[cut..].iter().position(|b| *b == b'\n') {
                cut += pos + 1;
            }

            self.data.drain(..cut.min(self.data.len()));
        }
    }

    pub fn snapshot(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_the_tail() {
        let mut buffer = RingBuffer::new(16);
        buffer.push(b"0123456789\nabcdefghij\nXYZ");
        let snapshot = buffer.snapshot();
        assert!(snapshot.len() <= 16);
        assert!(snapshot.ends_with(b"XYZ"));
    }

    #[test]
    fn small_writes_are_kept_whole() {
        let mut buffer = RingBuffer::new(64);
        buffer.push(b"hola ");
        buffer.push(b"mundo");
        assert_eq!(buffer.snapshot(), b"hola mundo");
    }
}
