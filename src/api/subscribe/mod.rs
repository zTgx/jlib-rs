pub mod subs;

use api::subscribe::subs::Subscribe;
pub use api::subscribe::subs::SubscribeI;

lazy_static! {
    pub static ref SUBSCRIBE: Subscribe = {
        Subscribe::new()
    };
}