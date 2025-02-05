use super::common::Policy;
use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{get_thread_tids, UNNAME_TIDS},
};
use hashbrown::HashMap;
#[cfg(debug_assertions)]
use log::debug;
use log::info;
use std::cmp;
use std::time::Duration;

const TOP: [&[u8]; 0] = [];
const ONLY6: [&[u8]; 2] = [b"RHIThread", b"RenderThread"];
const ONLY7: [&[u8]; 0] = [];
const MIDDLE: [&[u8]; 0] = [];
const BACKEND: [&[u8]; 0] = [];

pub fn start_task(args: &mut StartArgs) {
    // 获取全局通道的发送端
    let tx = &UNNAME_TIDS.0;
    args.controller.init_game(*args.pid);

    // 创建一个HashMap<i32, i32>
    let mut high_usage_tids = HashMap::new();

    let mut finish = false;
    let mut usage_top1 = 0;
    let mut insert_count = 0;

    loop {
        let pid = args.activity_utils.top_app_utils.get_pid();
        if pid != args.pid {
            args.controller.init_default();
            return;
        }

        let task_map = args.activity_utils.tid_utils.get_task_map(*pid);
        if finish {
            Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND)
                .execute_policy(task_map, usage_top1, finish);
            std::thread::sleep(Duration::from_millis(100));
        } else {
            let unname_tids = get_thread_tids(task_map, b"Thread-");
            #[cfg(debug_assertions)]
            debug!("发送即将开始");
            tx.send(unname_tids).unwrap();
            #[cfg(debug_assertions)]
            debug!("发送已经完毕");
            std::thread::sleep(Duration::from_millis(100));
            args.controller.update_max_usage_tid();
            let Some(tid1) = args.controller.first_max_tid() else {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            };
        
            if insert_count < 20 {
                *high_usage_tids.entry(tid1).or_insert(0) += 1;
                insert_count += 1;
                #[cfg(debug_assertions)]
                debug!("负载第一高:{tid1}\n");
                Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND)
                    .execute_policy(task_map, tid1, finish);
            } else {
                args.controller.init_default();
                // 按频次排序，取出频次最高的一个tid
                let mut sorted_tids: Vec<_> = high_usage_tids.iter().collect();
                sorted_tids.sort_unstable_by(|(_, a), (_, b)| {
                    b.partial_cmp(a).unwrap_or(cmp::Ordering::Equal)
                });
                sorted_tids.truncate(1);
                usage_top1 = *sorted_tids[0].0;

                finish = true;

                // #[cfg(debug_assertions)]
                info!("计算后最终结果为:{usage_top1}\n");
                continue;
            }
        }

        std::thread::sleep(Duration::from_millis(1900));
    }
}
