use crate::game::{Action, Cell, Phase};
use crate::{
    ActionQueue, DefaultActionQueue, Event, EventResult, LinearLayout, PlayerId, Printer, State,
    World,
};
use cursive::align::HAlign;
use cursive::direction::Direction;
use cursive::event::{AnyCb, MouseButton, MouseEvent};
use cursive::traits::{Finder, Nameable, View};
use cursive::view::{CannotFocus, Selector, ViewNotFound};
use cursive::views::{Button, Panel};
use cursive::{Rect, Vec2};
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
        let layout = {
            let game_board_layout = GameView::game_board_layout(&game_state, &action_queue);
            let players = &game_state.borrow().players;
            // for more players this method would have been implemented quite differently
            assert_eq!(players.len(), 2);
            let players_and_board_layout = LinearLayout::horizontal()
                .child(Panel::new(PlayerView::new(
                    players[0].id,
                    Rc::clone(&game_state),
                )))
                .child(game_board_layout)
                .child(Panel::new(PlayerView::new(
                    players[1].id,
                    Rc::clone(&game_state),
                )));
            LinearLayout::vertical()
                .child(Panel::new(GameInfoView::new(Rc::clone(&game_state))))
                .child(players_and_board_layout)
                .child(Panel::new(GameControlsView::new(
                    Rc::clone(&game_state),
                    action_queue.as_ref().map(Rc::clone).or(None),
                )))
        };
        Self {
            _game_state: game_state,
            _action_queue: action_queue,
            game_world,
            layout,
        }
    }

    fn on_loop_iteration(&self) {
        self.game_world.advance();
    }

    fn game_board_layout(
        game_state: &Rc<RefCell<State>>,
        action_queue: &Option<Rc<DefaultActionQueue>>,
    ) -> LinearLayout {
        let board_size = game_state.borrow().board.size();
        let mut game_board_layout = LinearLayout::vertical();
        for x in 0..board_size {
            let mut column = LinearLayout::horizontal();
            for y in 0..board_size {
                column.add_child(Panel::new(CellView::new(
                    Cell::new(x, y),
                    Rc::clone(game_state),
                    action_queue.as_ref().map(Rc::clone).or(None),
                )));
            }
            game_board_layout.add_child(column);
        }
        game_board_layout
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

    fn focus_view(&mut self, _selector: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
        Err(ViewNotFound)
    }

    fn take_focus(&mut self, _source: Direction) -> Result<EventResult, CannotFocus> {
        Err(CannotFocus)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        self.layout.important_area(view_size)
    }
}

#[derive(Debug)]
struct GameInfoView {
    game_state: Rc<RefCell<State>>,
    size: Vec2,
}

impl GameInfoView {
    fn new(game_state: Rc<RefCell<State>>) -> Self {
        Self {
            game_state,
            size: Vec2::default(),
        }
    }
}

impl View for GameInfoView {
    fn draw(&self, printer: &Printer) {
        let game_state = &*self.game_state.borrow();
        printer.print(
            Vec2::new(
                HAlign::Center.get_offset(1, self.size.x),
                HAlign::Center.get_offset(1, self.size.y),
            ),
            &format!("round {}/{}", game_state.round + 1, game_state.rounds),
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

    fn on_event(&mut self, _event: Event) -> EventResult {
        EventResult::Ignored
    }

    fn call_on_any<'a>(&mut self, _selector: &Selector<'_>, _cb: AnyCb<'a>) {}

    fn focus_view(&mut self, _selector: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
        Err(ViewNotFound)
    }

    fn take_focus(&mut self, _source: Direction) -> Result<EventResult, CannotFocus> {
        Err(CannotFocus)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        Rect::from_size((0, 0), view_size)
    }
}

#[derive(Debug)]
struct PlayerView {
    player_id: PlayerId,
    game_state: Rc<RefCell<State>>,
    size: Vec2,
}

impl PlayerView {
    fn new(player_id: PlayerId, game_state: Rc<RefCell<State>>) -> Self {
        Self {
            player_id,
            game_state,
            size: Vec2::default(),
        }
    }
}

impl View for PlayerView {
    fn draw(&self, printer: &Printer) {
        printer.print(
            Vec2::new(
                HAlign::Center.get_offset(1, self.size.x),
                HAlign::Center.get_offset(1, self.size.y),
            ),
            &format!(
                "wins: {}",
                self.game_state.borrow().players[self.player_id.idx].wins
            ),
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

    fn on_event(&mut self, _event: Event) -> EventResult {
        EventResult::Ignored
    }

    fn call_on_any<'a>(&mut self, _selector: &Selector<'_>, _cb: AnyCb<'a>) {}

    fn focus_view(&mut self, _selector: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
        Err(ViewNotFound)
    }

    fn take_focus(&mut self, _source: Direction) -> Result<EventResult, CannotFocus> {
        Err(CannotFocus)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        Rect::from_size((0, 0), view_size)
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

    fn on_mouse_press_left(&self) -> EventResult {
        if let Some(action_queue) = &self.action_queue {
            let game_state = &*self.game_state.borrow();
            if game_state.phase == Phase::Inround
                && game_state.turn() == action_queue.player_id()
                && game_state.board.get(&self.cell) == None
            {
                action_queue.add(Action::Occupy(self.cell));
                EventResult::Consumed(None)
            } else {
                EventResult::Ignored
            }
        } else {
            EventResult::Ignored
        }
    }
}

impl View for CellView {
    fn draw(&self, printer: &Printer) {
        if let Some(mark) = self.game_state.borrow().board.get(&self.cell) {
            printer.print(
                Vec2::new(
                    HAlign::Center.get_offset(1, self.size.x),
                    HAlign::Center.get_offset(1, self.size.y),
                ),
                &format!("{}", mark),
            );
        }
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
            } if position.fits_in_rect(offset, self.size) => self.on_mouse_press_left(),
            _ => EventResult::Ignored,
        }
    }

    fn call_on_any<'a>(&mut self, _selector: &Selector<'_>, _cb: AnyCb<'a>) {}

    fn focus_view(&mut self, _selector: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
        Err(ViewNotFound)
    }

    fn take_focus(&mut self, source: Direction) -> Result<EventResult, CannotFocus> {
        // take focus only from mouse
        if source == Direction::none()
            && !GameControlsView::needs_go_confirmation(
                &*self.game_state.borrow(),
                self.action_queue.as_ref().map(|aq| aq.player_id()),
            )
        {
            Ok(EventResult::consumed())
        } else {
            Err(CannotFocus)
        }
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        Rect::from_size((0, 0), view_size)
    }
}

struct GameControlsView {
    game_state: Rc<RefCell<State>>,
    interacting_player_id: Option<PlayerId>,
    layout: LinearLayout,
    go_button_enabled: bool,
}

impl GameControlsView {
    const GO_BUTTON_ID: &'static str = "GO_BUTTON_ID";

    fn new(game_state: Rc<RefCell<State>>, action_queue: Option<Rc<DefaultActionQueue>>) -> Self {
        let mut layout = LinearLayout::vertical();
        if let Some(action_queue) = &action_queue {
            let action_queue = Rc::clone(action_queue);
            let game_state = Rc::clone(&game_state);
            let go_button = Button::new("Go", move |tui| {
                let game_state = &*game_state.borrow();
                if GameControlsView::needs_go_confirmation(
                    game_state,
                    Some(action_queue.player_id()),
                ) {
                    action_queue.add(Action::Ready);
                    tui.call_on_name(GameControlsView::GO_BUTTON_ID, |go_button: &mut Button| {
                        go_button.disable();
                    });
                } else {
                    panic!();
                }
            })
            .disabled()
            .with_name(GameControlsView::GO_BUTTON_ID);
            layout.add_child(go_button);
        }
        Self {
            game_state,
            interacting_player_id: action_queue.map(|action_queue| action_queue.player_id()),
            layout,
            go_button_enabled: false,
        }
    }

    fn needs_go_confirmation(game_state: &State, interacting_player_id: Option<PlayerId>) -> bool {
        if let Some(interacting_player_id) = interacting_player_id {
            game_state.required_ready.contains(&interacting_player_id)
        } else {
            false
        }
    }
}

impl View for GameControlsView {
    fn draw(&self, printer: &Printer) {
        self.layout.draw(printer);
    }

    fn layout(&mut self, view_size: Vec2) {
        let needs_go_confirmation = GameControlsView::needs_go_confirmation(
            &self.game_state.borrow(),
            self.interacting_player_id,
        );
        let switch_go_button_enabled = needs_go_confirmation != self.go_button_enabled;
        let observed_go_button_enabled = self
            .layout
            .find_name::<Button>(GameControlsView::GO_BUTTON_ID)
            .unwrap()
            .is_enabled();
        if observed_go_button_enabled && !self.go_button_enabled {
            // TODO still does not focus
            assert!(self
                .layout
                .focus_view(&Selector::Name(GameControlsView::GO_BUTTON_ID))
                .unwrap()
                .is_consumed());
        }
        self.go_button_enabled = observed_go_button_enabled;
        self.layout
            .call_on_name(GameControlsView::GO_BUTTON_ID, |go_button: &mut Button| {
                if switch_go_button_enabled {
                    if needs_go_confirmation {
                        go_button.enable();
                    } else {
                        go_button.disable();
                    }
                }
            });
        self.layout.layout(view_size);
    }

    fn needs_relayout(&self) -> bool {
        true
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

    fn focus_view(&mut self, _selector: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
        Err(ViewNotFound)
    }

    fn take_focus(&mut self, source: Direction) -> Result<EventResult, CannotFocus> {
        self.layout.take_focus(source)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        self.layout.important_area(view_size)
    }
}
