use objc2_web_kit::WKWebView;
use tauri::{Runtime, Webview};

pub struct WebkitOptions {
  /// Determine whether pressing a link displays a preview of the destination for the link.
  ///
  /// <https://developer.apple.com/documentation/webkit/wkwebview/allowslinkpreview?language=objc>
  pub allows_link_preview: bool,
}

impl Default for WebkitOptions {
  fn default() -> Self {
    Self { allows_link_preview: false }
  }
}

pub(crate) fn on_webview_ready<R>(webview: &Webview<R>, options: &WebkitOptions)
where
  R: Runtime,
{
  let allows_link_preview = options.allows_link_preview;
  let _ = webview.with_webview(move |platform_webview| unsafe {
    let web_view = platform_webview
      .inner()
      .cast::<WKWebView>()
      .as_ref()
      .expect("failed to get WKWebView");

    web_view.setAllowsLinkPreview(allows_link_preview);
  });
}
