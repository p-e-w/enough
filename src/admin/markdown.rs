// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use pulldown_cmark::{html::push_html, Options, Parser};

pub(super) fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all());

    let mut html = String::new();
    push_html(&mut html, parser);

    html
}
