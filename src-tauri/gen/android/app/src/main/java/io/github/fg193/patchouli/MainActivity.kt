package io.github.fg193.patchouli

import android.os.Bundle
import android.webkit.WebView
import androidx.activity.SystemBarStyle
import androidx.activity.enableEdgeToEdge
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat
import androidx.webkit.ScriptHandler
import androidx.webkit.WebViewCompat
import androidx.webkit.WebViewFeature

class MainActivity : TauriActivity() {
  private var insetScript: ScriptHandler? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge(
      statusBarStyle = SystemBarStyle.light(
        getColor(R.color.launch_background),
        getColor(R.color.launch_background),
      ),
    )
    super.onCreate(savedInstanceState)
  }

  override fun onWebViewCreate(webView: WebView) {
    super.onWebViewCreate(webView)

    // Keep the WebView edge-to-edge and expose only the top inset to CSS. Applying
    // padding to WebView clips position:sticky content instead of moving it.
    webView.setBackgroundColor(getColor(R.color.launch_background))

    ViewCompat.setOnApplyWindowInsetsListener(window.decorView) { _, windowInsets ->
      val topInset = windowInsets.getInsets(
        WindowInsetsCompat.Type.systemBars() or
          WindowInsetsCompat.Type.displayCutout(),
      ).top
      val topInsetCssPixels = topInset / resources.displayMetrics.density
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
    ViewCompat.requestApplyInsets(window.decorView)
  }
}
