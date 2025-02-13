use kovi::build_bot;

fn main() {
    let bot = build_bot!(hi, kovi_plugin_cmd);
    bot.run();
}
