use std::borrow::Cow;

use rari_templ_func::rari_f;
use rari_types::{AnyArg, ArgError};
use tracing::{span, Level};

use crate::error::DocError;
use crate::templ::api::RariApi;

/// Creates a link to a DOM/Web API reference page on MDN.
///
/// This macro generates links to Web API interfaces, methods, properties, and other
/// DOM-related documentation. It handles various API naming conventions and can
/// automatically format display text and anchors for methods and properties.
///
/// # Arguments
/// * `api_name` - The API name (interface, method, property, etc.)
/// * `display` - Optional custom display text for the link
/// * `anchor` - Optional anchor/fragment to append to the URL
/// * `no_code` - Optional flag to disable code formatting (default: false)
///
/// # Examples
/// * `{{DOMxRef("Document")}}` -> links to Document interface
/// * `{{DOMxRef("document.getElementById()")}}` -> links to getElementById method
/// * `{{DOMxRef("Element.innerHTML", "innerHTML property")}}` -> custom display text
/// * `{{DOMxRef("Node", "", "", true)}}` -> disables code formatting
///
/// # Special handling
/// - Converts spaces to underscores and removes `()` from method names
/// - Handles prototype chain notation (`.prototype.` becomes `/`)
/// - Automatically capitalizes first letter of interface names in URLs
/// - Appends method/property names to display text when using anchors
/// - Formats links with `<code>` tags unless `no_code` is true
#[rari_f(register = "crate::Templ")]
pub fn domxref(
    api_name: String,
    display: Option<String>,
    anchor: Option<String>,
    no_code: Option<AnyArg>,
) -> Result<String, DocError> {
    let span = span!(Level::ERROR, "domxref", basepath = "/docs/Web/API/");
    let _enter = span.enter();
    let display = display.as_deref().filter(|s| !s.is_empty());
    let mut display_with_fallback = Cow::Borrowed(display.unwrap_or(api_name.as_str()));
    let api = api_name
        .replace(' ', "_")
        .replace("()", "")
        .replace(".prototype.", ".")
        .replace('.', "/");
    if api.is_empty() {
        return Err(DocError::ArgError(ArgError::MustBeProvided));
    }
    let (first_char_index, _) = api.char_indices().next().unwrap_or_default();
    let mut url = format!(
        "/{}/docs/Web/API/{}{}",
        env.locale.as_url_str(),
        &api[0..first_char_index].to_uppercase(),
        &api[first_char_index..],
    );
    if let Some(anchor) = anchor {
        if !anchor.starts_with('#') {
            url.push('#');
            display_with_fallback = Cow::Owned(format!("{display_with_fallback}.{anchor}"));
        }
        url.push_str(&anchor);
        if let Some(anchor) = anchor.strip_prefix('#') {
            display_with_fallback = Cow::Owned(format!("{display_with_fallback}.{anchor}"));
        }
    }

    let code = !no_code.map(|nc| nc.as_bool()).unwrap_or_default();
    RariApi::link(
        &url,
        env.locale,
        Some(&display_with_fallback),
        code,
        display,
        false,
    )
}
