// use futures::stream::iter;
use std::io::{Write, stdout, stdin};
use std::path::{Path, PathBuf};

use tonic::Request;
use futures::stream;

use termion::{clear, cursor, color, style};
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;

use futurecommander_proto::vfs::{
    virtual_file_system_client::{VirtualFileSystemClient},
    Entry,
//    ListDirectoryRequest,
//    ListDirectoryResponse,
//    CreateNodeRequest,
//    CreateNodeResponse,
    RemoveNodeRequest,
    RemoveNodeResponse,
//    CopyNodeRequest,
//    CopyNodeResponse,
//    MoveNodeRequest,
//    MoveNodeResponse,
    RequestStatus,
    ResponseStatus,
};

const HORZ_BOUNDARY: &'static str = "─";
const VERT_BOUNDARY: &'static str = "│";
const TOP_LEFT_CORNER: &'static str = "┌";
const TOP_RIGHT_CORNER: &'static str = "┐";
const BOTTOM_LEFT_CORNER: &'static str = "└";
const BOTTOM_RIGHT_CORNER: &'static str = "┘";

fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn identity<T>(t: T) -> T { t }

#[derive(Clone, Copy, Debug)]
struct Position {
    pub x: u16,
    pub y: u16
}

impl Position {
    fn to_tuple(&self) -> (u16, u16){
        (self.x, self.y)
    }

    fn is_contained_by(&self, rectangle: &Rectangle) -> bool {        
        self.x >= rectangle.from.x && self.x <= rectangle.to.x && self.y >= rectangle.from.y && self.y <= rectangle.to.y
    }
}

#[derive(Debug)]
struct Rectangle {
    pub from: Position,
    pub to: Position
}

enum CellChoice {
    Skip(SkipCell),
    Render(RenderCell)
}

struct SkipCell {
    pub position: Position
}

struct RenderCell {
    pub position: Position,
    pub content: String
}

impl SkipCell {
    pub fn choose_to_skip(position: Position) -> CellChoice {
        CellChoice::Skip(
            SkipCell { 
                position 
            }
        )
    }

    pub fn choose_to_render(self, content: &str) -> CellChoice {
        CellChoice::Render(
            RenderCell {
                position: self.position,
                content: String::from(content)
            }
        )
    }
}

fn box_router(rectangle: Rectangle) -> impl Fn(CellChoice) -> CellChoice {    
    move |cellchoice| match cellchoice {
        CellChoice::Render(_) => cellchoice,
        CellChoice::Skip(cell) => { 
            let position = cell.position;
            if position.is_contained_by(&rectangle) {                
                match position.to_tuple() {                    
                    (x, y) if x == rectangle.from.x && y == rectangle.from.y => cell.choose_to_render(TOP_LEFT_CORNER),
                    (x, y) if x == rectangle.to.x && y == rectangle.from.y => cell.choose_to_render(TOP_RIGHT_CORNER),
                    (x, y) if x == rectangle.to.x && y == rectangle.to.y => cell.choose_to_render(BOTTOM_RIGHT_CORNER),
                    (x, y) if x == rectangle.from.x && y == rectangle.to.y => cell.choose_to_render(BOTTOM_LEFT_CORNER),
                    (x, _) if x == rectangle.from.x || x == rectangle.to.x => cell.choose_to_render(VERT_BOUNDARY),                        
                    (_, y) if y == rectangle.from.y || y == rectangle.to.y => cell.choose_to_render(HORZ_BOUNDARY),                        
                    _ => CellChoice::Skip(cell)
                }                                    
            } else {
                CellChoice::Skip(cell)
            }            
        }
    }
}

fn render<W: Write> (out: &mut W, rectangle: Rectangle, cell_router: &dyn Fn(CellChoice) -> CellChoice){    
    for x in rectangle.from.x..rectangle.to.x {
        for y in rectangle.from.y..rectangle.to.y {            
            match cell_router(SkipCell::choose_to_skip(Position{x, y})) {
                CellChoice::Render(cell) => { write!(out, "{}{}", cursor::Goto(x + 1, y + 1), cell.content).unwrap(); }
                CellChoice::Skip(_) => { write!(out, "{}{}", cursor::Goto(x + 1, y + 1), "A").unwrap(); }
                _ => {}
            };
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let mut client = VirtualFileSystemClient::new(channel);

    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w,_)| w);
    let termheight = termsize.map(|(_,h)| h);

    println!("{:?}", termsize);
    println!("{:?}", termwidth);
    println!("{:?}", termheight);

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();

    // let panel = ExplorerPanel::default();

    for c in stdin.events() {
        let event = c.unwrap();
        match event {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        // write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                        if let Some(x) = termwidth {
                            if let Some(y) = termheight{                                
                                render(&mut stdout, 
                                    Rectangle {
                                        from: Position { x: 0, y: 0 },
                                        to: Position { x, y },
                                    },                                    
                                    &compose(
                                        box_router(
                                            Rectangle {
                                                from: Position { x: 2, y: 2 },
                                                to: Position { x: x - 3, y: y - 3 },
                                            }                                        
                                        ),
                                        box_router(Rectangle {
                                            from: Position { x: 0, y: 0 },
                                            to: Position { x: x - 1, y: y - 1 },
                                        })
                                    )
                                );
                            }
                        };
                        
                        /* let request = tonic::Request::new(stream::iter(vec![
                            RemoveNodeRequest {
                                status:RequestStatus::Initiating as i32,
                                recursive: true,
                                path:String::from("A")
                            },
                            RemoveNodeRequest {
                                status:RequestStatus::Initiating as i32,
                                recursive: true,
                                path:String::from("B")
                            },
                            RemoveNodeRequest {
                                status:RequestStatus::Initiating as i32,
                                recursive: true,
                                path:String::from("C")
                            },
                        ]));

                        let response = client.remove_node(request).await?;
                        let mut inbound = response.into_inner();

                        while let Some(res) = inbound.message().await? {
                            println!("NOTE = {:?}", res);
                        } */
                    },
                    _ => (),
                }
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }

    Ok(())
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

