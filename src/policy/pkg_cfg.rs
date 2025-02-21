use super::{
    name_match::policies::{policy_sky, policy_ue, policy_unity},
    usage_top1::policies::{
        policy_cocos, policy_top1, policy_ue5, policy_unity_t1, policy_unity_t1_u2,
    },
    usage_top2::{policy_party, policy_top2, policy_ue_t2, policy_unity_t2},
};
use crate::activity::ActivityUtils;
use libc::pid_t;

// 对于普通的Unity游戏
const UNITY: [&str; 13] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.GenshinImpact",
    "com.miHoYo.ys.bilibili",
    "com.miHoYo.yuanshencb",
    "com.miHoYo.hkrpg",
    "com.miHoYo.hkrpg.bilibili",
    "com.HoYoverse.hkrpgoversea",
    "com.miHoYo.hkrpgcb",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
    "com.tencent.KiHan",
];

// 单纯的的线程名匹配，对于ue游戏
const UE: [&str; 1] = ["com.kurogame.mingchao"];

// 单纯的的线程名匹配，对于光遇游戏
const SKY_T2: [&str; 1] = ["com.netease.sky"];

// 对于需要取一个cputime最大的线程，其线程前缀名为"Thread-"
const UE_T1: [&str; 2] = ["com.tencent.lzhx", "com.tencent.tmgp.pubgmhd"];

// 需要取一个cputime最大的线程，其线程前缀名为"GameThread"，只有无限暖暖
const UE5_T1: [&str; 1] = ["com.papegames.infinitynikki"];

// 对于三国杀，跟暖暖策略一样，只是线程名不同
const COCOS_T1: [&str; 1] = ["com.bf.sgs.hdexp"];

// 需要取一个cputime第二大的线程，其线程前缀名为"Thread-"，且为unity游戏
const UNITY_T1_U2: [&str; 1] = ["com.tencent.lolm"];

// 需要单独把负载最重的unitymain绑定到cpu7
const UNITY_T1: [&str; 2] = ["com.tencent.tmgp.cod", "com.tencent.tmgp.cf"];

// 对于需要取两个重负载线程的游戏，其线程前缀名均为"Thread-"，目前策略是燕云十六声特调
const USAGE_T2: [&str; 1] = ["com.netease.yyslscn"];

// 对于需要取两个重负载线程的游戏，其线程前缀名分别为"Thread-"，"MainThread"，目前策略是蛋仔派对特调
const PARTY_T2: [&str; 1] = ["com.netease.party"];

// 对于需要取两个重负载线程的游戏，其线程前缀名分别为"Thread-"，"UnityMain"，目前策略是NBA巅峰对决特调
const UNITY_T2: [&str; 1] = ["com.galasports.operablebasketball.mi"];

// 对于需要取两个重负载线程的游戏，其线程前缀名分别为"GameThread","RenderThread"，目前策略只有三角洲
const UE_T2: [&str; 1] = ["com.tencent.tmgp.dfm"];

pub struct StartArgs<'a> {
    pub activity_utils: &'a mut ActivityUtils,
    pub pid: pid_t,
}

type ConfigTuple = (&'static [&'static str], fn(&mut StartArgs));

pub const PACKAGE_CONFIGS: [ConfigTuple; 12] = [
    (&UE_T1, policy_top1::start_task),
    (&USAGE_T2, policy_top2::start_task),
    (&UNITY_T2, policy_unity_t2::start_task),
    (&PARTY_T2, policy_party::start_task),
    (&UNITY, policy_unity::start_task),
    (&UE, policy_ue::start_task),
    (&UE_T2, policy_ue_t2::start_task),
    (&SKY_T2, policy_sky::start_task),
    (&UE5_T1, policy_ue5::start_task),
    (&COCOS_T1, policy_cocos::start_task),
    (&UNITY_T1_U2, policy_unity_t1_u2::start_task),
    (&UNITY_T1, policy_unity_t1::start_task),
];
