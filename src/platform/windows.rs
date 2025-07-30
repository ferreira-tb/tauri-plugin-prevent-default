use tauri::{Runtime, Webview};
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings4;
use windows::core::Interface;

#[non_exhaustive]
pub struct PlatformOptions {
  /// Determine whether general form information will be saved and autofilled.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.isgeneralautofillenabled>
  pub general_autofill: bool,

  /// Determine whether password information will be autosaved.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.ispasswordautosaveenabled>
  pub password_autosave: bool,
}

impl Default for PlatformOptions {
  fn default() -> Self {
    Self {
      general_autofill: true,
      password_autosave: false,
    }
  }
}

pub(crate) fn on_webview_ready<R>(webview: &Webview<R>, options: &PlatformOptions)
where
  R: Runtime,
{
  let general_autofill = options.general_autofill;
  let password_autosave = options.password_autosave;
  let _ = webview.with_webview(move |platform_webview| unsafe {
    let settings = platform_webview
      .controller()
      .CoreWebView2()
      .expect("failed to get ICoreWebView2")
      .Settings()
      .expect("failed to get ICoreWebView2Settings")
      .cast::<ICoreWebView2Settings4>()
      .expect("failed to cast to ICoreWebView2Settings4");

    let _ = settings.SetIsGeneralAutofillEnabled(general_autofill);
    let _ = settings.SetIsPasswordAutosaveEnabled(password_autosave);
  });
}
