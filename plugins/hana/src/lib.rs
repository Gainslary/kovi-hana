use std::{env, process::exit, sync::Arc};
use exception::PluginError;
use global_state::*;
use kovi::PluginBuilder as plugin;

pub mod util;
pub mod log;
pub mod global_state;
pub mod exception;
pub mod store;
pub mod agent;
pub mod group_notice;
pub mod command;

#[kovi::plugin]
async fn main() {
    if let Err(e) = init_global_state().await {
        log_and_abort(e);
    }

    plugin::on_group_msg(move |e| async move {
        agent::logger(Arc::clone(&e)).await;
        util::sleep_rand_time().await;
        command::act(Arc::clone(&e)).await;
        agent::at_me_handler(Arc::clone(&e)).await;
    });

    plugin::on_all_notice(move |e| async move {
        util::sleep_rand_time().await;
        group_notice::act(e).await;
    });

    plugin::on_admin_msg(|_e| async move {});

    plugin::on_private_msg(move |_e| async move {
        // util::sleep_rand_time().await;
        // no-op
    });
}

fn log_and_abort(e: PluginError) {
    std_error!("{}", e);
    let bot = plugin::get_runtime_bot();
    bot.disable_plugin("chat").unwrap();
    exit(1);
}