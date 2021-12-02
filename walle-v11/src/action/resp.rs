use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resp {
    pub status: String,
    pub retcode: u16,
    pub data: RespContent,
    pub echo: String,
}

impl Resp {
    pub fn empty_404(echo: String) -> Self {
        Resp {
            status: "failed".to_string(),
            retcode: 1404,
            data: RespContent::empty(),
            echo,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum RespContent {
    Message {
        message_id: i32,
    },
    None(crate::utils::EmptyStruct),
    MessageDetail {
        time: i32,
        message_type: String,
        message_id: i32,
        real_id: i32,
        sender: crate::utils::Sender,
        message: Message,
    },
}

impl RespContent {
    pub fn empty() -> Self {
        Self::None(crate::utils::EmptyStruct {})
    }
}
