use std::io::{Read, Write};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use portable_pty::{ChildKiller, CommandBuilder, MasterPty, PtySize, native_pty_system};
use tauri::ipc::{Channel, InvokeResponseBody};

use super::ring_buffer::RingBuffer;
use crate::core::error::{AppError, AppResult};
use crate::infrastructure::process::now_ms;

/// Tamaño del historial que se conserva por sesión para re-adjuntar la terminal.
const SCROLLBACK_BYTES: usize = 1024 * 1024;

/// Cómo arrancar el proceso dentro del PTY.
pub struct PtySpec {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub env: Vec<(String, String)>,
    pub rows: u16,
    pub cols: u16,
}

type Sink = Arc<Mutex<Option<Channel<InvokeResponseBody>>>>;

/// Un pseudo-terminal en ejecución. La salida se guarda siempre en un búfer y solo se
/// envía al frontend mientras haya un canal adjunto, así las sesiones en segundo plano
/// no cuestan nada en la interfaz.
pub struct Pty {
    master: Mutex<Box<dyn MasterPty + Send>>,
    writer: Mutex<Box<dyn Write + Send>>,
    killer: Mutex<Box<dyn ChildKiller + Send + Sync>>,
    buffer: Arc<Mutex<RingBuffer>>,
    sink: Sink,
    last_output: Arc<AtomicI64>,
}

impl Pty {
    pub fn spawn(spec: PtySpec, on_exit: impl FnOnce(Option<i32>) + Send + 'static) -> AppResult<Self> {
        let pair = native_pty_system()
            .openpty(PtySize {
                rows: spec.rows,
                cols: spec.cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| AppError::msg(format!("no se pudo abrir el PTY: {e}")))?;

        let mut command = CommandBuilder::new(&spec.program);
        command.args(&spec.args);
        command.cwd(&spec.cwd);
        for (key, value) in &spec.env {
            command.env(key, value);
        }

        let mut child = pair
            .slave
            .spawn_command(command)
            .map_err(|e| AppError::msg(format!("no se pudo iniciar `{}`: {e}", spec.program)))?;
        // El extremo esclavo debe cerrarse en el padre para detectar EOF correctamente.
        drop(pair.slave);

        let killer = child.clone_killer();
        let mut reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| AppError::msg(format!("no se pudo leer el PTY: {e}")))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| AppError::msg(format!("no se pudo escribir en el PTY: {e}")))?;

        let buffer = Arc::new(Mutex::new(RingBuffer::new(SCROLLBACK_BYTES)));
        let sink: Sink = Arc::new(Mutex::new(None));
        let last_output = Arc::new(AtomicI64::new(now_ms()));

        {
            let (buffer, sink, last_output) = (buffer.clone(), sink.clone(), last_output.clone());
            thread::spawn(move || {
                let mut chunk = [0u8; 16 * 1024];
                loop {
                    match reader.read(&mut chunk) {
                        Ok(0) | Err(_) => break,
                        Ok(n) => {
                            last_output.store(now_ms(), Ordering::Relaxed);

                            // Guardar y enviar bajo el mismo candado hace atómico el "adjuntar + historial".
                            let mut history = buffer.lock().expect("pty buffer");
                            history.push(&chunk[..n]);

                            if let Some(channel) = sink.lock().expect("pty sink").as_ref() {
                                let _ = channel.send(InvokeResponseBody::Raw(chunk[..n].to_vec()));
                            }
                        }
                    }
                }
            });
        }

        thread::spawn(move || {
            let code = child.wait().ok().map(|status| status.exit_code() as i32);
            on_exit(code);
        });

        Ok(Self {
            master: Mutex::new(pair.master),
            writer: Mutex::new(writer),
            killer: Mutex::new(killer),
            buffer,
            sink,
            last_output,
        })
    }

    pub fn write(&self, data: &[u8]) -> AppResult<()> {
        let mut writer = self.writer.lock().expect("pty writer");
        writer.write_all(data)?;
        writer.flush()?;
        Ok(())
    }

    pub fn resize(&self, rows: u16, cols: u16) -> AppResult<()> {
        self.master
            .lock()
            .expect("pty master")
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| AppError::msg(e.to_string()))
    }

    pub fn kill(&self) {
        let _ = self.killer.lock().expect("pty killer").kill();
    }

    /// Conecta un canal y devuelve el historial acumulado. Ningún byte se pierde ni se duplica:
    /// lo que ya estaba en el historial viaja en el retorno y lo posterior llega por el canal.
    pub fn attach(&self, channel: Channel<InvokeResponseBody>) -> Vec<u8> {
        let history = self.buffer.lock().expect("pty buffer");
        *self.sink.lock().expect("pty sink") = Some(channel);
        history.snapshot()
    }

    pub fn detach(&self) {
        *self.sink.lock().expect("pty sink") = None;
    }

    pub fn last_output_ms(&self) -> i64 {
        self.last_output.load(Ordering::Relaxed)
    }
}
