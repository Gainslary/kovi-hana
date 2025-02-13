use kovi::PluginBuilder as plugin;

#[kovi::plugin]
async fn main() {
    plugin::on_msg(|event| async move {
        if event.borrow_text() == Some("Hi Bot") {
            event.reply("Hi This is Hana Bot!")
        }
    });
}
