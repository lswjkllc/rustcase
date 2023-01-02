use std::sync::Arc;
use tracing::debug;

use crate::{Storage, CommandRequest, CommandResponse, MemTable, command_request::RequestData, KvError};

mod command_service;

/// 对 Command 的处理的抽象
pub trait CommandService {
    /// 处理 Command，返回 Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

/// Service 内部 ServiceInner 数据结构
struct ServiceInner<Store> {
    store: Store,
}

/// Service 数据结构
#[derive(Clone)]
pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

// impl<Store> Clone for Service<Store> {
//     fn clone(&self) -> Self {
//         Self {
//             inner: Arc::clone(&self.inner),
//         }
//     }
// }

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store })
        }
    }

    pub fn print_elems(&self, table: &str) {
        match self.inner.store.get_all(table) {
            Ok(pairs) => pairs.iter().for_each(|v| println!("table: {} ===> {:?}", table, v)),
            Err(_) => println!("unknown error")
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        // TODO: 发送 on_received 事件
        let resp = dispatch(cmd, &self.inner.store);
        debug!("Executed response: {:?}", resp);
        // TODO: 发送 on_executed 事件

        resp
    }
}

/// 核心分发任务函数!!!
fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        Some(RequestData::Hgetall(param)) => param.execute(store),
        Some(RequestData::Hset(param)) => param.execute(store),
        _ => KvError::Internal("Not implemented".into()).into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MemTable, Value};

    #[test]
    fn service_should_works() {
        // 我们需要一个 service 结构至少包含 Storage
        let service = Service::new(MemTable::default());

        // service 可以运行在多线程环境下，它的 clone 应该是轻量级的
        let cloned = service.clone();

        // 创建一个线程，在 table t1 中写入 k1, v1
        let handle = std::thread::spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        // 在当前线程下读取 table t1 的 k1，应该返回 v1
        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

#[cfg(test)]
use crate::{Kvpair, Value};

// 测试成功返回的结果
#[cfg(test)]
pub fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

// 测试失败返回的结果
#[cfg(test)]
pub fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
    assert_eq!(res.status, code);
    assert!(res.message.contains(msg));
    assert_eq!(res.values, &[]);
    assert_eq!(res.pairs, &[]);
}