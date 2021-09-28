mod ready;

use poise::serenity_prelude::{async_trait, Context, Event, RawEventHandler};

pub struct Events;

#[async_trait]
impl RawEventHandler for Events {
    async fn raw_event(&self, context: Context, event: Event) {
        match event {
            Event::Ready(event) => ready::handle(context, event.ready),
            _ => (),
        }
    }
}
