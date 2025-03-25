pub mod adapter;
pub mod manager;
pub mod peripheral;

use ::jni::objects::JObject;
use ::jni::JNIEnv;
use log::debug;
use once_cell::sync::OnceCell;

mod jni;

static GLOBAL_ADAPTER: OnceCell<adapter::Adapter> = OnceCell::new();

pub fn init(env: &JNIEnv) -> crate::Result<()> {
    debug!("droidplug: jni::init starting");
    match self::jni::init(env) {
        Ok(_) => debug!("JNI initialization succeeded"),
        Err(err) => {
            debug!("JNI initialization failed: {:?}", err);
            return Err(err);
        }
    }
    debug!("droidplug: jni::init finished");
    GLOBAL_ADAPTER.get_or_try_init(|| adapter::Adapter::new())?;
    Ok(())
}

pub fn init_with_loader(env: &JNIEnv, class_loader: JObject) -> crate::Result<()> {
    debug!("droidplug: jni::init_with_loader starting");
    match self::jni::init_with_loader(env, class_loader) {
        Ok(_) => debug!("JNI initialization with_loader succeeded"),
        Err(err) => {
            debug!("JNI initialization with_loader failed: {:?}", err);
            return Err(err);
        }
    }
    debug!("droidplug: jni::init_with_loader finished");
    GLOBAL_ADAPTER.get_or_try_init(|| adapter::Adapter::new_with_loader(class_loader))?;
    Ok(())
}

pub fn global_adapter() -> &'static adapter::Adapter {
    debug!("droidplug::global_adapter starting");
    GLOBAL_ADAPTER.get().expect(
        "Droidplug has not been initialized. Please initialize it with btleplug::platform::init().",
    )
}
