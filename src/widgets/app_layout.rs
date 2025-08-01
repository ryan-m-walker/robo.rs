use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::{
    state::{AppState, View},
    widgets::{
        header::Header,
        nav_tabs::NavTabs,
        status_line::StatusLine,
        views::{chat::ChatViewWidget, home::HomeViewWidget, logs::LogsViewWidget},
    },
};

pub struct AppLayout<'a> {
    state: &'a AppState,
}

impl<'a> AppLayout<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

impl Widget for AppLayout<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(area);

        Header::new(self.state).render(layout[0], buf);
        NavTabs::new(self.state).render(layout[1], buf);

        match self.state.view {
            View::Home => HomeViewWidget::new(self.state).render(layout[2], buf),
            View::Logs => LogsViewWidget::new(self.state).render(layout[2], buf),
            View::Chat => ChatViewWidget::new(self.state).render(layout[2], buf),
        }

        StatusLine::new(self.state).render(layout[3], buf);
    }
}
