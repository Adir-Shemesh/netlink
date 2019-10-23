use crate::{
    constants::*, CommandMessage, ConnectorMessage, ConnectorMessagePayload, ConnectorMsgHeader,
    CoreDumpMessage, DecodeError, ExecMessage, ExitMessage, Field, ForkMessage, GidChangeMessage,
    ParseableParametrized, ProcAckMessage, ProcConnectorEvent, ProcConnectorMessage, PtraceMessage,
    Rest, SidMessage, UidChangeMessage,
};

use byteorder::{ByteOrder, NativeEndian};

use failure::ResultExt;
use log::info;

pub const IDX: Field = 0..4; // u32
pub const VAL: Field = 4..8; // u32
pub const SEQ: Field = 8..12; // u32
pub const ACK: Field = 12..16; // u32
pub const LEN: Field = 16..18; // u16
pub const FLAGS: Field = 18..20; // u16
pub const DATA: Rest = 20..;

pub struct ConnectorBuffer<T> {
    buffer: T,
}

impl<T: AsRef<[u8]>> ConnectorBuffer<T> {
    pub fn new(buffer: T) -> ConnectorBuffer<T> {
        ConnectorBuffer { buffer }
    }

    pub fn length(&self) -> usize {
        self.buffer.as_ref().len()
    }

    pub fn new_checked(buffer: T) -> Result<ConnectorBuffer<T>, DecodeError> {
        Ok(Self::new(buffer))
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> ConnectorBuffer<&'a T> {
    pub fn inner(&self) -> &'a [u8] {
        &self.buffer.as_ref()[..]
    }
}

impl<'a, T: AsRef<[u8]> + AsMut<[u8]> + ?Sized> ConnectorBuffer<&'a mut T> {
    pub fn inner_mut(&mut self) -> &mut [u8] {
        &mut self.buffer.as_mut()[..]
    }
}

impl<'buffer, T: AsRef<[u8]> + ?Sized> ParseableParametrized<ConnectorMessage, u16>
    for ConnectorBuffer<&'buffer T>
{
    fn parse_with_param(&self, _message_type: u16) -> Result<ConnectorMessage, DecodeError> {
        let idx: u32 = NativeEndian::read_u32(&self.buffer.as_ref()[IDX]);
        let val: u32 = NativeEndian::read_u32(&self.buffer.as_ref()[VAL]);
        let seq: u32 = NativeEndian::read_u32(&self.buffer.as_ref()[SEQ]);
        let ack: u32 = NativeEndian::read_u32(&self.buffer.as_ref()[ACK]);
        let len: u16 = NativeEndian::read_u16(&self.buffer.as_ref()[LEN]);
        let flags: u16 = NativeEndian::read_u16(&self.buffer.as_ref()[FLAGS]);

        if idx != CN_IDX_PROC {
            return Ok(ConnectorMessage {
                header: ConnectorMsgHeader {
                    idx,
                    val,
                    seq,
                    ack,
                    len,
                    flags,
                },
                payload: ConnectorMessagePayload::Other,
            });
        }

        let data = self.buffer.as_ref()[DATA].to_vec().clone();

        let proc_event_type = NativeEndian::read_u32(&data[ProcConnectorMessage::WHAT]);
        info!("Got proc connector event: {:x}", proc_event_type);
        let cpu = NativeEndian::read_u32(&data[ProcConnectorMessage::CPU]);
        let timestamp = NativeEndian::read_u64(&data[ProcConnectorMessage::TIMESTAMP]);
        let payload = &data[ProcConnectorMessage::INNER_PAYLOAD];

        let proc_message = match proc_event_type {
            PROC_EVENT_NONE => ProcConnectorMessage::Event((
                cpu,
                timestamp,
                ProcConnectorEvent::Ack(<ProcAckMessage>::from(payload)),
            )),
            PROC_EVENT_FORK => ProcConnectorMessage::Event((
                cpu,
                timestamp,
                ProcConnectorEvent::Fork(<ForkMessage>::from(payload)),
            )),
            PROC_EVENT_EXEC => ProcConnectorMessage::Event((
                cpu,
                timestamp,
                ProcConnectorEvent::Exec(<ExecMessage>::from(payload)),
            )),
            PROC_EVENT_UID => ProcConnectorMessage::Event((
                cpu,
                timestamp,
                ProcConnectorEvent::UidChange(<UidChangeMessage>::from(payload)),
            )),
            PROC_EVENT_GID => ProcConnectorMessage::Event((
                cpu,
                timestamp,
                ProcConnectorEvent::GidChange(<GidChangeMessage>::from(payload)),
            )),
            PROC_EVENT_SID => ProcConnectorMessage::Event((
                cpu,
                timestamp,
                ProcConnectorEvent::Sid(<SidMessage>::from(payload)),
            )),
            PROC_EVENT_PTRACE => ProcConnectorMessage::Event((
                cpu,
                timestamp,
                ProcConnectorEvent::Ptrace(<PtraceMessage>::from(payload)),
            )),
            PROC_EVENT_COMM => ProcConnectorMessage::Event((
                cpu,
                timestamp,
                ProcConnectorEvent::Command(<CommandMessage>::from(payload)),
            )),
            PROC_EVENT_COREDUMP => ProcConnectorMessage::Event((
                cpu,
                timestamp,
                ProcConnectorEvent::CoreDump(<CoreDumpMessage>::from(payload)),
            )),
            PROC_EVENT_EXIT => ProcConnectorMessage::Event((
                cpu,
                timestamp,
                ProcConnectorEvent::Exit(<ExitMessage>::from(payload)),
            )),
            _ => return Err(format!("Unknown proc event: {}", proc_event_type).into()),
        };

        Ok(ConnectorMessage {
            header: ConnectorMsgHeader {
                idx,
                val,
                seq,
                ack,
                len,
                flags,
            },
            payload: ConnectorMessagePayload::ProcConnector(proc_message),
        })
    }
}
