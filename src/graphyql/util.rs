use std::rc::Rc;
use std::cell::Cell;
use std::any::Any;

use crate::graphyql::model::*;

pub fn downcast_to_serverinfo(value: Rc<dyn Any>) -> ServerInfoResponse {
    let mut s = ServerInfoResponse::default();
    if let Ok(u) = value.downcast::<Cell<ServerInfoResponse>>() {
        s = u.take();
    }

    s
}

pub fn downcast_to_bool(value: Rc<dyn Any>) -> bool {
    let mut s = false;
    if let Ok(u) = value.downcast::<Cell<bool>>() {
        s = u.take();
    }

    s
}

pub fn downcast_to_paymentinfo(value: Rc<dyn Any>) -> PaymentInfo {
    let mut s = PaymentInfo::default();
    if let Ok(u) = value.downcast::<Cell<PaymentInfo>>() {
        s = u.take();
    }

    s
}

pub fn downcast_to_ledgerinfo(value: Rc<dyn Any>) -> LedgerInfo {
    let mut s = LedgerInfo::default();
    if let Ok(u) = value.downcast::<Cell<LedgerInfo>>() {
        s = u.take();
    }

    s
}
