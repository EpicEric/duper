use tracing::{debug, warn};
use tracing_duper::DuperLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tracing::instrument]
fn send_gifts(count: &mut usize) {
    if *count < 12 {
        warn!("too few gifts... try again later");
    } else {
        debug!(
            user_id = &b"santa"[..],
            "$duper.delivery_date" = "(PlainMonthDay('12-25'), \"Christmas\")",
            "sending {count} gifts"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
        *count = 0;
    }
}

fn main() {
    tracing_subscriber::registry()
        .with(DuperLayer::new().with_span_timings(true))
        .init();
    let mut gifts = 10;
    send_gifts(&mut gifts);
    gifts += 13;
    send_gifts(&mut gifts);
}
