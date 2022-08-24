use knife_util::crates::async_trait::async_trait;

use crate::web::request::HyperRequest;
use crate::web::response::HyperResponse;

#[async_trait]
pub trait ComponentTrait {}

#[async_trait]
pub trait RouterTrait {
    async fn router_handle(&self, req: HyperRequest) -> HyperResponse;
}

pub enum Component {
    COMPONENT(Box<dyn ComponentTrait + Send + Sync>),
    ROUTER(Box<dyn RouterTrait + Send + Sync>),
}

impl Component {
    pub(crate) fn as_mut<V>(&mut self) -> &mut V
    where
        V: Send + Sync,
    {
        match self {
            Component::COMPONENT(r) => unsafe {
                &mut *(r.as_mut() as *mut dyn ComponentTrait as *mut V)
            },
            Component::ROUTER(r) => unsafe { &mut *(r.as_mut() as *mut dyn RouterTrait as *mut V) },
        }
    }

    pub(crate) fn to_router(&mut self) -> &mut (dyn RouterTrait + Send + Sync) {
        match self {
            Component::ROUTER(r) => r.as_mut(),
            _ => panic!("不是Router对象"),
        }
    }
}
