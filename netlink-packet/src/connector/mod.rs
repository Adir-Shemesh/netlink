use crate::constants::*;
use crate::{Emitable, Field, Rest};

mod buffer;
pub use self::buffer::*;

use byteorder::{ByteOrder, NativeEndian};

// ------- NoneMessage ------- //

#[derive(Debug, Eq, Clone, PartialEq, Default)]
pub struct ProcAckMessage {
    pub err: u32,
}

impl From<&[u8]> for ProcAckMessage {
    fn from(buffer: &[u8]) -> ProcAckMessage {
        let err = NativeEndian::read_u32(&buffer);
        ProcAckMessage { err }
    }
}

impl Emitable for ProcAckMessage {
    fn buffer_len(&self) -> usize {
        std::mem::size_of::<ProcAckMessage>()
    }

    fn emit(&self, mut buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer, self.err);
    }
}

// ------- ForkMessage ------- //

#[derive(Debug, Eq, Clone, PartialEq, Default)]
pub struct ForkMessage {
    pub parent_pid: u32,
    pub parent_tgid: u32,
    pub child_pid: u32,
    pub child_tgid: u32,
}

impl ForkMessage {
    const PARENT_PID: Field = 0..4;
    const PARENT_TGID: Field = 4..8;
    const CHILD_PID: Field = 8..12;
    const CHILD_TGID: Field = 12..16;
}

impl From<&[u8]> for ForkMessage {
    fn from(buffer: &[u8]) -> ForkMessage {
        let parent_pid = NativeEndian::read_u32(&buffer[Self::PARENT_PID]);
        let parent_tgid = NativeEndian::read_u32(&buffer[Self::PARENT_TGID]);
        let child_pid = NativeEndian::read_u32(&buffer[Self::CHILD_PID]);
        let child_tgid = NativeEndian::read_u32(&buffer[Self::CHILD_TGID]);

        ForkMessage {
            parent_pid,
            parent_tgid,
            child_pid,
            child_tgid,
        }
    }
}

impl Emitable for ForkMessage {
    fn buffer_len(&self) -> usize {
        std::mem::size_of::<ForkMessage>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer[Self::PARENT_PID], self.parent_pid);
        NativeEndian::write_u32(&mut buffer[Self::PARENT_TGID], self.parent_tgid);
        NativeEndian::write_u32(&mut buffer[Self::CHILD_PID], self.child_pid);
        NativeEndian::write_u32(&mut buffer[Self::CHILD_TGID], self.child_tgid);
    }
}

// ------- ExecMessage ------- //

#[derive(Debug, Eq, Clone, PartialEq, Default)]
pub struct ExecMessage {
    pub pid: u32,
    pub tgid: u32,
}

impl ExecMessage {
    const PID: Field = 0..4;
    const TGID: Field = 4..8;
}

impl From<&[u8]> for ExecMessage {
    fn from(buffer: &[u8]) -> ExecMessage {
        let pid = NativeEndian::read_u32(&buffer[Self::PID]);
        let tgid = NativeEndian::read_u32(&buffer[Self::TGID]);
        ExecMessage { pid, tgid }
    }
}

impl Emitable for ExecMessage {
    fn buffer_len(&self) -> usize {
        std::mem::size_of::<ExecMessage>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer[Self::PID], self.pid);
        NativeEndian::write_u32(&mut buffer[Self::TGID], self.tgid);
    }
}

// -------- UidChangeMessage --------- //

#[derive(Debug, Eq, Clone, PartialEq, Default)]
pub struct UidChangeMessage {
    pub process_pid: u32,
    pub process_tgid: u32,
    pub uid: u32,
    pub euid: u32,
}

impl UidChangeMessage {
    const PROCESS_PID: Field = 0..4;
    const PROCESS_TGID: Field = 4..8;
    const UID: Field = 8..12;
    const EUID: Field = 12..16;
}

impl From<&[u8]> for UidChangeMessage {
    fn from(buffer: &[u8]) -> UidChangeMessage {
        let process_pid = NativeEndian::read_u32(&buffer[Self::PROCESS_PID]);
        let process_tgid = NativeEndian::read_u32(&buffer[Self::PROCESS_TGID]);
        let uid = NativeEndian::read_u32(&buffer[Self::UID]);;
        let euid = NativeEndian::read_u32(&buffer[Self::EUID]);;

        UidChangeMessage {
            process_pid,
            process_tgid,
            uid,
            euid,
        }
    }
}

impl Emitable for UidChangeMessage {
    fn buffer_len(&self) -> usize {
        std::mem::size_of::<UidChangeMessage>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer[Self::PROCESS_PID], self.process_pid);
        NativeEndian::write_u32(&mut buffer[Self::PROCESS_TGID], self.process_tgid);
        NativeEndian::write_u32(&mut buffer[Self::UID], self.uid);
        NativeEndian::write_u32(&mut buffer[Self::EUID], self.euid);
    }
}

// -------- GidChangeMessage --------- //

#[derive(Debug, Eq, Clone, PartialEq, Default)]
pub struct GidChangeMessage {
    pub process_pid: u32,
    pub process_tgid: u32,
    pub gid: u32,
    pub egid: u32,
}

impl GidChangeMessage {
    const PROCESS_PID: Field = 0..4;
    const PROCESS_TGID: Field = 4..8;
    const GID: Field = 8..12;
    const EGID: Field = 12..16;
}

impl From<&[u8]> for GidChangeMessage {
    fn from(buffer: &[u8]) -> GidChangeMessage {
        let process_pid = NativeEndian::read_u32(&buffer[Self::PROCESS_PID]);
        let process_tgid = NativeEndian::read_u32(&buffer[Self::PROCESS_TGID]);
        let gid = NativeEndian::read_u32(&buffer[Self::GID]);;
        let egid = NativeEndian::read_u32(&buffer[Self::EGID]);;

        GidChangeMessage {
            process_pid,
            process_tgid,
            gid,
            egid,
        }
    }
}

impl Emitable for GidChangeMessage {
    fn buffer_len(&self) -> usize {
        std::mem::size_of::<GidChangeMessage>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer[Self::PROCESS_PID], self.process_pid);
        NativeEndian::write_u32(&mut buffer[Self::PROCESS_TGID], self.process_tgid);
        NativeEndian::write_u32(&mut buffer[Self::GID], self.gid);
        NativeEndian::write_u32(&mut buffer[Self::EGID], self.egid);
    }
}

// -------- SidMessage --------- //

#[derive(Debug, Eq, Clone, PartialEq, Default)]
pub struct SidMessage {
    pub parent_pid: u32,
    pub parent_tgid: u32,
}

impl SidMessage {
    const PARENT_PID: Field = 0..4;
    const PARENT_TGID: Field = 4..8;
}

impl From<&[u8]> for SidMessage {
    fn from(buffer: &[u8]) -> SidMessage {
        let parent_pid = NativeEndian::read_u32(&buffer[Self::PARENT_PID]);
        let parent_tgid = NativeEndian::read_u32(&buffer[Self::PARENT_TGID]);

        SidMessage {
            parent_pid,
            parent_tgid,
        }
    }
}

impl Emitable for SidMessage {
    fn buffer_len(&self) -> usize {
        std::mem::size_of::<SidMessage>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer[Self::PARENT_PID], self.parent_pid);
        NativeEndian::write_u32(&mut buffer[Self::PARENT_TGID], self.parent_tgid);
    }
}

// -------- PtraceMessage --------- //

#[derive(Debug, Eq, Clone, PartialEq, Default)]
pub struct PtraceMessage {
    pub parent_pid: u32,
    pub parent_tgid: u32,
    pub tracer_pid: u32,
    pub tracer_tgid: u32,
}

impl PtraceMessage {
    pub const PARENT_PID: Field = 0..4;
    pub const PARENT_TGID: Field = 4..8;
    pub const TRACER_PID: Field = 8..12;
    pub const TRACER_TGID: Field = 12..16;
}

impl From<&[u8]> for PtraceMessage {
    fn from(buffer: &[u8]) -> PtraceMessage {
        let parent_pid = NativeEndian::read_u32(&buffer[Self::PARENT_PID]);
        let parent_tgid = NativeEndian::read_u32(&buffer[Self::PARENT_TGID]);
        let tracer_pid = NativeEndian::read_u32(&buffer[Self::TRACER_PID]);
        let tracer_tgid = NativeEndian::read_u32(&buffer[Self::TRACER_TGID]);

        PtraceMessage {
            parent_pid,
            parent_tgid,
            tracer_pid,
            tracer_tgid,
        }
    }
}

impl Emitable for PtraceMessage {
    fn buffer_len(&self) -> usize {
        std::mem::size_of::<PtraceMessage>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer[Self::PARENT_PID], self.parent_pid);
        NativeEndian::write_u32(&mut buffer[Self::PARENT_TGID], self.parent_tgid);
        NativeEndian::write_u32(&mut buffer[Self::TRACER_PID], self.tracer_pid);
        NativeEndian::write_u32(&mut buffer[Self::TRACER_TGID], self.tracer_tgid);
    }
}

// -------- CommandMessage --------- //

#[derive(Debug, Eq, Clone, PartialEq, Default)]
pub struct CommandMessage {
    pub parent_pid: u32,
    pub parent_tgid: u32,
    pub comm: [u8; 16],
}

impl CommandMessage {
    pub const PARENT_PID: Field = 0..4;
    pub const PARENT_TGID: Field = 4..8;
    pub const COMM: Rest = 8..;
}

impl From<&[u8]> for CommandMessage {
    fn from(buffer: &[u8]) -> CommandMessage {
        let parent_pid = NativeEndian::read_u32(&buffer[Self::PARENT_PID]);
        let parent_tgid = NativeEndian::read_u32(&buffer[Self::PARENT_TGID]);
        let mut comm: [u8; 16] = [0; 16];

        comm.copy_from_slice(&buffer[Self::COMM]);

        CommandMessage {
            parent_pid,
            parent_tgid,
            comm,
        }
    }
}

impl Emitable for CommandMessage {
    fn buffer_len(&self) -> usize {
        std::mem::size_of::<CommandMessage>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer[Self::PARENT_PID], self.parent_pid);
        NativeEndian::write_u32(&mut buffer[Self::PARENT_TGID], self.parent_tgid);
        buffer[Self::COMM].copy_from_slice(&self.comm);
    }
}

// -------- CoreDumpMessage --------- //

#[derive(Debug, Eq, Clone, PartialEq, Default)]
pub struct CoreDumpMessage {
    pub process_pid: u32,
    pub process_tgid: u32,
    pub parent_pid: u32,
    pub parent_tgid: u32,
}

impl CoreDumpMessage {
    pub const PROCESS_PID: Field = 0..4;
    pub const PROCESS_TGID: Field = 4..8;
    pub const PARENT_PID: Field = 8..12;
    pub const PARENT_TGID: Field = 12..16;
}

impl From<&[u8]> for CoreDumpMessage {
    fn from(buffer: &[u8]) -> CoreDumpMessage {
        let process_pid = NativeEndian::read_u32(&buffer[Self::PROCESS_PID]);
        let process_tgid = NativeEndian::read_u32(&buffer[Self::PROCESS_TGID]);
        let parent_pid = NativeEndian::read_u32(&buffer[Self::PARENT_PID]);
        let parent_tgid = NativeEndian::read_u32(&buffer[Self::PARENT_TGID]);

        CoreDumpMessage {
            process_pid,
            process_tgid,
            parent_pid,
            parent_tgid,
        }
    }
}

impl Emitable for CoreDumpMessage {
    fn buffer_len(&self) -> usize {
        std::mem::size_of::<CoreDumpMessage>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer[Self::PROCESS_PID], self.process_pid);
        NativeEndian::write_u32(&mut buffer[Self::PROCESS_TGID], self.process_tgid);
        NativeEndian::write_u32(&mut buffer[Self::PARENT_PID], self.parent_pid);
        NativeEndian::write_u32(&mut buffer[Self::PARENT_TGID], self.parent_tgid);
    }
}

// -------- ExitMessage --------- //

#[derive(Debug, Eq, Clone, PartialEq, Default)]
pub struct ExitMessage {
    pub process_pid: u32,
    pub process_tgid: u32,
    pub exit_code: u32,
    pub exit_signal: u32,
    pub parent_pid: u32,
    pub parent_tgid: u32,
}

impl ExitMessage {
    pub const PROCESS_PID: Field = 0..4;
    pub const PROCESS_TGID: Field = 4..8;
    pub const EXIT_CODE: Field = 8..12;
    pub const EXIT_SIGNAL: Field = 12..16;
    pub const PARENT_PID: Field = 16..20;
    pub const PARENT_TGID: Field = 20..24;
}

impl From<&[u8]> for ExitMessage {
    fn from(buffer: &[u8]) -> ExitMessage {
        let process_pid = NativeEndian::read_u32(&buffer[Self::PROCESS_PID]);
        let process_tgid = NativeEndian::read_u32(&buffer[Self::PROCESS_TGID]);
        let exit_code = NativeEndian::read_u32(&buffer[Self::EXIT_CODE]);
        let exit_signal = NativeEndian::read_u32(&buffer[Self::EXIT_SIGNAL]);
        let parent_pid = NativeEndian::read_u32(&buffer[Self::PARENT_PID]);
        let parent_tgid = NativeEndian::read_u32(&buffer[Self::PARENT_TGID]);

        ExitMessage {
            process_pid,
            process_tgid,
            exit_code,
            exit_signal,
            parent_pid,
            parent_tgid,
        }
    }
}

impl Emitable for ExitMessage {
    fn buffer_len(&self) -> usize {
        std::mem::size_of::<ExitMessage>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer[Self::PROCESS_PID], self.process_pid);
        NativeEndian::write_u32(&mut buffer[Self::PROCESS_TGID], self.process_tgid);
        NativeEndian::write_u32(&mut buffer[Self::EXIT_CODE], self.exit_code);
        NativeEndian::write_u32(&mut buffer[Self::EXIT_SIGNAL], self.exit_signal);
        NativeEndian::write_u32(&mut buffer[Self::PARENT_PID], self.parent_pid);
        NativeEndian::write_u32(&mut buffer[Self::PARENT_TGID], self.parent_tgid);
    }
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum ProcConnectorEvent {
    Ack(ProcAckMessage),
    Fork(ForkMessage),
    Exec(ExecMessage),
    UidChange(UidChangeMessage),
    GidChange(GidChangeMessage),
    Sid(SidMessage),
    Ptrace(PtraceMessage),
    Command(CommandMessage),
    CoreDump(CoreDumpMessage),
    Exit(ExitMessage),
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum ProcConnectorMessage {
    ProcMcastListen,
    ProcMcastIgnore,
    Event((u32, u64, ProcConnectorEvent)),
}

impl ProcConnectorMessage {
    // Associated consts for event parsing
    pub const WHAT: Field = 0..4; // u32
    pub const CPU: Field = 4..8; // u32
    pub const TIMESTAMP: Field = 8..16; // u64
    pub const INNER_PAYLOAD: Rest = 16..; // Rest
}

impl Emitable for ProcConnectorMessage {
    fn buffer_len(&self) -> usize {
        use self::ProcConnectorMessage::*;

        match self {
            ProcMcastListen | ProcMcastIgnore => std::mem::size_of::<u32>(), // sizeof(enum proc_cn_mcast_op)
            // TODO: Change for specific event maybe? also the addition of cpu and timestamp is horrible.
            Event((_, _, event)) => {
                event.buffer_len() // Actual event struct
                    + std::mem::size_of::<u32>() // cpu
                    + std::mem::size_of::<u64>() // Timestamp
            }
        }
    }

    fn emit(&self, buffer: &mut [u8]) {
        use self::ProcConnectorMessage::*;

        match self {
            ProcMcastListen => NativeEndian::write_u32(&mut buffer[DATA], PROC_CN_MCAST_LISTEN),
            ProcMcastIgnore => NativeEndian::write_u32(&mut buffer[DATA], PROC_CN_MCAST_IGNORE),
            Event((cpu, timestamp, ref msg)) => msg.emit(buffer), // TODO: emit cpu and timestamp as well?
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ConnectorMsgHeader {
    pub idx: u32,
    pub val: u32,
    pub seq: u32,
    pub ack: u32,
    pub len: u16,
    pub flags: u16,
}

impl Emitable for ConnectorMsgHeader {
    fn buffer_len(&self) -> usize {
        // Fixed size, should be 20 bytes
        std::mem::size_of::<ConnectorMsgHeader>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        NativeEndian::write_u32(&mut buffer[IDX], self.idx);
        NativeEndian::write_u32(&mut buffer[VAL], self.val);
        NativeEndian::write_u32(&mut buffer[SEQ], self.seq);
        NativeEndian::write_u32(&mut buffer[ACK], self.ack);
        NativeEndian::write_u16(&mut buffer[LEN], self.len);
        NativeEndian::write_u16(&mut buffer[FLAGS], self.flags);
    }
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct ConnectorMessage {
    pub header: ConnectorMsgHeader,
    pub payload: ConnectorMessagePayload,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum ConnectorMessagePayload {
    ProcConnector(ProcConnectorMessage),
    Other,
}

impl ConnectorMessage {
    pub fn message_type(&self) -> u16 {
        use self::ConnectorMessagePayload::*;

        match self.payload {
            ProcConnector(_) => NLMSG_DONE,
            Other => NLMSG_DONE, // TODO: ???
        }
    }
}

impl Emitable for ConnectorMessage {
    fn buffer_len(&self) -> usize {
        use self::ConnectorMessagePayload::*;

        match self.payload {
            ProcConnector(ref msg) => msg.buffer_len() + self.header.buffer_len(),
            Other => self.header.buffer_len(), // We don't handle other connector protocols at the moment
                                               // so we don't know what payload they hold
        }
    }

    fn emit(&self, buffer: &mut [u8]) {
        use self::ConnectorMessagePayload::*;

        // Write connector header to buffer
        self.header.emit(buffer);

        // Write connector payload to buffer
        match self.payload {
            ProcConnector(ref msg) => msg.emit(buffer),
            _ => (),
        }
    }
}

impl Emitable for ProcConnectorEvent {
    fn buffer_len(&self) -> usize {
        use self::ProcConnectorEvent::*;

        match self {
            Ack(ref msg) => msg.buffer_len(),
            Fork(ref msg) => msg.buffer_len(),
            Exec(ref msg) => msg.buffer_len(),
            UidChange(ref msg) => msg.buffer_len(),
            GidChange(ref msg) => msg.buffer_len(),
            Sid(ref msg) => msg.buffer_len(),
            Ptrace(ref msg) => msg.buffer_len(),
            Command(ref msg) => msg.buffer_len(),
            CoreDump(ref msg) => msg.buffer_len(),
            Exit(ref msg) => msg.buffer_len(),
        }
    }

    fn emit(&self, buffer: &mut [u8]) {
        use self::ProcConnectorEvent::*;

        match self {
            Ack(ref msg) => msg.emit(buffer),
            Fork(ref msg) => msg.emit(buffer),
            Exec(ref msg) => msg.emit(buffer),
            UidChange(ref msg) => msg.emit(buffer),
            GidChange(ref msg) => msg.emit(buffer),
            Sid(ref msg) => msg.emit(buffer),
            Ptrace(ref msg) => msg.emit(buffer),
            Command(ref msg) => msg.emit(buffer),
            CoreDump(ref msg) => msg.emit(buffer),
            Exit(ref msg) => msg.emit(buffer),
        }
    }
}
