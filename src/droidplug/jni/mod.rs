pub mod objects;

use ::jni::{objects::JObject, JNIEnv, JavaVM, NativeMethod};
use jni::{objects::JString, sys::jboolean};
use log::debug;
use once_cell::sync::OnceCell;
use std::ffi::c_void;

static GLOBAL_JVM: OnceCell<JavaVM> = OnceCell::new();

pub fn init(env: &JNIEnv) -> crate::Result<()> {
    debug!("JNI initialization starting");
    match GLOBAL_JVM.set(env.get_java_vm()?) {
        Ok(_) => debug!("Java VM successfully set in GLOBAL_JVM"),
        Err(_) => debug!("Failed to set Java VM in GLOBAL_JVM"),
    };

    debug!("Registering native methods");
    let class = env.auto_local(env.find_class("com/nonpolynomial/btleplug/android/impl/Adapter")?);
    env.register_native_methods(
        &class,
        &[
            NativeMethod {
                name: "reportScanResult".into(),
                sig: "(Landroid/bluetooth/le/ScanResult;)V".into(),
                fn_ptr: adapter_report_scan_result as *mut c_void,
            },
            NativeMethod {
                name: "onConnectionStateChanged".into(),
                sig: "(Ljava/lang/String;Z)V".into(),
                fn_ptr: adapter_on_connection_state_changed as *mut c_void,
            },
        ],
    )?;
    debug!("Native methods registered");

    let classes = [
        "com/nonpolynomial/btleplug/android/impl/Peripheral",
        "com/nonpolynomial/btleplug/android/impl/ScanFilter",
        "com/nonpolynomial/btleplug/android/impl/NotConnectedException",
        "com/nonpolynomial/btleplug/android/impl/PermissionDeniedException",
        "com/nonpolynomial/btleplug/android/impl/UnexpectedCallbackException",
        "com/nonpolynomial/btleplug/android/impl/UnexpectedCharacteristicException",
        "com/nonpolynomial/btleplug/android/impl/NoSuchCharacteristicException",
    ];

    for class in &classes {
        debug!("Finding and adding class: {}", class);
        jni_utils::classcache::find_add_class(env, class)?;
    }
    debug!("All classes found and added successfully");

    Ok(())
}

pub fn global_jvm() -> &'static JavaVM {
    GLOBAL_JVM.get().expect(
        "Droidplug has not been initialized. Please initialize it with btleplug::platform::init().",
    )
}

impl From<::jni::errors::Error> for crate::Error {
    fn from(err: ::jni::errors::Error) -> Self {
        Self::Other(Box::new(err))
    }
}

extern "C" fn adapter_report_scan_result(env: JNIEnv, obj: JObject, scan_result: JObject) {
    let _ = super::adapter::adapter_report_scan_result_internal(&env, obj, scan_result);
}

extern "C" fn adapter_on_connection_state_changed(
    env: JNIEnv,
    obj: JObject,
    addr: JString,
    connected: jboolean,
) {
    let _ =
        super::adapter::adapter_on_connection_state_changed_internal(&env, obj, addr, connected);
}
