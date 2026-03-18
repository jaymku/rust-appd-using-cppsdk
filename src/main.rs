#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;// Tells Rust to look for src/bindings.rs
use bindings::*;// Bring all appd_ functions into scope
use std::env;
use std::ffi::CString;
use std::process;
use std::thread::sleep;
use std::time::Duration;

fn require_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        eprintln!("Error: required environment variable {} is not set.", name);
        process::exit(1);
    })
}

fn main() {
    // Load configuration from environment (no credentials in source)
    let controller_host = require_env("APPD_CONTROLLER_HOST");
    let account_name = require_env("APPD_CONTROLLER_ACCOUNT");
    let access_key = require_env("APPD_CONTROLLER_ACCESS_KEY");
    let app_name = env::var("APPD_APP_NAME").unwrap_or_else(|_| "RustHelloWorldApp".into());
    let tier_name = env::var("APPD_TIER_NAME").unwrap_or_else(|_| "RustTier".into());
    let node_name = env::var("APPD_NODE_NAME").unwrap_or_else(|_| "RustNode".into());

    unsafe {
        // --- 1. Setup Configuration ---
        let config = appd_config_init();

        let app = CString::new(app_name).unwrap();
        let tier = CString::new(tier_name).unwrap();
        let node = CString::new(node_name).unwrap();
        let controller_host_c = CString::new(controller_host).unwrap();
        let account_name_c = CString::new(account_name).unwrap();
        let access_key_c = CString::new(access_key).unwrap();

        // Set the configuration settings
        appd_config_set_app_name(config, app.as_ptr());
        appd_config_set_tier_name(config, tier.as_ptr());
        appd_config_set_node_name(config, node.as_ptr());
        appd_config_set_controller_host(config, controller_host_c.as_ptr());
        appd_config_set_controller_port(config, 443);
        appd_config_set_controller_account(config, account_name_c.as_ptr());
        appd_config_set_controller_access_key(config, access_key_c.as_ptr());
        appd_config_set_controller_use_ssl(config, 1);
        appd_config_set_init_timeout_ms(config, 10000);
        // appd_config_set_flush_metrics_on_shutdown:
        // Controls whether the SDK flushes (sends) any unsent metrics to the AppDynamics controller on process shutdown.
        // Default: 0 (do not flush metrics on shutdown).
        // Usage: Pass 1 to enable flushing, 0 to disable.
        appd_config_set_flush_metrics_on_shutdown(config, 1);
        // Set AppDynamics SDK logging levels.
        //appd_config_set_logging_min_level(config, APPD_LOG_LEVEL_INFO);   // Mininal logging
        appd_config_set_logging_min_level(config, appd_config_log_level_APPD_LOG_LEVEL_TRACE); // Trace logs (most verbose)

        // --- 2. Initialize SDK ---
        if appd_sdk_init(config) != 0 {
            eprintln!("Error: AppDynamics SDK failed to start.");
            return;
        }

        println!("SDK Started. Beginning 50-iteration loop..."); // Simulation for 50 API calls

        // --- 3. Instrumented Loop for dummy traffic i.e. Rust Hello World!---
        let bt_name = CString::new("HelloLoopTransaction").unwrap();

        for i in 1..=50 {
            // Start BT tracking
            let bt = appd_bt_begin(bt_name.as_ptr(), std::ptr::null());
            // check if it's initialized (should be non-null if successful)
            println!("appd_bt_begin returned bt handle: {:?}", bt);

            println!("Iteration {}: Hello World!", i);
            sleep(Duration::from_millis(1000)); // Simulate work

            // End BT tracking
            appd_bt_end(bt);
        }

        println!("Loop complete. Shutting down...");
        
        appd_sdk_term();
        println!("SDK Terminated.");
    }
}
