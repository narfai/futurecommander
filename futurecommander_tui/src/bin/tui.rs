// use futures::stream::iter;
use std::io::{Write, stdout, stdin};
use std::path::{Path, PathBuf};

use tonic::Request;
use futures::stream;
use math::round;

use termion::{clear, cursor, color, style};
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;

use futurecommander_tui::components::{     
    Bordered,
    TextList
};

use futurecommander_tui::position::{ 
    Area,
    Position
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w,_)| w);
    let termheight = termsize.map(|(_,h)| h);

    let stdin = stdin();

    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();


    if let Some(width) = termwidth {
        if let Some(height) = termheight {            
            let render_area = Area {
                left_top: Position { x: 0, y: 0 },
                right_bottom: Position { x: width, y: height }
            };
            
            /*let b  = Bordered::new();
            let t = TextList::new(vec![String::from("testA"), String::from("testB")]);

            for ord in 0..height {
                for abs in 0..width {                    
                    let p = Position { x: abs, y: ord };
                    let to_display = if let Some(s) = b.render_at_pos(p, &render_area.inside(1)) {
                        s
                    } else if let Some(s) = t.render_at_pos(p, &render_area.inside(2)){
                        s
                    } else {
                        String::from(" ")
                    };
                    write!(stdout, "{}", to_display);      
                }
            } */
        }
    }

    stdout.flush().unwrap();

    Ok(())
    //a module is a char decider : module(pos: Position)
}
/*
Menu
    MenuGroup
        MenuEntry
ExplorerPanel
    PathNavBar
    NodeTree
        File
        Dir
ActionMenu
    MenuEntry
*/
