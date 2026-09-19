package io.github.fg193.patchouli.systemui

import android.app.Activity
import android.graphics.Color
import android.webkit.WebView
import androidx.activity.ComponentActivity
import androidx.activity.SystemBarStyle
import androidx.activity.enableEdgeToEdge
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat
import androidx.webkit.ScriptHandler
import androidx.webkit.WebViewCompat
import androidx.webkit.WebViewFeature
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin

@InvokeArg
class StatusBarStyleArgs {
    lateinit var backgroundColor: String
    var darkIcons: Boolean = true
}

@TauriPlugin
class SystemUiPlugin(private val activity: Activity) : Plugin(activity) {
    private var insetScript: ScriptHandler? = null

    override fun load(webView: WebView) {
        val launchBackground = activity.getColor(R.color.launch_background)

        applyStatusBarStyle(launchBackground, darkIcons = true)
        webView.setBackgroundColor(launchBackground)

        ViewCompat.setOnApplyWindowInsetsListener(activity.window.decorView) { _, windowInsets ->
            val topInset = windowInsets.getInsets(
                WindowInsetsCompat.Type.systemBars() or
                    WindowInsetsCompat.Type.displayCutout(),
            ).top
            val topInsetCssPixels = topInset / activity.resources.displayMetrics.density
            val script = """
                (() => {
                  const applyInset = () => document.documentElement?.style.setProperty(
                    "--system-safe-area-top",
                    "${topInsetCssPixels}px",
                  );
                  applyInset();
                  if (!document.documentElement) {
                    document.addEventListener("DOMContentLoaded", applyInset, { once: true });
                  }
                })();
            """.trimIndent()

            if (WebViewFeature.isFeatureSupported(WebViewFeature.DOCUMENT_START_SCRIPT)) {
                insetScript?.remove()
                insetScript = WebViewCompat.addDocumentStartJavaScript(webView, script, setOf("*"))
            }
            webView.evaluateJavascript(script, null)
            windowInsets
        }
        ViewCompat.requestApplyInsets(activity.window.decorView)
    }

    @Command
    fun setStatusBarStyle(invoke: Invoke) {
        val args = invoke.parseArgs(StatusBarStyleArgs::class.java)
        val backgroundColor = try {
            Color.parseColor(args.backgroundColor)
        } catch (error: IllegalArgumentException) {
            invoke.reject("Invalid status bar background color", error)
            return
        }

        activity.runOnUiThread {
            applyStatusBarStyle(backgroundColor, args.darkIcons)
            invoke.resolve()
        }
    }

    override fun onDestroy(activity: AppCompatActivity) {
        insetScript?.remove()
        insetScript = null
        ViewCompat.setOnApplyWindowInsetsListener(activity.window.decorView, null)
    }

    private fun applyStatusBarStyle(backgroundColor: Int, darkIcons: Boolean) {
        val statusBarStyle = if (darkIcons) {
            SystemBarStyle.light(backgroundColor, backgroundColor)
        } else {
            SystemBarStyle.dark(backgroundColor)
        }

        (activity as ComponentActivity).enableEdgeToEdge(statusBarStyle = statusBarStyle)
        activity.window.statusBarColor = Color.TRANSPARENT
    }
}
