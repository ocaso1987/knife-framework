use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tracing::debug;

use super::component::Component;

pub(crate) struct Container {
    pub(crate) _container_name: String,
    pub(crate) component_map: HashMap<String, Component>,
}

lazy_static! {
    static ref GLOBAL_SCOPE: Arc<Mutex<HashMap<String, Container>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub(crate) struct GlobalScope {}

impl GlobalScope {
    pub(crate) fn get_container(container_name: String) -> &'static mut Container {
        let mut container_map = GLOBAL_SCOPE.lock().unwrap();
        if !container_map.contains_key(container_name.as_str()) {
            container_map.insert(
                container_name.to_string(),
                Container {
                    _container_name: container_name.to_string(),
                    component_map: HashMap::new(),
                },
            );
        }
        let container = container_map.get_mut(container_name.as_str()).unwrap();
        unsafe { &mut *(container as *mut Container) }
    }

    pub(crate) fn get_component(
        container_name: String,
        name: String,
    ) -> Option<&'static mut Component> {
        let container = Self::get_container(container_name);
        let component_map = &mut container.component_map;
        if !component_map.contains_key(name.as_str()) {
            return None;
        }
        component_map.get_mut(name.as_str())
    }

    pub(crate) fn register_component<V>(
        container_name: String,
        name: String,
        v: V,
        replaceable: bool,
    ) -> &'static mut V
    where
        V: Into<Component> + Send + Sync + 'static,
    {
        let container = Self::get_container(container_name.clone());
        let component_map = &mut container.component_map;
        if !component_map.contains_key(name.as_str()) || replaceable {
            component_map.insert(name.to_string(), v.into());
            debug!(
                "设置Component对象:container={},name={}",
                container_name.clone(),
                name
            );
        }
        component_map.get_mut(name.as_str()).unwrap().as_mut::<V>()
    }
}
