use crate::app::{App, Focus};
use crate::braille;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

const SIDEBAR_WIDTH: u16 = 22;

/// Max scroll for help content (generous to account for text wrapping on small screens)
pub const HELP_CONTENT_LINES: u16 = 60;

// UI color scheme
const BORDER_COLOR: Color = Color::Cyan;
const HIGHLIGHT_COLOR: Color = Color::Yellow;
const TEXT_COLOR: Color = Color::White;
const DIM_TEXT_COLOR: Color = Color::Gray;

/// Creates a standard styled block with rounded borders
fn styled_block(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER_COLOR))
        .title(title)
}

/// Main render function
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    if app.fullscreen_mode {
        render_canvas(frame, area, app);
    } else {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(SIDEBAR_WIDTH), Constraint::Min(0)])
            .split(area);

        render_sidebar(frame, layout[0], app);
        render_canvas(frame, layout[1], app);
    }

    if app.show_help {
        render_help_overlay(frame, area, app);
    }
}

/// Calculate the canvas size (excluding borders)
pub fn get_canvas_size(frame_area: Rect, fullscreen: bool) -> (u16, u16) {
    if fullscreen {
        (frame_area.width.saturating_sub(2), frame_area.height.saturating_sub(2))
    } else {
        let canvas_width = frame_area.width.saturating_sub(SIDEBAR_WIDTH + 2);
        let canvas_height = frame_area.height.saturating_sub(2);
        (canvas_width, canvas_height)
    }
}

fn render_sidebar(frame: &mut Frame, area: Rect, app: &App) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Status
            Constraint::Length(10), // Parameters (added extra line)
            Constraint::Min(10),    // Controls
        ])
        .split(area);

    render_status_box(frame, sections[0], app);
    render_params_box(frame, sections[1], app);
    render_controls_box(frame, sections[2], app);
}

fn render_status_box(frame: &mut Frame, area: Rect, app: &App) {
    let block = styled_block(" DLA Simulator ");

    let progress = app.simulation.progress();
    let progress_width = (area.width.saturating_sub(4)) as usize;
    let filled = (progress * progress_width as f32) as usize;
    let empty = progress_width.saturating_sub(filled);

    let status_text = if app.simulation.paused {
        "PAUSED"
    } else if app.simulation.is_complete() {
        "COMPLETE"
    } else {
        "RUNNING"
    };

    let status_color = if app.simulation.paused {
        HIGHLIGHT_COLOR
    } else if app.simulation.is_complete() {
        Color::Green
    } else {
        BORDER_COLOR
    };

    let content = vec![
        Line::from(vec![
            Span::styled(
                format!("{} / {}", app.simulation.particles_stuck, app.simulation.num_particles),
                Style::default().fg(TEXT_COLOR),
            ),
        ]),
        Line::from(vec![
            Span::styled("█".repeat(filled), Style::default().fg(Color::Green)),
            Span::styled("░".repeat(empty), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(Span::styled(status_text, Style::default().fg(status_color))),
    ];

    let paragraph = Paragraph::new(content).block(block);
    frame.render_widget(paragraph, area);
}

fn render_params_box(frame: &mut Frame, area: Rect, app: &App) {
    let block = styled_block(" Parameters ");

    let make_line = |label: &str, value: String, focused: bool| {
        let prefix = if focused { "> " } else { "  " };
        let style = if focused {
            Style::default().fg(HIGHLIGHT_COLOR)
        } else {
            Style::default().fg(TEXT_COLOR)
        };
        Line::from(Span::styled(format!("{}{}: {}", prefix, label, value), style))
    };

    let settings = &app.simulation.settings;

    let content = vec![
        make_line(
            "Sticky",
            format!("{:.2}", app.simulation.stickiness),
            app.focus == Focus::Stickiness,
        ),
        make_line(
            "Particles",
            format!("{}", app.simulation.num_particles),
            app.focus == Focus::Particles,
        ),
        make_line(
            "Seed",
            app.simulation.seed_pattern.name().to_string(),
            app.focus == Focus::Seed,
        ),
        make_line(
            "Color",
            app.color_scheme.name().to_string(),
            app.focus == Focus::ColorScheme,
        ),
        make_line(
            "Speed",
            format!("{}", app.steps_per_frame),
            app.focus == Focus::Speed,
        ),
        Line::from(Span::styled(
            format!("  Mode: {}", settings.color_mode.name()),
            Style::default().fg(DIM_TEXT_COLOR),
        )),
        Line::from(Span::styled(
            format!("  Step: {:.1}", settings.walk_step_size),
            Style::default().fg(DIM_TEXT_COLOR),
        )),
    ];

    let paragraph = Paragraph::new(content).block(block);
    frame.render_widget(paragraph, area);
}

fn render_controls_box(frame: &mut Frame, area: Rect, app: &App) {
    let block = styled_block(" Controls ");

    let key_style = Style::default().fg(HIGHLIGHT_COLOR);
    let desc_style = Style::default().fg(DIM_TEXT_COLOR);

    let make_control = |key: &str, desc: &str| {
        Line::from(vec![
            Span::styled(format!("{:>6}", key), key_style),
            Span::styled(format!(" {}", desc), desc_style),
        ])
    };

    let settings = &app.simulation.settings;

    let mut content = vec![
        make_control("Space", "pause/resume"),
        make_control("H/?", "help"),
        make_control("R", "reset"),
        make_control("1-0", "seed patterns"),
        make_control("C", "color scheme"),
        make_control("M", &format!("mode: {}", settings.color_mode.name())),
        make_control("N", &format!("nbr: {}", settings.neighborhood.short_name())),
        make_control("B", &format!("bnd: {}", settings.boundary_behavior.name())),
        make_control("S", &format!("spn: {}", settings.spawn_mode.name())),
        make_control("W/E", &format!("step: {:.1}", settings.walk_step_size)),
        make_control("Q", "quit"),
    ];

    // Show current focus hint
    if app.focus != Focus::None {
        content.push(Line::from(""));
        content.push(Line::from(Span::styled(
            format!("Editing: {:?}", app.focus),
            Style::default().fg(HIGHLIGHT_COLOR),
        )));
    }

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn render_canvas(frame: &mut Frame, area: Rect, app: &App) {
    let block = styled_block("");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Get settings for rendering
    let settings = &app.simulation.settings;

    // Render Braille pattern (uses LUT for fast color lookup)
    let cells = braille::render_to_braille(
        &app.simulation,
        inner.width,
        inner.height,
        &app.color_lut,
        app.color_by_age,
        settings.color_mode,
        settings.highlight_recent,
        settings.invert_colors,
    );

    for cell in cells {
        let x = inner.x + cell.x;
        let y = inner.y + cell.y;

        if x < inner.x + inner.width && y < inner.y + inner.height {
            let cell_rect = Rect {
                x,
                y,
                width: 1,
                height: 1,
            };
            let span = Span::styled(cell.char.to_string(), Style::default().fg(cell.color));
            let paragraph = Paragraph::new(Line::from(span));
            frame.render_widget(paragraph, cell_rect);
        }
    }
}

fn render_help_overlay(frame: &mut Frame, area: Rect, app: &App) {
    // Calculate the canvas area (exclude sidebar unless fullscreen)
    let canvas_x = if app.fullscreen_mode { 0 } else { SIDEBAR_WIDTH };
    let canvas_width = if app.fullscreen_mode {
        area.width
    } else {
        area.width.saturating_sub(SIDEBAR_WIDTH)
    };

    // Center the help dialog within the canvas
    let help_width = 56.min(canvas_width.saturating_sub(4));
    let help_height = area.height.saturating_sub(4).min(40);
    let x = canvas_x + (canvas_width.saturating_sub(help_width)) / 2;
    let y = (area.height.saturating_sub(help_height)) / 2;

    let help_area = Rect {
        x: area.x + x,
        y: area.y + y,
        width: help_width,
        height: help_height,
    };

    // Clear the background
    frame.render_widget(Clear, help_area);

    // Build expanded help content (formatted for wrapping)
    let content = vec![
        Line::from(""),
        Line::from(Span::styled("DIFFUSION-LIMITED AGGREGATION", Style::default().fg(BORDER_COLOR))),
        Line::from(""),
        Line::from("Particles randomly walk until they touch and stick to the growing structure, creating fractal patterns."),
        Line::from(""),
        Line::from(Span::styled("SEED PATTERNS (1-0):", Style::default().fg(HIGHLIGHT_COLOR))),
        Line::from("1=Point, 2=Line, 3=Cross, 4=Circle, 5=Ring, 6=Block, 7=Multi, 8=Starburst, 9=Noise, 0=Scatter"),
        Line::from(""),
        Line::from(Span::styled("ADVANCED SETTINGS:", Style::default().fg(HIGHLIGHT_COLOR))),
        Line::from(""),
        Line::from(Span::styled("M - Color Mode", Style::default().fg(TEXT_COLOR))),
        Line::from("Age (when stuck), Distance (from center), Density (neighbor count), Direction (approach angle)"),
        Line::from(""),
        Line::from(Span::styled("N - Neighborhood", Style::default().fg(TEXT_COLOR))),
        Line::from("VonNeumann (4): angular patterns"),
        Line::from("Moore (8): natural fractals"),
        Line::from("Extended (24): dense blobs"),
        Line::from(""),
        Line::from(Span::styled("B - Boundary Behavior", Style::default().fg(TEXT_COLOR))),
        Line::from("Clamp (stop), Wrap (toroidal), Bounce (reflect), Stick (edges), Absorb (respawn)"),
        Line::from(""),
        Line::from(Span::styled("S - Spawn Mode", Style::default().fg(TEXT_COLOR))),
        Line::from("Circle, Edges, Corners, Random, Top, Bottom, Left, Right"),
        Line::from(""),
        Line::from(Span::styled("W/E - Walk Step Size", Style::default().fg(TEXT_COLOR))),
        Line::from("Larger = faster but coarser, Smaller = slower but finer detail"),
        Line::from(""),
        Line::from(Span::styled("I - Invert Colors", Style::default().fg(TEXT_COLOR))),
        Line::from(Span::styled("[/] - Highlight Recent", Style::default().fg(TEXT_COLOR))),
        Line::from("Show newest particles in white"),
        Line::from(""),
        Line::from(Span::styled("BASIC CONTROLS:", Style::default().fg(HIGHLIGHT_COLOR))),
        Line::from("Space=Pause, R=Reset, C=Colors, A=Color-by-age, V=Fullscreen, Tab/Arrows=Adjust, +/-=Speed, Q=Quit"),
        Line::from(""),
    ];

    let content_height = content.len() as u16;
    let visible_height = help_height.saturating_sub(2); // minus borders
    let max_scroll = content_height.saturating_sub(visible_height);
    let is_scrollable = max_scroll > 0;

    // Update title to show scroll hint if scrollable
    let title = if is_scrollable {
        " Help (↑↓ scroll, H to close) "
    } else {
        " Help (H to close) "
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(HIGHLIGHT_COLOR))
        .title(title);

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: true })
        .scroll((app.help_scroll, 0));

    frame.render_widget(paragraph, help_area);
}
