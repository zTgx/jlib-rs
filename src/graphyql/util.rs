use std::rc::Rc;
use std::cell::Cell;
use std::any::Any;

use crate::graphyql::model::{ServerInfoResponse};

pub fn downcast_to_serverinfo(value: Rc<dyn Any>) -> ServerInfoResponse {
    let mut s = ServerInfoResponse::default();
    if let Ok(u) = value.downcast::<Cell<ServerInfoResponse>>() {
        s = u.take();
    }

    s
}
