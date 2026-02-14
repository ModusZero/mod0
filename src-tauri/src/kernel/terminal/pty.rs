use portable_pty::{native_pty_system, CommandBuilder, PtySize, MasterPty};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use crate::kernel::terminal::TerminalOutput;

const MAX_BUFFER_CHARS: usize = 20000;

pub struct PtyInstance {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    output_buffer: Arc<Mutex<VecDeque<char>>>,
    pub cwd: PathBuf,
}

impl PtyInstance {
    pub fn new(session_id: String, cwd: PathBuf, app_handle: AppHandle) -> Result<Self, String> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows: 24, cols: 80, pixel_width: 0, pixel_height: 0,
        }).map_err(|e| e.to_string())?;

        let shell = if cfg!(windows) { "powershell.exe" } else { "bash" };
        let mut cmd = CommandBuilder::new(shell);
        cmd.cwd(cwd.clone());

        pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
        
        let master = pair.master;
        let writer = master.take_writer().map_err(|e| e.to_string())?;
        let mut reader = master.try_clone_reader().map_err(|e| e.to_string())?;
        let output_buffer = Arc::new(Mutex::new(VecDeque::with_capacity(MAX_BUFFER_CHARS)));
        let output_buffer_clone = Arc::clone(&output_buffer);

        let sid = session_id.clone();
        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            while let Ok(n) = reader.read(&mut buffer) {
                if n == 0 { break; }
                let data = String::from_utf8_lossy(&buffer[..n]).to_string();
                
                if let Ok(mut buf) = output_buffer_clone.lock() {
                    for c in data.chars() {
                        if buf.len() >= MAX_BUFFER_CHARS { buf.pop_front(); }
                        buf.push_back(c);
                    }
                }

                let _ = app_handle.emit("terminal:stdout", TerminalOutput {
                    session_id: sid.clone(),
                    data,
                });
            }
        });

        Ok(Self { master, writer, output_buffer, cwd })
    }

    pub fn read_buffer(&self) -> String {
        self.output_buffer.lock().map(|buf| buf.iter().collect()).unwrap_or_default()
    }

    pub fn write(&mut self, data: &str) -> Result<(), String> {
        self.writer.write_all(data.as_bytes()).map_err(|e| e.to_string())
    }

    pub fn resize(&mut self, rows: u16, cols: u16) -> Result<(), String> {
        self.master.resize(PtySize {
            rows, cols, pixel_width: 0, pixel_height: 0,
        }).map_err(|e| e.to_string())
    }
}