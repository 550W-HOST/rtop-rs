use ratatui::layout::{Constraint, Direction, Layout};

use crate::widgets::barchart::BarChart;

/// Renders the user interface widgets.
pub fn render(app: &mut crate::app::App, frame: &mut ratatui::Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(frame.size());
    let data = vec![("B", 10), ("C", 20), ("D", 30), ("E", 40), ("F", 50)];
    let chart = BarChart::default().data(&data).bar_width(1);
    frame.render_widget(chart, main_layout[1]);
}
