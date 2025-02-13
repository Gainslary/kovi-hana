use kovi::{Message, PluginBuilder as plugin};
// 两个 trait，第一个用于 RuntimeBot，第二个用于 Vec
use kovi_plugin_expand_napcat::{NapCatApi, NapCatVec};

#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    plugin::on_msg(|event| async move {
        if event.borrow_text() == Some("Hi Bot") {
            event.reply("Hi This is Hana Bot!")
        }
    })
}
