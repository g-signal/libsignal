//
// Copyright 2024 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

const SAFE_DOMAIN_SUFFIXES: &[&str] = &[".imba-test.com", ".ba-chat.com"];

pub(crate) fn log_safe_domain(domain: &str) -> &str {
    match domain {
        "localhost" => domain,
        d if SAFE_DOMAIN_SUFFIXES.iter().any(|s| d.ends_with(s)) => d,
        _ => "REDACTED",
    }
}
