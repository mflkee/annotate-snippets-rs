use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"This is example of two annotations spanning
multiple lines with the exact same byte range."#;

    // Both annotations span the exact same byte range.
    let span = 35..58;

    let input = &[Level::ERROR.primary_title("attention!").element(
        Snippet::source(source)
            .path("foo.txt")
            .annotation(
                AnnotationKind::Primary
                    .span(span.clone())
                    .label("primary label"),
            )
            .annotation(
                AnnotationKind::Context
                    .span(span.clone())
                    .label("context label"),
            ),
    )];

    let expected_ascii = file!["ann_multiline_overlapping.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = file!["ann_multiline_overlapping.unicode.term.svg": TermSvg];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}
