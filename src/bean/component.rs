use knife_util::crates::async_trait::async_trait;

use crate::web::request::HyperRequest;
use crate::web::response::HyperResponse;

/// 对象容器类型
///
/// 支持将struct转换为受不同生命周期管理的容器对象
#[async_trait]
pub trait ComponentTrait {}

/// 路由类型
///
/// 支持将fn转换为路由处理对象
#[async_trait]
pub trait RouterTrait {
    async fn router_handle(&self, req: HyperRequest) -> HyperResponse;
}

/// 全局容器模块
///
/// 类似于Spring的Ioc容器以存储不同类型的全局对象
pub enum Component {
    /// 对象容器类型
    COMPONENT(Box<dyn ComponentTrait + Send + Sync>),
    /// 路由类型
    ROUTER(Box<dyn RouterTrait + Send + Sync>),
}

impl Component {
    /// 返回Component容器类型
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

    /// 将Component容器类型转换为路由类型
    pub(crate) fn as_router(&mut self) -> &mut (dyn RouterTrait + Send + Sync) {
        match self {
            Component::ROUTER(r) => r.as_mut(),
            _ => panic!("不是Router对象"),
        }
    }
}
