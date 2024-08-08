use log::info;
use log::LevelFilter;
use std::borrow::BorrowMut;
use std::io;
use std::sync::{Arc, Mutex};
use tui::backend::{Backend, CrosstermBackend};
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::{Block, Borders, StatefulWidget, Widget};
use tui::Terminal;

use tui_log::{TuiLogger, Writable};

use gnostr_bins::get_blockheight;
use gnostr_bins::get_weeble;
use gnostr_bins::get_wobble;

#[derive(Default, Clone)]
pub struct LogWidgetState {
    pub history: Vec<String>,
}

#[derive(Default, Clone)]
pub struct LogWidget {}

impl StatefulWidget for LogWidget {
    type State = LogWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let max_lines = area.height - 1;

        let history_to_show = state.history.iter().rev().take(max_lines as usize).rev();

        for (y, line) in history_to_show.enumerate() {
            buf.set_string(area.left(), area.top() + y as u16, line, Style::default());
        }
    }
}

impl Widget for LogWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self, area, buf, &mut LogWidgetState::default())
    }
}

impl Writable for LogWidgetState {
    fn write_line(&mut self, message: &str) {
        self.history.push(message.to_string())
    }

    fn flush(&mut self) {
        self.history.clear()
    }
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let state = Arc::new(Mutex::new(LogWidgetState::default()));
    TuiLogger::init(LevelFilter::Info, state.clone()).expect("Could not init logger");

    terminal.clear().expect("Could not clear terminal");
    let mut i = 0;
    let mut weeble: String = get_weeble().unwrap();
    let mut blockheight: String = get_blockheight().unwrap();
    let mut wobble: String = get_wobble().unwrap();
    info!("{:}/{:}/{:}", weeble, blockheight, wobble);
    loop {
        draw(&mut terminal, state.clone())?;

        let loop_weeble = get_weeble().unwrap();
        if loop_weeble.parse::<i32>().unwrap_or(i32::MAX) >= weeble.parse::<i32>().unwrap_or(0)
            && i > 0
        {
            //info!(
            //    "{}:{} >= {}",
            //    (loop_weeble.parse::<i32>().unwrap_or(i32::MAX)
            //        >= weeble.parse::<i32>().unwrap_or(0)),
            //    loop_weeble,
            //    weeble
            //);
            weeble = loop_weeble.clone();
            info!(
                "{:}/{:}/{:}",
                weeble,
                get_blockheight().unwrap(),
                get_wobble().unwrap()
            );
        }
        draw(&mut terminal, state.clone())?;

        let loop_blockheight = get_blockheight().unwrap();
        if loop_blockheight.parse::<i32>().unwrap_or(i32::MAX)
            >= blockheight.parse::<i32>().unwrap_or(0)
            && i > 0
        {
            //info!(
            //    "{}:{} >= {}",
            //    (loop_blockheight.parse::<i32>().unwrap_or(i32::MAX)
            //        >= blockheight.parse::<i32>().unwrap_or(0)),
            //    loop_weeble,
            //    weeble
            //);
            blockheight = loop_blockheight.clone();
            info!("{:}/{:}/{:}", weeble, blockheight, get_wobble().unwrap());
        }
        draw(&mut terminal, state.clone())?;
        let loop_wobble = get_wobble().unwrap();
        if loop_wobble.parse::<i32>().unwrap_or(i32::MAX) >= wobble.parse::<i32>().unwrap_or(0)
            && i > 0
        {
            //info!(
            //    "{}:{} >= {}",
            //    (loop_wobble.parse::<i32>().unwrap_or(i32::MAX)
            //        >= wobble.parse::<i32>().unwrap_or(0)),
            //    loop_wobble,
            //    wobble
            //);
            wobble = loop_wobble.clone();
            info!("{:}/{:}/{:}", weeble, blockheight, get_wobble().unwrap());
        }
        draw(&mut terminal, state.clone())?;

        //terminal.clear().expect("Could not clear terminal");
        //draw(&mut terminal, state.clone())?;
        //info!("count={}", i);
        info!(
            "{:}/{:}/{:}",
            get_weeble().unwrap(),
            get_blockheight().unwrap(),
            get_wobble().unwrap()
        );
        //let padded_hash: String = String::from(format!("{}{}{}",get_weeble().unwrap(),get_blockheight().unwrap(), get_wobble().unwrap()));
        //info!("{:0>64}",padded_hash);
        i += 1;
        //info!("count={}", i);
        //terminal.clear().expect("Could not clear terminal");
    }
}

fn draw<B: Backend>(
    terminal: &mut Terminal<B>,
    log_widget_state: Arc<Mutex<LogWidgetState>>,
) -> io::Result<()> {
    terminal.draw(|f| {
        let title: String = String::from(format!(
            "──[\"GNOSTR\",{{\"weeble\": {}, \"blockheight\": {}, \"wobble\": {}}}]",
            get_weeble().unwrap(),
            get_blockheight().unwrap(),
            get_wobble().unwrap()
        ));
        let titles = ["Tab1", "Tab2", "Tab3", "Tab4"]
            .iter()
            .cloned()
            .map(tui::text::Spans::from)
            .collect();
        let tabs = tui::widgets::Tabs::new(titles)
            //.block(Block::default().title(title).borders(Borders::ALL))
            .style(tui::style::Style::default().fg(tui::style::Color::White))
            .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow))
            .divider(tui::symbols::DOT);
        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(f.size());

        f.render_widget(tabs, v_chunks[0]);

        //let block = Block::default().title(title).borders(Borders::ALL);
        //f.render_widget(block, chunks[0]);

        //
        let log_title: String = String::from(format!(
            "──[\"GNOSTR\",{{\"weeble\": {}, \"blockheight\": {}, \"wobble\": {}}}]",
            get_weeble().unwrap(),
            get_blockheight().unwrap(),
            get_wobble().unwrap()
        ));
        let block = Block::default().title(log_title).borders(Borders::ALL);
        f.render_widget(block, v_chunks[1]);

        //
        let inset_area = edge_inset(&v_chunks[1], 1);
        let log_widget = LogWidget::default();
        f.render_stateful_widget(
            log_widget,
            inset_area,
            log_widget_state.lock().unwrap().borrow_mut(),
        );
    })?; //end terminal.draw

    Ok(())
}

fn edge_inset(area: &Rect, margin: u16) -> Rect {
    let mut inset_area = *area;
    inset_area.x += margin;
    inset_area.y += margin;
    inset_area.height -= margin;
    inset_area.width -= margin;

    inset_area
}
