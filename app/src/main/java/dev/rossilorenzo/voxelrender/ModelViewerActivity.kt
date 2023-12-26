package dev.rossilorenzo.voxelrender

import android.content.Intent
import android.os.Bundle
import android.util.Log
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import com.google.androidgamesdk.GameActivity


class ModelViewerActivity : GameActivity() {

    var scene: ByteArray? = null
    private fun hideSystemUI() {
        // From API 30 onwards, this is the recommended way to hide the system UI, rather than
        // using View.setSystemUiVisibility.
        val decorView = window.decorView
        val controller = WindowInsetsControllerCompat(
            window,
            decorView
        )
        controller.hide(WindowInsetsCompat.Type.systemBars())
        controller.hide(WindowInsetsCompat.Type.displayCutout())
        controller.systemBarsBehavior =
            WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        // When true, the app will fit inside any system UI windows.
        // When false, we render behind any system UI windows.
        WindowCompat.setDecorFitsSystemWindows(window, false)
        hideSystemUI()

        if (Intent.ACTION_VIEW == intent.action) {
            Log.i(TAG, "INTENT IS VIEW!!!")
        } else {
            Log.e(TAG, "intent was something else: ${intent.action}")
            finish()
            super.onCreate(savedInstanceState)
            return
        }

        scene = intent.data?.let { uri ->
            if (uri.scheme == "assets") {
                val path = (uri.path ?: "/").substring(1)
                Log.e(TAG, "URI IS: $uri, path: $path")
                assets.open(path)
            } else {
                contentResolver.openInputStream(uri)
            }
        }?.let { stream ->
            stream.use { it.readBytes() }
        }

        // You can set IME fields here or in native code using GameActivity_setImeEditorInfoFields.
        // We set the fields in native_engine.cpp.
        // super.setImeEditorInfoFields(InputType.TYPE_CLASS_TEXT,
        //     IME_ACTION_NONE, IME_FLAG_NO_FULLSCREEN );
        super.onCreate(savedInstanceState)
    }

    fun onNativeError(error: String) {
        Log.e(TAG, error)
    }

    companion object {
        const val TAG = "ModelViewer"

        init {
            // Load the native library.
            // The name "android-game" depends on your CMake configuration, must be
            // consistent here and inside AndroidManifect.xml
            System.loadLibrary("main")
        }
    }
}
