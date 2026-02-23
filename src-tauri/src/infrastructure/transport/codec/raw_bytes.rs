pub struct RawCodec;

impl RawCodec {
    /// Pasa los bytes directamente (agnóstico total)
    pub fn encode(data: Vec<u8>) -> Vec<u8> {
        data
    }

    /// Simplemente un alias para consistencia
    pub fn decode(data: Vec<u8>) -> Vec<u8> {
        data
    }
}