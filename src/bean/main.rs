use super::{component::Component, scope::GlobalScope};

/// 注册全局容器对象
pub fn register_global<V>(container: String, name: String, v: V) -> &'static mut V
where
    V: Into<Component> + Send + Sync + 'static,
{
    GlobalScope::register_component::<V>(container, name, v, false)
}

/// 获取全局容器对象
pub fn get_global<V>(container: String, name: String) -> Option<&'static mut V>
where
    V: Into<Component> + Send + Sync + 'static,
{
    GlobalScope::get_component(container, name).map(|x| x.as_mut::<V>())
}

/// 获取全局容器Component对象
pub fn component_global(container: String, name: String) -> Option<&'static mut Component> {
    GlobalScope::get_component(container, name)
}

/// 遍历全局容器对象
pub fn foreach_global<F>(container: String, f: F)
where
    F: FnMut((String, &'static mut Component)) + Send + Sync,
{
    GlobalScope::foreach_component(container, f)
}
