mod server;
mod http1;
mod server_config;

fn log_launch_and_open_browser(sc: &server_config::ServerConfig) {
    let addr = sc.addr().to_string();
    log::info!(
        "🚀 SpcaePls launched at [{}] over {}",
        addr,
        sc.http_version()
    );
    if sc.graphiql() {
        let url = sc.graphiql_url();
        log::info!("🌍 Playground: {}", url);

        let _ = webbrowser::open(url.as_str());
    }
}
