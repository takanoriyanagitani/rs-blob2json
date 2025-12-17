use std::collections::HashMap;
use std::io;

use io::BufRead;
use io::Read;

use base64::Engine;

#[derive(serde::Serialize)]
pub struct Blob {
    /// The name of the blob. e.g., "blob.dat"
    pub name: String,

    pub content_type: String,

    pub content_encoding: String,

    /// (Always base64).
    pub content_transfer_encoding: String,

    /// Base64 encoded content(even if the content is human readable text).
    pub body: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

pub struct BlobBuilder {
    content_type: String,
    content_encoding: String,
    max_bytes: u64,
    metadata: Option<HashMap<String, String>>,
}

impl BlobBuilder {
    pub fn bytes2blob(&self, blob: &[u8], name: String) -> Blob {
        let encoded: String = base64::engine::general_purpose::STANDARD.encode(blob);
        Blob {
            name,
            content_type: self.content_type.clone(),
            content_encoding: self.content_encoding.clone(),
            content_transfer_encoding: "base64".into(),
            body: encoded,
            metadata: self.metadata.clone(),
        }
    }

    pub fn rdr2blob<R>(&self, rdr: R, name: String) -> Result<Blob, io::Error>
    where
        R: BufRead,
    {
        let mut taken = rdr.take(self.max_bytes);
        let mut buf: Vec<u8> = vec![];
        taken.read_to_end(&mut buf)?;
        Ok(self.bytes2blob(&buf, name))
    }
}

pub const CONTENT_TYPE_DEFAULT: &str = "application/octet-stream";
pub const CONTENT_ENCODING_DEFAULT: &str = "";
pub const MAX_BYTES_DEFAULT: u64 = 1048576;

impl Default for BlobBuilder {
    fn default() -> Self {
        Self {
            content_type: CONTENT_TYPE_DEFAULT.into(),
            content_encoding: CONTENT_ENCODING_DEFAULT.into(),
            max_bytes: MAX_BYTES_DEFAULT,
            metadata: None,
        }
    }
}

impl BlobBuilder {
    pub fn with_content_type(mut self, ctyp: String) -> Self {
        self.content_type = ctyp;
        self
    }

    pub fn with_content_encoding(mut self, enc: String) -> Self {
        self.content_encoding = enc;
        self
    }

    pub fn with_max_bytes(mut self, mb: u64) -> Self {
        self.max_bytes = mb;
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}
