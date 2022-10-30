use once_cell::sync::OnceCell;
use rhai::Engine;

// i dont know any better way to do this than do a python style GIL :sadge:
static GLOBAL_BONGTALK_LOCK: Arc<OnceCell<BongtalkContext>> = Arc::new(OnceCell::new());

pub struct BongtalkContext {
    rhai_engine: Engine,
}