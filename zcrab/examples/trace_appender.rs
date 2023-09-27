use tracing::info;
use zcrab::trace::appender;

//#[tokio::main] // support tokio async
//async fn main(){
fn main(){
    let _guard = appender::init_appender(
        None,
        tracing::Level::TRACE,
        true,
        false,
        false,
        false);

    info!("Hello tracing appender");
}
