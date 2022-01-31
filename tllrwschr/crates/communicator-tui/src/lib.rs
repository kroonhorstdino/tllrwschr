use anyhow::{Error, Result};

use tui::Terminal;

use core::{
    periphery::communicator::{CommData, CommUnitConnection, Communicator, CommunicatorId},
    periphery::device::Device,
    periphery::messages::{
        comm::FromCommGeneral, master::FromCommMasterOrder, slave::FromCommSlaveEvent,
        FromCommMsgContent, FromUnitMsgContent,
    },
    periphery::{FromCommMsg, FromUnitMsg},
};

pub struct TuiCommunicator {
    data: CommData,
    tui: Terminal<tui::backend::CrosstermBackend<W>>,
}

//TODO: Impl this with tui!
impl Communicator for TuiCommunicator {
    fn with_communicator(id: CommunicatorId, conn: CommUnitConnection) -> Self {
        Self {
            data: CommData { id, conn },
        }
    }

    fn start(&self) -> Result<()> {
        //Send confirmation message
        self.data
            .conn
            .general
            .to_unit
            .try_send(FromCommMsg {
                device_id: 0,
                msg: FromCommGeneral::CommReady,
            })
            .map_err(Error::msg)
    }

    fn close(&mut self) -> Result<()> {
        todo!()
    }

    fn get_id(&self) -> CommunicatorId {
        todo!()
    }

    fn get_registered_devices<'a>() -> Option<&'a Vec<Device>> {
        todo!()
    }
}