use crate::amf0::errors::Amf0ReadError;
use crate::protocol_control_messages::errors::ProtocolControlMessageReaderError;
use liverust_lib::netio::errors::IOReadError;
pub enum MessageErrorValue {
    IO(IOReadError),
    UnknowReadState,
    Amf0ReadError(Amf0ReadError),
    UnknowMessageType,
    ProtocolControlMessageReaderError(ProtocolControlMessageReaderError),
    //IO(io::Error),
}

pub struct MessageError {
    pub value: MessageErrorValue,
}

impl From<MessageErrorValue> for MessageError {
    fn from(val: MessageErrorValue) -> Self {
        MessageError { value: val }
    }
}

impl From<IOReadError> for MessageError {
    fn from(error: IOReadError) -> Self {
        MessageError {
            value: MessageErrorValue::IO(error),
        }
    }
}

impl From<Amf0ReadError> for MessageError {
    fn from(error: Amf0ReadError) -> Self {
        MessageError {
            value: MessageErrorValue::Amf0ReadError(error),
        }
    }
}

impl From<ProtocolControlMessageReaderError> for MessageError {
    fn from(error: ProtocolControlMessageReaderError) -> Self {
        MessageError {
            value: MessageErrorValue::ProtocolControlMessageReaderError(error),
        }
    }
}