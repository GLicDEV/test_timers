use std::time::Duration;

use candid::candid_method;
use ic_cdk::timer::TimerId;
use ic_cdk_macros::*;

#[candid_method(update, rename = "testTimers")]
#[update(name = "testTimers")]
fn async_flow(count: u16) {
    for i in 0..count {
        let timer_id: TimerId =
            ic_cdk::timer::set_timer(Duration::new(5, 0), move || call_from_timer(i));
        ic_cdk::print(format!(
            "Iteration: {} - Added a timer with timer_id: {:?}",
            i, timer_id
        ));
    }
}

fn call_from_timer(iteration: u16) {
    ic_cdk::print(format!("Iteration: {} - Got called from timer!", iteration));
}

// Auto export the candid interface
candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_candid() {
        let expected =
            String::from_utf8(std::fs::read("test_timers_backend.did").unwrap()).unwrap();

        let actual = __export_service();

        if actual != expected {
            println!("{}", actual);
        }

        assert_eq!(
            actual, expected,
            "Generated candid definition does not match expected did file"
        );
    }
}
