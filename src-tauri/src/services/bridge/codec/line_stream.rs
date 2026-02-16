pub struct LineCodec;

impl LineCodec {
    /// Envuelve cualquier string en una línea para el transporte
    pub fn encode(data: &str) -> Vec<u8> {
        format!("{}\n", data).into_bytes()
    }

    /// Limpia el input del stream
    pub fn decode(data: &str) -> String {
        data.trim().to_string()
    }
}