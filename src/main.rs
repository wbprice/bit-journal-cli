use chrono::Local;
use termion::{clear, color, style};
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, stdout, Stdout};

#[derive(Debug, Default, Clone)]
struct Event {
    important: bool,
    content: String,
}

#[derive(Debug, Default, Clone)]
struct Task {
    important: bool,
    completed: bool,
    cancelled: bool,
    content: String,
}

#[derive(Debug, Default, Clone)]
struct Note {
    important: bool,
    content: String,
}

#[derive(Debug)]
enum Entries {
    Event(Event),
    Task(Task),
    Note(Note)
}

trait Journalable {
    fn render(&self) -> String;
    fn toggle_completed(&self) -> Entries;
    fn set_content(&self, content: String) -> Entries;
}

impl Journalable for Entries {
    fn render(&self) -> String {
        match self {
            Entries::Event(item) => format!("{important} {symbol} {content}",
                important = if item.important { "*" } else {" "},
                symbol = "\u{26AC}",
                content = item.content),
            Entries::Task(item) => format!("{important} {symbol} {content}",
                important = if item.important { "*" } else {" "},
                symbol = if item.completed { "X" } else { "\u{2022}" },
                content = item.content),
            Entries::Note(item) => format!("{important} {symbol} {content}",
                important = if item.important { "*" } else {" "},
                symbol = "-",
                content = item.content),
        }
    }

    fn toggle_completed(&self) -> Entries {
        match self {
            Entries::Task(item) => Entries::Task(Task {
                completed: !item.completed,
                ..item.clone()
            }),
            Entries::Note(note) => Entries::Note(Note {
                ..note.clone()
            }),
            Entries::Event(event) => Entries::Event(Event {
                ..event.clone()
            })
        }
    }

    fn set_content(&self, content: String) -> Entries {
        match self {
            Entries::Task(item) => Entries::Task(Task {
                content,
                ..item.clone()
            }),
            Entries::Note(note) => Entries::Note(Note {
                content,
                ..note.clone()
            }),
            Entries::Event(event) => Entries::Event(Event {
                content,
                ..event.clone()
            })
        }
    }
}

#[derive(Debug)]
struct Application {
    entries: Vec<Entries>,
    mode: Modes
}

impl Application {
    fn render_status_bar(&self, stdout : &mut RawTerminal<Stdout>) {
        writeln!(stdout, "{}", clear::All).unwrap();
        writeln!(
            stdout,
            "{green}Bit Journal v0.1.0{reset}\n",
            green = color::Fg(color::Green),
            reset = color::Fg(color::Reset)
        ).unwrap();
        writeln!(
            stdout,
            "{yellow}Today is {bold}{date}.{reset}\n",
            yellow = color::Fg(color::Yellow),
            bold = style::Bold,
            date = Local::now().format("%a, %b %e").to_string(),
            reset = color::Fg(color::Reset)
        ).unwrap();
    }

    fn render_tasks(&self, stdout: &mut RawTerminal<Stdout>) {
        for entry in self.entries.iter() {
            writeln!(stdout, "{}\n", entry.render()).unwrap();
        }
    }

    fn render_header_bar(&self, stdout: &mut RawTerminal<Stdout>) {
        writeln!(
            stdout,
            "{background}{white}The current mode is {mode}{reset}\n",
            background = color::Bg(color::Green),
            white = color::Fg(color::White),
            mode = self.mode.render(),
            reset = color::Fg(color::Reset)).unwrap();
    }

    fn render(&self) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        self.render_status_bar(&mut stdout);
        self.render_tasks(&mut stdout);
        self.render_header_bar(&mut stdout);
    }
}

#[derive(Debug)]
enum Modes {
    Normal,
    Insert
}

impl Modes {
    fn render(&self) -> String {
        match self {
            Modes::Normal => "Normal".to_string(),
            Modes::Insert => "Insert".to_string()
        }
    }
}

fn main() {

    let application = Application {
        mode: Modes::Normal,
        entries: vec![
            Entries::Event(Event {
                content: "Internal Standup at 4pm".into(),
                ..Default::default()
            }),
            Entries::Task(Task {
                content: "Figure out enums".into(),
                ..Default::default()
            }),
            Entries::Task(Task {
                content: "Take out the trash".into(),
                ..Default::default()
            })
            .toggle_completed()
            .set_content("Laugh uncontrollably".into()),
            Entries::Note(Note {
                content: "I'm just surprised this worked".into(),
                ..Default::default()
            }),
        ]
    };

    application.render();
}
