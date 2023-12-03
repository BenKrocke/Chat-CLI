use std::{
    io::{self, BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};


fn main() {
    stdout().execute(EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    terminal.clear().unwrap();

    loop {
	terminal.draw(|frame| {
	    let area = frame.size();
	    frame.render_widget(
		Paragraph::new("Hello Ratatui! (press 'q' to quit)")
		    .white()
		    .on_blue(),
		area,
	    )
	}).unwrap();
	if event::poll(std::time::Duration::from_millis(16)).unwrap() {
	    if let event::Event::Key(key) = event::read().unwrap() {
		if key.kind == KeyEventKind::Press
		    && key.code == KeyCode::Char('q') {
			break;
		    }
	    }
	}
    }

    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

fn old_main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(stream);
            }
            Err(e) => {
                println!("Connection failed. Error: {}", e);
            }
        }
    }
}

fn handle_stream(s: TcpStream) {
    let mut reader = BufReader::new(s);

    let rh = thread::spawn(move || {
        loop {
            let mut s = String::new();
            let received = reader.read_line(&mut s).unwrap();
            if received == 0 {
                break;
            }
            print!("{}", s);
            s.clear();
        }

        println!("Connection closed.");
    });

    let sh = thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        print!("About to send: {}", buffer);
    });

    rh.join().expect("rh join failed");
    sh.join().expect("sh join failed");
}
