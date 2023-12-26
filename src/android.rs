use winit::{
    platform::android::activity::AndroidApp,
    event_loop::EventLoopBuilder
};
use jni::{objects::{JByteArray, JObject, JValue}, JavaVM, JNIEnv};
use anyhow::Context;

use crate::{Scene, parse_scene, run};



fn get_scene(env: &mut JNIEnv, activity: &JObject) -> anyhow::Result<Option<Scene>> {
    let data = env.get_field(&activity, "scene", "[B").context("Failed to read field")?;
    let jdata = data.l().context("Field is not an object")?;
    if env.is_same_object(&jdata, JObject::null())? {
        return Ok(None);
    }
    let jdata: &JByteArray = &jdata.try_into()?;

    let rdata = env.convert_byte_array(jdata).context("Could not convert byte array")?;

    Ok(Some(parse_scene(&rdata, None).context("Cannot parse model")?))
}

fn throw_error(env: &mut JNIEnv, activity: &JObject, error: anyhow::Error) -> anyhow::Result<()> {
    let error = env.new_string(format!("{:?}", error))?;
    env.call_method(&activity, "onNativeError", "(Ljava/lang/String;)V", &[JValue::Object(&error.into())])?;
    Ok(())
}



#[allow(dead_code)]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) }.expect("Cannot find Java VM");
    let mut env = vm.attach_current_thread().expect("Cannot attach to current thread");
    let activity = unsafe { JObject::from_raw(app.activity_as_ptr().cast()) };


    let scene = match get_scene(&mut env, &activity) {
        Ok(x) => x,
        Err(e) => {
            throw_error(&mut env, &activity, e).expect("Failed to send error to onNativeError");
            return;
        }
    };

    let event_loop = EventLoopBuilder::new().with_android_app(app).build().unwrap();
    run(event_loop, scene);
}
