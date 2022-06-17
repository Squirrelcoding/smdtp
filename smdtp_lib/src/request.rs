use bytes::BytesMut;

use crate::error::SMDTPRequestError;


/// The request itself, it consists of two components: the destination and the data. Think of the destination as an HTTP path.
#[derive(Debug, Clone)]
pub struct SMDTPRequest {
    destination: u8,
    data: BytesMut,
}



impl SMDTPRequest {

    /// Returns the destination of the request.
    pub fn destination(&self) -> u8 {
        self.destination
    }

    /// Returns the data of the request wrapped in a reference.
    pub fn data(&self) -> &BytesMut {
        &self.data
    }
}


/// Attempt to convert raw data in the form of BytesMut into an SMDTPRequest.
impl TryFrom<BytesMut> for SMDTPRequest {
    type Error = SMDTPRequestError;

    fn try_from(mut value: BytesMut) -> Result<Self, Self::Error> {
        // If the length of the buffer is too large, return an error.
        if value.len() > 16 {
            return Err(SMDTPRequestError::TooManyBytes);
        }

        // Split the data
        let destination = value[0];
        let data = value.split_off(1);

        Ok(Self { destination, data })
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;


    use crate::error::SMDTPRequestError;

    use super::SMDTPRequest;

    #[test]
    fn successful_parse() {
        let bytes = BytesMut::from(&[69, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15][..]);
        assert_eq!(bytes.len(), 16);

        let parsed_request = SMDTPRequest::try_from(bytes);

        assert!(parsed_request.is_ok());
        assert_eq!(parsed_request.as_ref().unwrap().destination(), 69);
        assert_eq!(parsed_request.as_ref().unwrap().data().len(), 15);
    }

    #[test]
    fn unsuccessful_too_many_bytes_parse() {
        let bytes = BytesMut::from(
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25,
            ][..],
        );
        let parsed_request = SMDTPRequest::try_from(bytes);
        assert!(parsed_request.is_err());
        assert!(matches!(
            parsed_request,
            Err(SMDTPRequestError::TooManyBytes)
        ))
    }
}
