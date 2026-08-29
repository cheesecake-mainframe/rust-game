use ratatui::prelude::*;
use ratatui::widgets::*;

use super::super::ui::TuiApp;

pub fn render(frame: &mut Frame, tui: &TuiApp, module_id: &str) {
    let area = frame.area();

    let module = match tui.app.catalog.get_module(module_id) {
        Some(m) => m,
        None => {
            frame.render_widget(Paragraph::new("Module not found"), area);
            return;
        }
    };

    let lesson = match &tui.current_lesson {
        Some(l) => l,
        None => {
            let hint = module
                .book_url
                .as_deref()
                .unwrap_or("https://doc.rust-lang.org/book/");
            let msg = format!(
                "  No lesson has been written for {} yet.\n\n  Read the Rust Book chapter instead:\n  {}\n\n  [Esc] Back",
                module.name, hint
            );
            frame.render_widget(
                Paragraph::new(msg)
                    .block(Block::bordered().title(format!(" {} ", module.theme_name)))
                    .wrap(Wrap { trim: false }),
                area,
            );
            return;
        }
    };

    let outer = Layout::vertical([
        Constraint::Length(5), // Header: title + wrapped concepts
        Constraint::Min(4),    // Lesson body
        Constraint::Length(3), // Footer
    ])
    .split(area);

    // ─── Header ───────────────────────────────────────────
    // Show the module's plain name here; the border already carries the theme
    // name. Using the lesson's own h1 would print the same words twice whenever
    // an author titles the lesson after the theme.
    let mut header_lines = vec![Line::from(vec![
        Span::styled("  ", Style::new()),
        Span::styled(module.name.as_str(), Style::new().fg(Color::Cyan).bold()),
    ])];
    let concepts = lesson.concepts_line();
    if !concepts.is_empty() {
        header_lines.push(Line::from(vec![
            Span::styled("  Covers: ", Style::new().fg(Color::DarkGray)),
            Span::styled(concepts, Style::new().fg(Color::Gray)),
        ]));
    }
    frame.render_widget(
        Paragraph::new(header_lines)
            .block(Block::bordered().title(format!(" {} ", module.theme_name)))
            .wrap(Wrap { trim: false }),
        outer[0],
    );

    // ─── Body ─────────────────────────────────────────────
    let body_block = Block::bordered().title(" Lesson ");
    let inner = body_block.inner(outer[1]);

    // The header already shows the title, so drop the body's leading `# ...`
    // line to avoid printing it twice.
    let body = strip_leading_h1(&lesson.body);
    let text = tui_markdown::from_str(body);
    let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });

    // Clamp the scroll to the last useful offset. `line_count` accounts for
    // wrapping at the real render width, which the key handler cannot know —
    // so the renderer computes it and stores it for the handler to read.
    let total = paragraph.line_count(inner.width);
    let max_scroll = total.saturating_sub(inner.height as usize).min(u16::MAX as usize) as u16;
    tui.lesson_max_scroll.set(max_scroll);
    let offset = tui.lesson_scroll.min(max_scroll);

    frame.render_widget(
        paragraph.block(body_block).scroll((offset, 0)),
        outer[1],
    );

    // ─── Footer ───────────────────────────────────────────
    let read = tui.app.state.is_lesson_read(module_id);
    let position = if max_scroll == 0 {
        "all".to_string()
    } else {
        format!("{}%", (offset as usize * 100) / max_scroll as usize)
    };
    let footer_text = if read {
        format!(" [j/k] Scroll ({})  [Esc] Back  [q] Quit   |   already read", position)
    } else {
        format!(" [j/k] Scroll ({})  [m] Mark read  [Esc] Back  [q] Quit", position)
    };
    frame.render_widget(
        Paragraph::new(footer_text)
            .block(Block::bordered())
            .style(Style::new().fg(Color::DarkGray)),
        outer[2],
    );
}

/// Drop the leading `# Heading` line (and the blank line after it) from a
/// lesson body. The screen header already displays the title, so rendering it
/// again at the top of the body just wastes a line and reads as a duplicate.
fn strip_leading_h1(body: &str) -> &str {
    let trimmed = body.trim_start_matches('\n');
    if !trimmed.starts_with("# ") {
        return body;
    }
    match trimmed.find('\n') {
        Some(idx) => trimmed[idx + 1..].trim_start_matches('\n'),
        None => "",
    }
}

#[cfg(test)]
mod tests {
    use super::strip_leading_h1;

    #[test]
    fn strips_the_title_line_and_following_blank() {
        let body = "# The First Steps\n\nThe first Rust program...\n";
        assert_eq!(strip_leading_h1(body), "The first Rust program...\n");
    }

    #[test]
    fn leaves_a_body_with_no_leading_heading_alone() {
        let body = "Just prose.\n\n## A later heading\n";
        assert_eq!(strip_leading_h1(body), body);
    }

    #[test]
    fn does_not_strip_a_deeper_heading() {
        let body = "## Not an h1\n\ntext\n";
        assert_eq!(strip_leading_h1(body), body);
    }
}
