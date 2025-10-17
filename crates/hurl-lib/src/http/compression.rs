use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionCodec {
    Gzip,
    Deflate,
    Brotli,
}

impl CompressionCodec {
    pub fn as_header_value(&self) -> &'static str {
        match self {
            CompressionCodec::Gzip => "gzip",
            CompressionCodec::Deflate => "deflate",
            CompressionCodec::Brotli => "br",
        }
    }

    pub fn from_header_value(value: &str) -> Option<Self> {
        match value.trim().to_lowercase().as_str() {
            "gzip" => Some(CompressionCodec::Gzip),
            "deflate" => Some(CompressionCodec::Deflate),
            "br" => Some(CompressionCodec::Brotli),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub codecs: Vec<CompressionCodec>,
    pub min_size_bytes: usize,
}

impl CompressionConfig {
    pub fn new() -> Self {
        CompressionConfig {
            enabled: true,
            codecs: vec![CompressionCodec::Gzip],
            min_size_bytes: 1024,
        }
    }

    pub fn with_codecs(mut self, codecs: Vec<CompressionCodec>) -> Self {
        self.codecs = codecs;
        self
    }

    pub fn with_min_size(mut self, size: usize) -> Self {
        self.min_size_bytes = size;
        self
    }

    pub fn disabled() -> Self {
        CompressionConfig {
            enabled: false,
            ..Self::new()
        }
    }

    pub fn accept_encoding_header(&self) -> String {
        self.codecs
            .iter()
            .map(|c| c.as_header_value())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Decompressor;

impl Decompressor {
    pub fn decompress(data: &[u8], codec: CompressionCodec) -> crate::Result<Vec<u8>> {
        match codec {
            CompressionCodec::Gzip => Self::decompress_gzip(data),
            CompressionCodec::Deflate => Self::decompress_deflate(data),
            CompressionCodec::Brotli => Self::decompress_brotli(data),
        }
    }

    fn decompress_gzip(data: &[u8]) -> crate::Result<Vec<u8>> {
        use flate2::read::GzDecoder;

        let mut decoder = GzDecoder::new(data);
        let mut result = Vec::new();
        decoder
            .read_to_end(&mut result)
            .map_err(|e| crate::Error::Http(format!("gzip decompression failed: {}", e)))?;
        Ok(result)
    }

    fn decompress_deflate(data: &[u8]) -> crate::Result<Vec<u8>> {
        use flate2::read::DeflateDecoder;

        let mut decoder = DeflateDecoder::new(data);
        let mut result = Vec::new();
        decoder
            .read_to_end(&mut result)
            .map_err(|e| crate::Error::Http(format!("deflate decompression failed: {}", e)))?;
        Ok(result)
    }

    fn decompress_brotli(data: &[u8]) -> crate::Result<Vec<u8>> {
        let mut result = Vec::new();
        let mut decompressor = brotli::Decompressor::new(data, 4096);
        decompressor
            .read_to_end(&mut result)
            .map_err(|e| crate::Error::Http(format!("brotli decompression failed: {}", e)))?;
        Ok(result)
    }

    pub fn auto_decompress(data: &[u8], content_encoding: Option<&str>) -> crate::Result<Vec<u8>> {
        match content_encoding {
            Some(encoding) => {
                if let Some(codec) = CompressionCodec::from_header_value(encoding) {
                    Self::decompress(data, codec)
                } else {
                    Ok(data.to_vec())
                }
            }
            None => Ok(data.to_vec()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_codec_gzip() {
        assert_eq!(CompressionCodec::Gzip.as_header_value(), "gzip");
        assert_eq!(
            CompressionCodec::from_header_value("gzip"),
            Some(CompressionCodec::Gzip)
        );
    }

    #[test]
    fn test_compression_codec_deflate() {
        assert_eq!(CompressionCodec::Deflate.as_header_value(), "deflate");
        assert_eq!(
            CompressionCodec::from_header_value("deflate"),
            Some(CompressionCodec::Deflate)
        );
    }

    #[test]
    fn test_compression_codec_brotli() {
        assert_eq!(CompressionCodec::Brotli.as_header_value(), "br");
        assert_eq!(
            CompressionCodec::from_header_value("br"),
            Some(CompressionCodec::Brotli)
        );
    }

    #[test]
    fn test_compression_config_default() {
        let config = CompressionConfig::new();
        assert!(config.enabled);
        assert_eq!(config.codecs.len(), 1);
        assert_eq!(config.min_size_bytes, 1024);
    }

    #[test]
    fn test_compression_config_disabled() {
        let config = CompressionConfig::disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_accept_encoding_header() {
        let config = CompressionConfig::new()
            .with_codecs(vec![CompressionCodec::Gzip, CompressionCodec::Brotli]);
        let header = config.accept_encoding_header();
        assert!(header.contains("gzip"));
        assert!(header.contains("br"));
    }

    #[test]
    fn test_compression_codec_case_insensitive() {
        assert_eq!(
            CompressionCodec::from_header_value("GZIP"),
            Some(CompressionCodec::Gzip)
        );
        assert_eq!(
            CompressionCodec::from_header_value("GzIp"),
            Some(CompressionCodec::Gzip)
        );
    }

    #[test]
    fn test_compression_config_with_codecs() {
        let config = CompressionConfig::new()
            .with_codecs(vec![CompressionCodec::Deflate, CompressionCodec::Brotli]);
        assert_eq!(config.codecs.len(), 2);
        assert_eq!(config.codecs[0], CompressionCodec::Deflate);
        assert_eq!(config.codecs[1], CompressionCodec::Brotli);
    }

    #[test]
    fn test_compression_config_with_min_size() {
        let config = CompressionConfig::new().with_min_size(2048);
        assert_eq!(config.min_size_bytes, 2048);
    }

    #[test]
    fn test_unknown_codec() {
        assert_eq!(CompressionCodec::from_header_value("unknown"), None);
    }
}
