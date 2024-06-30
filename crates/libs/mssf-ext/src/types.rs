use mssf_com::FabricTypes::{
    FABRIC_SERVICE_KIND, FABRIC_SERVICE_KIND_STATEFUL, FABRIC_SERVICE_KIND_STATELESS,
    FABRIC_SERVICE_QUERY_DESCRIPTION, FABRIC_URI,
};
use mssf_core::HSTRING;

pub struct ServiceQueryDescription {
    pub application_name: HSTRING,           // in url format
    pub servicename_filter: Option<HSTRING>, // in url format
}

impl ServiceQueryDescription {
    // raw type has lifetime the same as self
    pub fn get_raw(&self) -> FABRIC_SERVICE_QUERY_DESCRIPTION {
        FABRIC_SERVICE_QUERY_DESCRIPTION {
            ApplicationName: FABRIC_URI(self.application_name.as_ptr() as *mut u16),
            ServiceNameFilter: if self.servicename_filter.is_none() {
                FABRIC_URI(std::ptr::null_mut())
            } else {
                FABRIC_URI(self.servicename_filter.as_ref().unwrap().as_ptr() as *mut u16)
            },
            Reserved: std::ptr::null_mut(),
        }
    }
}

pub enum ServiceKind {
    Invalid,
    Stateless,
    Stateful,
}

impl From<&FABRIC_SERVICE_KIND> for ServiceKind {
    fn from(value: &FABRIC_SERVICE_KIND) -> Self {
        match *value {
            FABRIC_SERVICE_KIND_STATEFUL => ServiceKind::Stateful,
            FABRIC_SERVICE_KIND_STATELESS => ServiceKind::Stateless,
            _ => ServiceKind::Invalid,
        }
    }
}

//pub struct ServiceList(IFabricGetServiceListResult);

// TODO: need to export iter mod in core
// impl FabricListAccessor<FABRIC_SERVICE_QUERY_RESULT_ITEM> for ServiceList{

// }
