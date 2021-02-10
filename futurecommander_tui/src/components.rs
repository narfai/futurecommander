use crate::position::{
    Position,
    Area
};

const HORZ_BOUNDARY: &'static str = "─";
const VERT_BOUNDARY: &'static str = "│";
const TOP_LEFT_CORNER: &'static str = "┌";
const TOP_RIGHT_CORNER: &'static str = "┐";
const BOTTOM_LEFT_CORNER: &'static str = "└";
const BOTTOM_RIGHT_CORNER: &'static str = "┘";


pub trait EventHandler {
    fn handle(&mut self, event: Event) -> Event;        
}

pub struct Bordered {
    area: Area
}

impl Bordered {
    pub fn new(area: Area) -> Self { Bordered { area } }
}

impl EventHandler for Bordered {
    fn handle(&mut self, event: Event) -> Event {
        match event {
            Event::CellQuery(pos) => {
                if pos.is_contained_by(&self.area) {
                    match pos.to_tuple() {
                        (x, y) if x == self.area.left_top.x && y == self.area.left_top.y => Event::CellResponse(String::from(TOP_LEFT_CORNER)),
                        (x, y) if x == self.area.right_bottom.x && y == self.area.left_top.y => Event::CellResponse(String::from(TOP_RIGHT_CORNER)),
                        (x, y) if x == self.area.right_bottom.x && y == self.area.right_bottom.y => Event::CellResponse(String::from(BOTTOM_RIGHT_CORNER)),
                        (x, y) if x == self.area.left_top.x && y == self.area.right_bottom.y => Event::CellResponse(String::from(BOTTOM_LEFT_CORNER)),
                        (x, _) if x == self.area.left_top.x || x == self.area.right_bottom.x => Event::CellResponse(String::from(VERT_BOUNDARY)),
                        (_, y) if y == self.area.left_top.y || y == self.area.right_bottom.y => Event::CellResponse(String::from(HORZ_BOUNDARY)),
                        _ => event
                    }
                } else {
                    event
                }
            },
            _ => event
        }
    }
}

pub struct TextList {
    area: Area,
    lines: Vec<String>
}

impl TextList {
    pub fn new(area: Area, lines: Vec<String>) -> Self {
        TextList {       
            area,    
            lines
        }
    }

    pub fn set_lines(&mut self, lines: Vec<String>) {
        self.lines = lines;
    }
}

impl EventHandler for TextList {
    fn handle(&mut self, event: Event) -> Event {
        match event {
            Event::CellQuery(pos) => {
                if pos.is_contained_by(&self.area) {
                    let inner = Position {
                        y: pos.y - self.area.left_top.y,
                        x: pos.x - self.area.left_top.x,
                    };
                    if inner.y < self.lines.len() as u16{
                        if let Some(c) = self.lines[inner.y as usize].chars().nth(inner.x as usize) {
                            return Event::CellResponse(String::from(c));
                        }
                    }
                };
                event
            },
            _ => event
        }
    }
}

/*

render at the beginning
rerender a subcomponent
handle keyboard and mouse event

*/

#[derive(Debug, PartialEq)]
pub enum Event {
    CellQuery(Position),
    CellResponse(String),
    UpdateTextList(Vec<String>)
}

impl Eq for Event {}

pub struct RenderEvent{
    content: Option<String>,
    position: Position
}

fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}
            
pub fn component<'a, T: EventHandler + 'a>(event_handler: &'a mut T) -> impl Fn(Event) -> Event {    
    move |event| event_handler.handle(event)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_bordered() {
        let render_area = Area::new(
            Position::new(0, 0), 
            Position::new(10, 10)
        );

        let border = Bordered::new(render_area);

        assert_eq!(
            border.handle(Event::CellQuery(Position::new(0,0))),
            Event::CellResponse(String::from(TOP_LEFT_CORNER))
        );
        assert_eq!(
            border.handle(Event::CellQuery(Position::new(10,10))),
            Event::CellResponse(String::from(BOTTOM_RIGHT_CORNER))
        );
    }

    #[test]
    pub fn test_builder(){
        let render_area = Area::new(Position::new(0, 0), Position::new(20, 50));
        
        let renderer = compose(
            component(&mut Bordered::new(render_area.inside(1))),
            compose(
                component(&mut Bordered::new(render_area.inside(2))),
                component(&mut TextList::new(render_area.inside(3), vec![String::from("testA"), String::from("testB"), String::from("testC")]))
            )
        );

        assert_eq!(
            Event::CellResponse(String::from(TOP_LEFT_CORNER)),
            renderer(Event::CellQuery(Position::new(1, 1)))
        );

        assert_eq!(
            Event::CellResponse(String::from(TOP_LEFT_CORNER)),
            renderer(Event::CellQuery(Position::new(2, 2)))
        );

        assert_eq!(
            Event::CellResponse(String::from(BOTTOM_RIGHT_CORNER)),
            renderer(Event::CellQuery(Position::new(19, 49)))
        );

        assert_eq!(
            Event::CellResponse(String::from("t")),
            renderer(Event::CellQuery(Position::new(3, 3)))
        );

        // text_list.set_lines(vec![String::from("b")]);
/* 
        assert_eq!(
            Some(String::from("b")),
            renderer(RenderEvent::new(Position::new(2, 2))).content
        ); */
    }

    // FAIRE UN TRUC SIMPLE ET SPECIFIQUE !!
}