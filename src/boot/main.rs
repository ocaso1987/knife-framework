use super::bootstrap::SERVER;

use super::bootstrap::Bootstrap;

/// 启动服务
pub fn start_server<F>(f: F)
where
    F: Fn() + Send + 'static,
{
    let bootstrap = Bootstrap::get_instance() as &mut Bootstrap;
    bootstrap.start(SERVER, f);
}

/// 停止服务
pub fn stop_server<F>(_f: F)
where
    F: Fn() + Send + 'static,
{
}
