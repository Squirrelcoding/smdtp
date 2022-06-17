// Empty for now

use bytes::{BufMut, BytesMut};

use crate::error::{SMDTPRequestError};

#[derive(Debug, Clone)]
pub struct SMDTPResponse {
    pub status: u8,
    pub data: BytesMut,
}

impl SMDTPResponse {
    pub fn as_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::with_capacity(16);
        bytes.put_u8(self.status);
        bytes.put(&self.data[..]);
        bytes
    }
}

/// Attempt to convert raw data in the form of BytesMut into an SMDTPResponse.
impl TryFrom<BytesMut> for SMDTPResponse {
    type Error = SMDTPRequestError;

    fn try_from(mut value: BytesMut) -> Result<Self, Self::Error> {
        // If the length of the buffer is too large, return an error.
        if value.len() > 16 {
            return Err(SMDTPRequestError::TooManyBytes);
        }

        // Split the data
        let status = value[0];
        let data = value.split_off(1);

        Ok(Self { data, status })
    }
}

/// Attempt to convert raw data in the form of BytesMut into an SMDTPResponse.
impl From<SMDTPRequestError> for SMDTPResponse {
    fn from(error_type: SMDTPRequestError) -> Self {
        let data = BytesMut::with_capacity(16);
        match error_type {
            SMDTPRequestError::UnmatchedByte => Self { status: 11, data },
            SMDTPRequestError::TooManyBytes => Self { status: 12, data },
            SMDTPRequestError::Forbidden => Self { status: 13, data },
            SMDTPRequestError::InternalServerError => Self { status: 14, data },
        }
    }
}
