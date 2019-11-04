use crate::packet::constants::{
    CN_IDX_PROC, CN_VAL_PROC, NLM_F_ACK, NLM_F_CREATE, NLM_F_DUMP, NLM_F_EXCL, NLM_F_NONREC,
    NLM_F_REQUEST, PROC_CN_MCAST_LISTEN,
};

use crate::packet::{
    ConnectorMessage, ConnectorMessagePayload, ConnectorMsgHeader, Emitable, NetlinkFlags,
    NetlinkMessage, NetlinkPayload, ProcConnectorMessage,
};

use failure::Fail;
use futures::{Future, Stream};
use netlink_proto::{ConnectionHandle, SocketAddr};
use std::process;

lazy_static! {
    static ref KERNEL_MCAST: SocketAddr = SocketAddr::new(0, 0);
}

use crate::{Error, ErrorKind};

/// A handle to the netlink connection, used to send and receive netlink messsage
#[derive(Clone, Debug)]
pub struct Handle(ConnectionHandle);

impl Handle {
    pub(crate) fn new(conn: ConnectionHandle) -> Self {
        Handle(conn)
    }

    /// Send a netlink message, and get the reponse as a stream of messages.
    pub fn request(
        &mut self,
        message: NetlinkMessage,
    ) -> impl Stream<Item = NetlinkMessage, Error = Error> {
        self.0
            .request(message, *KERNEL_MCAST)
            .map_err(|e| e.context(ErrorKind::RequestFailed).into())
    }

    /// Enable receiving proc connector events
    pub fn enable_events(&mut self) -> Result<(), Error> {
        let inner_payload = ProcConnectorMessage::ProcMcastListen;

        let msg = ConnectorMessage {
            header: ConnectorMsgHeader {
                idx: CN_IDX_PROC as u32,
                val: CN_VAL_PROC as u32,
                len: inner_payload.buffer_len() as u16,
                seq: 0,
                ack: 0,
                flags: 0,
            },
            payload: ConnectorMessagePayload::ProcConnector(inner_payload),
        };

        let mut req = NetlinkMessage::from(msg);

        req.header.sequence_number = 0;
        req.header.flags = NetlinkFlags::default();
        req.header.port_number = process::id();

        self.0
            .notify(req, *KERNEL_MCAST)
            .map_err(|e| e.context(ErrorKind::RequestFailed).into())
    }
}
