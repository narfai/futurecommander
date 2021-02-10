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

    fn is_contained_by(&self, area: &Area) -> bool {
        self.x >= area.from.x && self.x <= area.to.x && self.y >= area.from.y && self.y <= area.to.y
    }
}

#[derive(Clone, Copy, Debug)]
struct Area {
    pub from: Position,
    pub to: Position
}

impl Area {
    pub fn inside(&self, cell_number: u16) -> Area {
        fn sub_if_positive(n: u16, cn: u16) -> u16{ if n > 0 { n - cn } else { 0 } }
        Area {
            from: Position { x: self.from.x + cell_number, y: self.from.y + cell_number},
            to: Position { x: sub_if_positive(self.to.x, cell_number), y: sub_if_positive(self.to.y, cell_number)},
        }
    }
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
    pub content: String,
    pub foreground_color: String,
    pub background_color: String,
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
                content: String::from(content),
                foreground_color: String::from(format!("{}", color::Fg(color::White))),
                background_color: String::from(format!("{}", color::Bg(color::Blue)))
            }
        )
    }
}

impl RenderCell {
    pub fn render<W: Write> (self, out: &mut W, position: Position) {
        write!(
            out,
            "{}{}{}{}",
            cursor::Goto(position.x + 1, position.y + 1),
            self.foreground_color,
            self.background_color,
            self.content
        ).unwrap();
    }
}

fn bordered_area(area: Area) -> impl Fn(CellChoice) -> CellChoice {
    move |cellchoice| match cellchoice {
        CellChoice::Render(_) => cellchoice,
        CellChoice::Skip(cell) => {
            if cell.position.is_contained_by(&area) {
                match cell.position.to_tuple() {
                    (x, y) if x == area.from.x && y == area.from.y => cell.choose_to_render(TOP_LEFT_CORNER),
                    (x, y) if x == area.to.x && y == area.from.y => cell.choose_to_render(TOP_RIGHT_CORNER),
                    (x, y) if x == area.to.x && y == area.to.y => cell.choose_to_render(BOTTOM_RIGHT_CORNER),
                    (x, y) if x == area.from.x && y == area.to.y => cell.choose_to_render(BOTTOM_LEFT_CORNER),
                    (x, _) if x == area.from.x || x == area.to.x => cell.choose_to_render(VERT_BOUNDARY),
                    (_, y) if y == area.from.y || y == area.to.y => cell.choose_to_render(HORZ_BOUNDARY),
                    _ => CellChoice::Skip(cell)
                }
            } else {
                CellChoice::Skip(cell)
            }
        }
    }
}

#[derive(Debug)]
struct ExplorerState {
    pub selected: Option<u16>,
    pub nodes: Vec<String>
}

fn explorer(area: Area, state: ExplorerState) -> impl Fn(CellChoice) -> CellChoice {
    compose(
        bordered_area(area),
        move |cellchoice| match cellchoice {
            CellChoice::Render(_) => cellchoice,
            CellChoice::Skip(cell) => {
                let inside_area = area.inside(1);
                if cell.position.is_contained_by(&inside_area) {
                    let inner = Position {
                        y: cell.position.y - inside_area.from.y,
                        x: cell.position.x - inside_area.from.x,
                    };
                    if inner.y < state.nodes.len() as u16{
                        if let Some(c) = state.nodes[inner.y as usize].chars().nth(inner.x as usize) {
                            cell.choose_to_render(&String::from(c))
                        } else {
                            CellChoice::Skip(cell)
                        }
                    } else {
                        CellChoice::Skip(cell)
                    }
                } else {
                    CellChoice::Skip(cell)
                }
            }
        }
    )
}

#[derive(Debug)]
struct LayoutState {
    pub left_explorer: ExplorerState,
    pub right_explorer: ExplorerState
}

fn layout(area: Area, state: LayoutState)  -> impl Fn(CellChoice) -> CellChoice {
    let middle = Position{
        x: round::ceil((area.to.x/2).into(), 0) as u16,
        y: round::ceil((area.to.y/2).into(), 0) as u16,
    };
    compose(
        explorer(
            Area {
                from: Position { x: area.from.x, y: area.from.y },
                to: Position { x: middle.x, y: area.to.y - 1 },
            },
            state.left_explorer
        ),
        explorer(
            Area {
                from: Position { x: middle.x + 1, y: area.from.y },
                to: Position { x: area.to.x - 1, y: area.to.y - 1},
            },
            state.right_explorer
        ),

    )
}



fn render<W: Write> (out: &mut W, area: Area, cell_router: &dyn Fn(CellChoice) -> CellChoice){
    for x in area.from.x..area.to.x {
        for y in area.from.y..area.to.y {
            let position = Position{x, y};
            match cell_router(SkipCell::choose_to_skip(position)) {
                CellChoice::Render(cell) => cell.render(out, position),
                CellChoice::Skip(_) => { write!(out, "{}{}{}", cursor::Goto(x + 1, y + 1), color::Bg(color::Blue), " ").unwrap(); }
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


    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();

    let layout_state = LayoutState {
        left_explorer: ExplorerState {
            selected: None,
            nodes: vec!["A".into(), "B".into(), "C".into()]
        },
        right_explorer: ExplorerState {
            selected: None,
            nodes: vec!["C".into(), "D".into(), "E".into()]
        }
    };

    for c in stdin.events() {
        let event = c.unwrap();
        match event {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        // write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                        /*
                        if let Some(x) = termwidth {
                            if let Some(y) = termheight {
                                let render_area = Area {
                                    from: Position { x: 0, y: 0 },
                                    to: Position { x, y },
                                };
                                render(
                                    &mut stdout,
                                    render_area,
                                    &layout(render_area, &layout_state)
                                );
                            }
                        };
                        */

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

    write!(stdout, "{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

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
