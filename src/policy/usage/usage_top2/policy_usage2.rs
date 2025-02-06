use super::common::execute_policy;
use crate::policy::{
    pkg_cfg::StartArgs,
    usage::{get_thread_tids, UNNAME_TIDS},
};
use hashbrown::HashMap;
use libc::pid_t;
#[cfg(debug_assertions)]
use log::debug;
use log::info;
use std::cmp;
use std::time::Duration;

pub fn start_task(args: &mut StartArgs) {
    // 获取全局通道的发送端
    let tx = &UNNAME_TIDS.0;
    args.controller.init_game(*args.pid);
    // 创建一个HashMap<i32, i32>
    let mut high_usage_tids: Option<HashMap<pid_t, i32>> = Some(HashMap::new());

    let mut finish = false;

    let mut usage_top1 = 0;
    let mut usage_top2 = 0;
    let mut insert_count = 0;

    loop {
        let pid = args.activity_utils.top_app_utils.get_pid();
        if pid != args.pid {
            args.controller.init_default();
            return;
        }

        let task_map = args.activity_utils.tid_utils.get_task_map(*pid);

        if finish {
            execute_policy(task_map, usage_top1, usage_top2);
            std::thread::sleep(Duration::from_millis(100));
        } else {
            let unname_tids = get_thread_tids(task_map, b"Thread-");
            #[cfg(debug_assertions)]
            debug!("发送即将开始");
            tx.send(unname_tids).unwrap();
            #[cfg(debug_assertions)]
            debug!("发送已经完毕，喵等待一段时间计算");
            std::thread::sleep(Duration::from_millis(100));
            args.controller.update_max_usage_tid();
            let Some(tid1) = args.controller.first_max_tid() else {
                #[cfg(debug_assertions)]
                debug!("获取不到first max tid，直接循环");
                std::thread::sleep(Duration::from_millis(100));
                continue;
            };

            let Some(tid2) = args.controller.second_max_tid() else {
                #[cfg(debug_assertions)]
                debug!("获取不到second max tid，直接循环");
                std::thread::sleep(Duration::from_millis(100));
                continue;
            };

            if insert_count < 11 {
                insert_count += 1;
                if let Some(map) = high_usage_tids.as_mut() {
                    *map.entry(tid1).or_insert(0) += 1;
                    *map.entry(tid2).or_insert(0) += 1;
                }
                #[cfg(debug_assertions)]
                debug!("负载第一高:{tid1}\n第二高:{tid2}");
                execute_policy(task_map, tid1, tid2);
            } else {
                // 按频次排序，取出频次最高的两个tid
                if let Some(map) = high_usage_tids.as_mut() {
                    if map.len() != 2 {
                        #[cfg(debug_assertions)]
                        debug!("map长度不为2，重新vote");
                        map.clear();
                        insert_count = 0;
                        continue;
                    }
                    let mut sorted_tids: Vec<_> = map.iter().collect();
                    sorted_tids.sort_unstable_by(|(_, a), (_, b)| {
                        b.partial_cmp(a).unwrap_or(cmp::Ordering::Equal)
                    });
                    sorted_tids.truncate(2);
                    usage_top1 = *sorted_tids[0].0;
                    usage_top2 = *sorted_tids[1].0;
                }

                args.controller.init_default();
                finish = true;
                high_usage_tids = None;
                #[cfg(debug_assertions)]
                info!("计算后最终结果为:{usage_top1}\n第二高:{usage_top2}");
                continue;
            }
        }

        std::thread::sleep(Duration::from_millis(1900));
    }
}
