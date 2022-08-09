use super::{component::Component, scope::GlobalScope};

pub fn register_global<V>(container: String, name: String, v: V) -> &'static mut V
where
    V: Into<Component> + Send + Sync + 'static,
{
    GlobalScope::register_component::<V>(container, name, v, false)
}

pub fn get_global<V>(container: String, name: String) -> Option<&'static mut V>
where
    V: Into<Component> + Send + Sync + 'static,
{
    GlobalScope::get_component(container, name).map(|x| x.as_mut::<V>())
}

pub fn component_global(container: String, name: String) -> Option<&'static mut Component> {
    GlobalScope::get_component(container, name)
}
