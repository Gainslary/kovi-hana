use kovi::build_bot;

fn main() {
    let bot = build_bot!(hana, kovi_plugin_cmd);
    bot.run();
}
