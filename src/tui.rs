use crate::game::{Action, Cell};
use crate::{
    ActionQueue, DefaultActionQueue, Event, EventResult, LinearLayout, Printer, State, World,
};
use cursive::align::HAlign;
use cursive::direction::Direction;
use cursive::event::{AnyCb, MouseButton, MouseEvent};
use cursive::view::{CannotFocus, Selector, ViewNotFound};
use cursive::views::Panel;
use cursive::{Rect, Vec2, View};
use std::cell::RefCell;
use std::rc::Rc;

pub struct GameView {
    _game_state: Rc<RefCell<State>>,
    _action_queue: Option<Rc<DefaultActionQueue>>,
    game_world: World,
    layout: LinearLayout,
}

impl GameView {
    pub fn new(
        game_state: Rc<RefCell<State>>,
        action_queue: Option<Rc<DefaultActionQueue>>,
        game_world: World,
    ) -> Self {
        let game_board = {
            let state = game_state.borrow();
            let mut layout = LinearLayout::vertical();
            for x in 0..state.board.size() {
                let mut column = LinearLayout::horizontal();
                for y in 0..state.board.size() {
                    column.add_child(Panel::new(CellView::new(
                        Cell::new(x, y),
                        Rc::clone(&game_state),
                        action_queue.as_ref().map(Rc::clone).or(None),
                    )));
                }
                layout.add_child(column);
            }
            layout
        };
        Self {
            _game_state: game_state,
            _action_queue: action_queue,
            game_world,
            layout: game_board,
        }
    }

    fn on_loop_iteration(&self) {
        self.game_world.advance();
    }
}

impl View for GameView {
    fn draw(&self, printer: &Printer) {
        self.on_loop_iteration();
        self.layout.draw(printer);
    }

    fn layout(&mut self, view_size: Vec2) {
        self.layout.layout(view_size);
    }

    fn needs_relayout(&self) -> bool {
        self.layout.needs_relayout()
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.layout.required_size(constraint)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        self.layout.on_event(event)
    }

    fn call_on_any<'a>(&mut self, selector: &Selector<'_>, cb: AnyCb<'a>) {
        self.layout.call_on_any(selector, cb);
    }

    fn focus_view(&mut self, selector: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
        self.layout.focus_view(selector)
    }

    fn take_focus(&mut self, source: Direction) -> Result<EventResult, CannotFocus> {
        self.layout.take_focus(source)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        self.layout.important_area(view_size)
    }
}

#[derive(Debug)]
struct CellView {
    cell: Cell,
    game_state: Rc<RefCell<State>>,
    action_queue: Option<Rc<DefaultActionQueue>>,
    size: Vec2,
}

impl CellView {
    fn new(
        cell: Cell,
        game_state: Rc<RefCell<State>>,
        action_queue: Option<Rc<DefaultActionQueue>>,
    ) -> Self {
        Self {
            cell,
            game_state,
            action_queue,
            size: Vec2::default(),
        }
    }

    fn on_mouse_press_left(&self) {
        if let Some(action_queue) = &self.action_queue {
            let game_state = &*self.game_state.borrow();
            if game_state.turn() == action_queue.player_id()
                && game_state.board.get(&self.cell) == None
            {
                action_queue.add(Action::Occupy(self.cell));
            }
        }
    }
}

impl View for CellView {
    fn draw(&self, printer: &Printer) {
        printer.print(
            Vec2::new(
                HAlign::Center.get_offset(1, self.size.x),
                HAlign::Center.get_offset(1, self.size.y),
            ),
            &format!("{:?}", self.game_state.borrow().board.get(&self.cell)),
        );
    }

    fn layout(&mut self, view_size: Vec2) {
        self.size = view_size;
    }

    fn needs_relayout(&self) -> bool {
        true
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        constraint
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Mouse {
                event: MouseEvent::Press(MouseButton::Left),
                position,
                offset,
            } if position.fits_in_rect(offset, self.size) => {
                self.on_mouse_press_left();
                EventResult::Consumed(None)
            }
            _ => EventResult::Ignored,
        }
    }

    fn call_on_any<'a>(&mut self, _selector: &Selector<'_>, _cb: AnyCb<'a>) {}

    fn focus_view(&mut self, _selector: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
        Err(ViewNotFound)
    }

    fn take_focus(&mut self, _source: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::consumed())
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        Rect::from_size((0, 0), view_size)
    }
}
