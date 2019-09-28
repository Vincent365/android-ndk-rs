//! Bindings for `ANativeActivity`
//!
//! See also [the NDK
//! docs](https://developer.android.com/ndk/reference/struct/a-native-activity.html)

//use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr::NonNull;

/// An `ANativeActivity *`
///
/// This is either provided in `ANativeActivity_onCreate`, or accessible in
/// `android_native_app_glue`'s android_app.
pub struct NativeActivity {
    ptr: NonNull<ffi::ANativeActivity>,
}

// It gets shared between threads in android_native_app_glue
unsafe impl Send for NativeActivity {}
unsafe impl Sync for NativeActivity {}

impl NativeActivity {
    /// Create a `NativeActivity` from a pointer
    ///
    /// By calling this function, you assert that it is a valid pointer to a native
    /// `ANativeActivity`.
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ANativeActivity>) -> Self {
        Self { ptr }
    }

    /// The pointer to the native `ANativeActivity`
    pub fn ptr(&self) -> NonNull<ffi::ANativeActivity> {
        self.ptr
    }
}

/// Methods that relate to fields of the struct itself
///
/// The relevant NDK docs can be found
/// [here.](https://developer.android.com/ndk/reference/struct/a-native-activity)
impl NativeActivity {
    /// The platform's SDK version code
    pub fn sdk_version(&self) -> i32 {
        unsafe { self.ptr.as_ref().sdkVersion }
    }

    /// Path to this application's internal data directory
    pub fn internal_data_path(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.ptr.as_ref().internalDataPath) }
    }

    /// Path to this application's external (removable, mountable) data directory
    pub fn external_data_path(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.ptr.as_ref().externalDataPath) }
    }

    /// This app's asset manager, which can be used to access assets from the `.apk` file.
    pub fn asset_manager(&self) -> crate::asset::AssetManager {
        unsafe {
            crate::asset::AssetManager::from_ptr(
                NonNull::new(self.ptr.as_ref().assetManager).unwrap(),
            )
        }
    }

    /// Instance data associated with the activity
    pub fn instance(&self) -> *mut c_void {
        unsafe { self.ptr.as_ref().instance }
    }

    /// Instance data associated with the activity
    pub fn instance_mut(&mut self) -> &mut *mut c_void {
        unsafe { &mut self.ptr.as_mut().instance }
    }

    /// This processe's `JavaVM` object.
    ///
    /// ```no_run
    /// # use android_ndk::native_activity::NativeActivity;
    /// # let native_activity: NativeActivity = unimplemented!();
    /// let vm = native_activity.vm();
    /// let env = vm.attach_current_thread();
    /// // Do JNI with env ...
    /// ```
    pub fn vm(&self) -> jni::JavaVM {
        unsafe { jni::JavaVM::from_raw(self.ptr.as_ref().vm as *mut _).unwrap() }
    }

    /// The `android.app.NativeActivity` instance
    ///
    /// In the JNI, this is named `clazz`; however, as the docs say, "it should really be named
    /// 'activity' instead of 'clazz', since it's a reference to the NativeActivity instance.
    pub fn activity(&self) -> jni::objects::JObject<'_> {
        unsafe {
            jni::objects::JObject::from(&self.ptr.as_ref().clazz as *const _ as jni::sys::jobject)
        }
    }

    /// Path to the directory with the application's OBB files.
    ///
    /// Only available as of Honeycomb (Android 3.0+, API level 11+)
    pub unsafe fn obb_path(&self) -> &CStr {
        CStr::from_ptr(self.ptr.as_ref().obbPath)
    }
}

/// Methods that relate to `ANativeActivity_*` functions.
///
/// The relevant NDK docs can be found
/// [here.](https://developer.android.com/ndk/reference/group/native-activity)
impl NativeActivity {
    /// Sends a destroy event to the activity and stops it.
    pub fn finish(&self) {
        unsafe { ffi::ANativeActivity_finish(self.ptr.as_ptr()) }
    }

    /// Shows the IME (the on-screen keyboard).
    ///
    /// If `force` is true, the `SHOW_FORCED` flag is used; otherwise, the `SHOW_IMPLICIT` flag is
    /// used.  Depending on the value of this flag, the `hide_soft_input` method with behave
    /// differently.  See [the relevant
    /// javadoc](https://developer.android.com/reference/android/view/inputmethod/InputMethodManager#constants_2)
    /// for more information.
    pub fn show_soft_input(&self, force: bool) {
        let flag = if force {
            ffi::ANATIVEACTIVITY_SHOW_SOFT_INPUT_FORCED
        } else {
            ffi::ANATIVEACTIVITY_SHOW_SOFT_INPUT_IMPLICIT
        };
        unsafe { ffi::ANativeActivity_showSoftInput(self.ptr.as_ptr(), flag) }
    }

    /// Hides the IME (the on-screen keyboard).
    ///
    /// If `not_always` is true, the `HIDE_NOT_ALWAYS` flag is used; otherwise, the
    /// `HIDE_IMPLICIT_ONLY` flag is used.  Depending on the value of this flag and the way the IME
    /// was shown, it may or may not be hidden.  See [the relevant
    /// javadoc](https://developer.android.com/reference/android/view/inputmethod/InputMethodManager#constants_2)
    /// for more information.
    pub fn hide_soft_input(&self, not_always: bool) {
        let flag = if not_always {
            ffi::ANATIVEACTIVITY_HIDE_SOFT_INPUT_NOT_ALWAYS
        } else {
            ffi::ANATIVEACTIVITY_HIDE_SOFT_INPUT_IMPLICIT_ONLY
        };
        unsafe { ffi::ANativeActivity_hideSoftInput(self.ptr.as_ptr(), flag) }
    }

    /*/// Set the window format. Performs the Java `.getWindow().setFormat()`.
    ///
    /// See also [the relevant
    /// javadoc](https://developer.android.com/reference/android/view/Window#setFormat(int))
    pub unsafe fn set_window_format(&self, format: WindowFormat) {
        unsafe { ffi::ANativeActivity_setWindowFormat(self.ptr.as_ptr(), format.into()) }
    }*/
}

/*#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum WindowFormat {
    Rgb565 = ffi::ANativeWindow_LegacyFormat_WINDOW_FORMAT_RGB_565,
    Rgba8888 = ffi::ANativeWindow_LegacyFormat_WINDOW_FORMAT_RGBA_8888,
    Rgbx8888 = ffi::ANativeWindow_LegacyFormat_WINDOW_FORMAT_RGBX_8888,
}*/
