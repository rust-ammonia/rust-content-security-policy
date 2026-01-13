#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};
use bitflags::bitflags;

bitflags!{
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    pub struct SandboxingFlagSet: u32 {
        const SANDBOXED_NAVIGATION_BROWSING_CONTEXT_FLAG = 0x00000001;
        const SANDBOXED_AUXILIARY_NAVIGATION_BROWSING_CONTEXT_FLAG = 0x00000002;
        const SANDBOXED_TOP_LEVEL_NAVIGATION_WITHOUT_USER_ACTIVATION_BROWSING_CONTEXT_FLAG
            = 0x00000004;
        const SANDBOXED_TOP_LEVEL_NAVIGATION_WITH_USER_ACTIVATION_BROWSING_CONTEXT_FLAG
            = 0x00000008;
        const SANDBOXED_PLUGINS_BROWSING_CONTEXT_FLAG = 0x00000010;
        const SANDBOXED_ORIGIN_BROWSING_CONTEXT_FLAG = 0x00000020;
        const SANDBOXED_FORMS_BROWSING_CONTEXT_FLAG = 0x00000040;
        const SANDBOXED_POINTER_LOCK_BROWSING_CONTEXT_FLAG = 0x00000080;
        const SANDBOXED_SCRIPTS_BROWSING_CONTEXT_FLAG = 0x00000100;
        const SANDBOXED_AUTOMATIC_FEATURES_BROWSING_CONTEXT_FLAG = 0x00000200;
        const SANDBOXED_STORAGE_AREA_URLS_FLAG = 0x00000400;
        const SANDBOXED_DOCUMENT_DOMAIN_BROWSING_CONTEXT_FLAG = 0x00000800;
        const SANDBOX_PROPOGATES_TO_AUXILIARY_BROWSING_CONTEXTS_FLAG = 0x00001000;
        const SANDBOXED_MODALS_FLAG = 0x00002000;
        const SANDBOXED_ORIENTATION_LOCK_BROWSING_CONTEXT_FLAG = 0x00004000;
        const SANDBOXED_PRESENTATION_BROWSING_CONTEXT_FLAG = 0x00008000;
        const SANDBOXED_DOWNLOADS_BROWSING_CONTEXT_FLAG = 0x0010000;
        const SANBOXED_CUSTOM_PROTOCOLS_NAVIGATION_BROWSING_CONTEXT_FLAG = 0x0020000;
    }
}

/// <https://html.spec.whatwg.org/multipage/#parse-a-sandboxing-directive>
pub fn parse_a_sandboxing_directive(tokens: &[String]) -> SandboxingFlagSet {
    // Step 1. Split input on ASCII whitespace, to obtain tokens.
    //
    // Performed by callers
    // Step 2. Let output be empty.
    //
    // We inverse the logic here where we add all and then remove when required.
    // This is why we don't need to explicitly add some flags, separate from the
    // specification
    let mut output = SandboxingFlagSet::all();
    // Step 3. Add the following flags to output:
    for token in tokens {
        let remove = match &token[..] {
            // The sandboxed auxiliary navigation browsing context flag, unless tokens contains the allow-popups keyword.
            "allow-popups" =>
                SandboxingFlagSet::SANDBOXED_AUXILIARY_NAVIGATION_BROWSING_CONTEXT_FLAG |
                SandboxingFlagSet::SANBOXED_CUSTOM_PROTOCOLS_NAVIGATION_BROWSING_CONTEXT_FLAG,
            // The sandboxed top-level navigation without user activation browsing context flag, unless tokens contains the allow-top-navigation keyword.
            "allow-top-navigation" =>
                SandboxingFlagSet::SANDBOXED_TOP_LEVEL_NAVIGATION_WITHOUT_USER_ACTIVATION_BROWSING_CONTEXT_FLAG |
                    SandboxingFlagSet::SANDBOXED_TOP_LEVEL_NAVIGATION_WITH_USER_ACTIVATION_BROWSING_CONTEXT_FLAG |
                    SandboxingFlagSet::SANBOXED_CUSTOM_PROTOCOLS_NAVIGATION_BROWSING_CONTEXT_FLAG,
            // The sandboxed top-level navigation with user activation browsing context flag, unless tokens contains either
            // the allow-top-navigation-by-user-activation keyword or the allow-top-navigation keyword.
            "allow-top-navigation-by-user-activation" =>
                SandboxingFlagSet::SANDBOXED_TOP_LEVEL_NAVIGATION_WITH_USER_ACTIVATION_BROWSING_CONTEXT_FLAG,
            // The sandboxed origin browsing context flag, unless the tokens contains the allow-same-origin keyword.
            "allow-same-origin" =>
                SandboxingFlagSet::SANDBOXED_ORIGIN_BROWSING_CONTEXT_FLAG,
            // The sandboxed forms browsing context flag, unless tokens contains the allow-forms keyword.
            "allow-forms" =>
                SandboxingFlagSet::SANDBOXED_FORMS_BROWSING_CONTEXT_FLAG,
            // The sandboxed pointer lock browsing context flag, unless tokens contains the allow-pointer-lock keyword.
            "allow-pointer-lock" =>
                SandboxingFlagSet::SANDBOXED_POINTER_LOCK_BROWSING_CONTEXT_FLAG,
            // The sandboxed scripts browsing context flag, unless tokens contains the allow-scripts keyword.
            // The sandboxed automatic features browsing context flag, unless tokens contains the allow-scripts keyword (defined above).
            "allow-scripts" =>
                SandboxingFlagSet::SANDBOXED_SCRIPTS_BROWSING_CONTEXT_FLAG |
                    SandboxingFlagSet::SANDBOXED_AUTOMATIC_FEATURES_BROWSING_CONTEXT_FLAG,
            // The sandbox propagates to auxiliary browsing contexts flag, unless tokens contains the allow-popups-to-escape-sandbox keyword.
            "allow-popups-to-escape-sandbox" =>
                SandboxingFlagSet::SANDBOX_PROPOGATES_TO_AUXILIARY_BROWSING_CONTEXTS_FLAG,
            // The sandboxed modals flag, unless tokens contains the allow-modals keyword.
            "allow-modals" =>
                SandboxingFlagSet::SANDBOXED_MODALS_FLAG,
            // The sandboxed orientation lock browsing context flag, unless tokens contains the allow-orientation-lock keyword.
            "allow-orientation-lock" =>
                SandboxingFlagSet::SANDBOXED_ORIENTATION_LOCK_BROWSING_CONTEXT_FLAG,
            // The sandboxed presentation browsing context flag, unless tokens contains the allow-presentation keyword.
            "allow-presentation" =>
                SandboxingFlagSet::SANDBOXED_PRESENTATION_BROWSING_CONTEXT_FLAG,
            // The sandboxed downloads browsing context flag, unless tokens contains the allow-downloads keyword.
            "allow-downloads" => SandboxingFlagSet::SANDBOXED_DOWNLOADS_BROWSING_CONTEXT_FLAG,
            // The sandboxed custom protocols navigation browsing context flag, unless tokens contains either the
            // allow-top-navigation-to-custom-protocols keyword, the allow-popups keyword, or the allow-top-navigation keyword.
            "allow-top-navigation-to-custom-protocols" => SandboxingFlagSet::SANBOXED_CUSTOM_PROTOCOLS_NAVIGATION_BROWSING_CONTEXT_FLAG,
            _ =>
                SandboxingFlagSet::empty(),
        };
        output.remove(remove);
    }
    output
}
