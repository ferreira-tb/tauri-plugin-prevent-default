use tauri::{Runtime, Webview};
use webview2_com::Microsoft::Web::WebView2::Win32::{
  ICoreWebView2Settings3, ICoreWebView2Settings4, ICoreWebView2Settings5, ICoreWebView2Settings6,
};
use windows::core::Interface;

#[must_use]
#[derive(Clone, Debug, Default)]
pub struct PlatformOptions {
  browser_accelerator_keys: Option<bool>,
  built_in_error_page: Option<bool>,
  default_context_menus: Option<bool>,
  default_script_dialogs: Option<bool>,
  dev_tools: Option<bool>,
  general_autofill: Option<bool>,
  host_objects: Option<bool>,
  password_autosave: Option<bool>,
  pinch_zoom: Option<bool>,
  script: Option<bool>,
  swipe_navigation: Option<bool>,
  web_message: Option<bool>,
  zoom_control: Option<bool>,
}

impl PlatformOptions {
  pub fn new() -> Self {
    Self::default()
  }

  /// Determines whether browser-specific accelerator keys are enabled.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.arebrowseracceleratorkeysenabled>
  pub fn browser_accelerator_keys(mut self, enabled: bool) -> Self {
    self.browser_accelerator_keys = Some(enabled);
    self
  }

  /// Determines whether to disable built in error page for navigation failure and render process failure.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.isbuiltinerrorpageenabled>
  pub fn built_in_error_page(mut self, enabled: bool) -> Self {
    self.built_in_error_page = Some(enabled);
    self
  }

  /// Determines whether the default context menus are shown to the user in WebView.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.aredefaultcontextmenusenabled>
  pub fn default_context_menus(mut self, enabled: bool) -> Self {
    self.default_context_menus = Some(enabled);
    self
  }

  /// Determines whether WebView renders the default JavaScript dialog box.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.aredefaultscriptdialogsenabled>
  pub fn default_script_dialogs(mut self, enabled: bool) -> Self {
    self.default_script_dialogs = Some(enabled);
    self
  }

  /// Determines whether the user is able to use the context menu or keyboard shortcuts to open the DevTools window.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.aredevtoolsenabled>
  pub fn dev_tools(mut self, enabled: bool) -> Self {
    self.dev_tools = Some(enabled);
    self
  }

  /// Determines whether general form information will be saved and autofilled.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.isgeneralautofillenabled>
  pub fn general_autofill(mut self, enabled: bool) -> Self {
    self.general_autofill = Some(enabled);
    self
  }

  /// Determines whether host objects are accessible from the page in WebView.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.arehostobjectsallowed>
  pub fn host_objects(mut self, enabled: bool) -> Self {
    self.host_objects = Some(enabled);
    self
  }

  /// Determines whether password information will be autosaved.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.ispasswordautosaveenabled>
  pub fn password_autosave(mut self, enabled: bool) -> Self {
    self.password_autosave = Some(enabled);
    self
  }

  /// Determines the ability of the end users to use pinching motions on touch input enabled devices to scale the web content in the WebView.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.ispinchzoomenabled>
  pub fn pinch_zoom(mut self, enabled: bool) -> Self {
    self.pinch_zoom = Some(enabled);
    self
  }

  /// Determines whether running JavaScript is enabled in all future navigations in the WebView.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.isscriptenabled>
  pub fn script(mut self, enabled: bool) -> Self {
    self.script = Some(enabled);
    self
  }

  /// Determines whether the end user can use swiping gesture on touch input enabled devices to navigate in the WebView.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.isswipenavigationenabled>
  pub fn swipe_navigation(mut self, enabled: bool) -> Self {
    self.swipe_navigation = Some(enabled);
    self
  }

  /// Determines whether communication from the host to the top-level HTML document of the WebView is allowed.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.iswebmessageenabled>
  pub fn web_message(mut self, enabled: bool) -> Self {
    self.web_message = Some(enabled);
    self
  }

  /// Determines whether the user is able to impact the zoom of the WebView.
  ///
  /// <https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2settings.iszoomcontrolenabled>
  pub fn zoom_control(mut self, enabled: bool) -> Self {
    self.zoom_control = Some(enabled);
    self
  }
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn on_webview_ready<R>(webview: &Webview<R>, options: PlatformOptions)
where
  R: Runtime,
{
  let _ = webview.with_webview(move |platform_webview| unsafe {
    let Ok(webview2) = platform_webview.controller().CoreWebView2() else {
      return;
    };

    let Ok(settings) = webview2.Settings() else {
      return;
    };

    if let Some(built_in_error_page) = options.built_in_error_page {
      let _ = settings.SetIsBuiltInErrorPageEnabled(built_in_error_page);
    }

    if let Some(default_context_menus) = options.default_context_menus {
      let _ = settings.SetAreDefaultContextMenusEnabled(default_context_menus);
    }

    if let Some(default_script_dialogs) = options.default_script_dialogs {
      let _ = settings.SetAreDefaultScriptDialogsEnabled(default_script_dialogs);
    }

    if let Some(dev_tools) = options.dev_tools {
      let _ = settings.SetAreDevToolsEnabled(dev_tools);
    }

    if let Some(host_objects) = options.host_objects {
      let _ = settings.SetAreHostObjectsAllowed(host_objects);
    }

    if let Some(script) = options.script {
      let _ = settings.SetIsScriptEnabled(script);
    }

    if let Some(web_message) = options.web_message {
      let _ = settings.SetIsWebMessageEnabled(web_message);
    }

    if let Some(zoom_control) = options.zoom_control {
      let _ = settings.SetIsZoomControlEnabled(zoom_control);
    }

    if let Some(browser_accelerator_keys) = options.browser_accelerator_keys {
      if let Ok(settings3) = settings.cast::<ICoreWebView2Settings3>() {
        let _ = settings3.SetAreBrowserAcceleratorKeysEnabled(browser_accelerator_keys);
      }
    }

    if let Ok(settings4) = settings.cast::<ICoreWebView2Settings4>() {
      if let Some(general_autofill) = options.general_autofill {
        let _ = settings4.SetIsGeneralAutofillEnabled(general_autofill);
      }

      if let Some(password_autosave) = options.password_autosave {
        let _ = settings4.SetIsPasswordAutosaveEnabled(password_autosave);
      }
    }

    if let Some(pinch_zoom) = options.pinch_zoom {
      if let Ok(settings5) = settings.cast::<ICoreWebView2Settings5>() {
        let _ = settings5.SetIsPinchZoomEnabled(pinch_zoom);
      }
    }

    if let Some(swipe_navigation) = options.swipe_navigation {
      if let Ok(settings6) = settings.cast::<ICoreWebView2Settings6>() {
        let _ = settings6.SetIsSwipeNavigationEnabled(swipe_navigation);
      }
    }
  });
}
