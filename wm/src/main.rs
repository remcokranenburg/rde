use std::process::Command;

#[cfg(feature = "profile-with-tracy-mem")]
#[global_allocator]
static GLOBAL: profiling::tracy_client::ProfiledAllocator<std::alloc::System> =
    profiling::tracy_client::ProfiledAllocator::new(std::alloc::System, 10);

fn main() -> Result<(), anyhow::Error> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt()
            .compact()
            .with_env_filter(env_filter)
            .init();
    } else {
        tracing_subscriber::fmt().compact().init();
    }

    #[cfg(feature = "profile-with-tracy")]
    profiling::tracy_client::Client::start();

    profiling::register_thread!("Main Thread");

    #[cfg(feature = "profile-with-puffin")]
    let _server =
        puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT)).unwrap();
    #[cfg(feature = "profile-with-puffin")]
    profiling::puffin::set_scopes_on(true);

    let env_wayland_display = ::std::env::var("WAYLAND_DISPLAY").ok();
    tracing::info!("WAYLAND_DISPLAY: {:?}", env_wayland_display);

    let mut args = std::env::args().into_iter();
    let mut dev_panel = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dev-panel" => {
                dev_panel = true;
            }
            "--help" | "-h" => {
                println!("Usage: wm [OPTIONS]");
                println!();
                println!("Options:");
                println!("  --help, -h          Print this help message");
                println!("  --dev-panel         Launch the development panel");
                return Ok(());
            }
            _ => {}
        }
    }

    match env_wayland_display.as_deref() {
        Some(envvar) if !envvar.is_empty() => {
            tracing::info!("Starting wm with winit backend because WAYLAND_DISPLAY is set");

            #[cfg(not(feature = "winit"))]
            panic!("winit backend is not enabled.");

            wm::winit::run_winit(dev_panel)
        }
        _ => {
            tracing::info!("Starting wm on a tty using udev because WAYLAND_DISPLAY is not set");

            #[cfg(not(feature = "udev"))]
            panic!("udev backend is not enabled.");

            wm::udev::run_udev(dev_panel)
        }
    }
}
