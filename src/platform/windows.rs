use tauri::{Runtime, Webview};
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Settings4;
use windows::core::Interface;

pub struct WebviewSettings {
  pub general_autofill: bool,
  pub password_autosave: bool,
}

pub fn on_webview_ready<R>(webview: &Webview<R>, settings: &WebviewSettings)
where
  R: Runtime,
{
  let general_autofill = settings.general_autofill;
  let password_autosave = settings.password_autosave;
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
