use crate::game::{Action, Cell, Phase};
use crate::{
    ActionQueue, Cursive, DefaultActionQueue, Event, EventResult, LinearLayout, Logic, PlayerId,
    Printer, State,
};
use cursive::align::HAlign;
use cursive::direction::Direction;
use cursive::event::{AnyCb, MouseButton, MouseEvent};
use cursive::traits::{Finder, Nameable, View};
use cursive::view::{CannotFocus, Selector, ViewNotFound};
use cursive::views::{Button, DummyView, HideableView, NamedView, Panel, ResizedView};
use cursive::{Rect, Vec2};
use std::cell::RefCell;
use std::cmp;
use std::ops::Add;
use std::rc::Rc;

pub struct GameView {
    // TODO remove unused, pass a map?
    _action_queue: Option<Rc<DefaultActionQueue>>,
    on_loop_iteration: Box<dyn Fn()>,
    layout: LinearLayout,
}

impl GameView {
    pub fn new(
        game_state: &Rc<RefCell<State>>,
        action_queue: Option<Rc<DefaultActionQueue>>,
        on_loop_iteration: Box<dyn Fn()>,
    ) -> Self {
        let layout = {
            let game_board_layout = GameView::game_board_layout(game_state, &action_queue);
            let players = &game_state.borrow().players;
            // for more players this method would have been implemented quite differently
            assert_eq!(players.len(), 2);
            let players_and_board_layout = LinearLayout::horizontal()
                .child(Panel::new(PlayerView::new(
                    players[0].id,
                    Rc::clone(game_state),
                )))
                .child(game_board_layout)
                .child(Panel::new(PlayerView::new(
                    players[1].id,
                    Rc::clone(game_state),
                )));
            LinearLayout::vertical()
                .child(Panel::new(GameInfoView::new(Rc::clone(game_state))))
                .child(players_and_board_layout)
                .child(Panel::new(GameControlsView::new(
                    Rc::clone(game_state),
                    action_queue.as_ref().map(Rc::clone).or(None),
                )))
        };
        Self {
            _action_queue: action_queue,
            on_loop_iteration,
            layout,
        }
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
        (self.on_loop_iteration)();
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
        let txt_round = &format!("Round {}/{}", game_state.round + 1, game_state.rounds);
        printer.print(
            Vec2::new(
                HAlign::Center.get_offset(txt_round.chars().count(), self.size.x),
                HAlign::Center.get_offset(1, self.size.y),
            ),
            txt_round,
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
        let player = &self.game_state.borrow().players[self.player_id.idx];
        let txt_description = &format!("Player: {}", player.mark);
        let txt_score = &format!("Wins: {}", player.wins);
        let start = Vec2::new(
            HAlign::Center.get_offset(
                cmp::max(txt_description.chars().count(), txt_score.chars().count()),
                self.size.x,
            ),
            HAlign::Center.get_offset(2, self.size.y),
        );
        printer.print(start, txt_description);
        printer.print(start.add(Vec2::new(0, 1)), txt_score);
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
            let txt_mark = &format!("{}", mark);
            printer.print(
                Vec2::new(
                    HAlign::Center.get_offset(txt_mark.chars().count(), self.size.x),
                    HAlign::Center.get_offset(1, self.size.y),
                ),
                txt_mark,
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
        // take focus only from mouse of programmatically
        if source == Direction::none()
            && !GameControlsView::needs_go(
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
    go_btn_visible: bool,
    surrender_btn_visible: bool,
}

impl GameControlsView {
    /// A view with this ID is used to work around a bug in the cursive framework that
    /// results in broken focusing behavior in situations
    /// when the application hides/disables views.
    const FOCUS_HOLDER_ID: &'static str = "FOCUS_HOLDER_ID";
    const GO_BTN_ID: &'static str = "GO_BTN_ID";
    const SURRENDER_BTN_ID: &'static str = "SURRENDER_BTN_ID";

    fn new(game_state: Rc<RefCell<State>>, action_queue: Option<Rc<DefaultActionQueue>>) -> Self {
        let mut layout = LinearLayout::horizontal();
        if let Some(action_queue) = &action_queue {
            let go_btn = {
                let action_queue = Rc::clone(action_queue);
                GameControlsView::btn_hidden_on_cb(
                    GameControlsView::GO_BTN_ID,
                    "Ready/Continue",
                    move |_tui| {
                        action_queue.add(Action::Ready);
                    },
                )
            };
            let surrender_btn = {
                let action_queue = Rc::clone(action_queue);
                GameControlsView::btn_hidden_on_cb(
                    GameControlsView::SURRENDER_BTN_ID,
                    "Surrender",
                    move |_tui| {
                        action_queue.add(Action::Surrender);
                        action_queue.add(Action::Ready);
                    },
                )
            };
            layout.add_child(DummyView {}.with_name(GameControlsView::FOCUS_HOLDER_ID));
            layout.add_child(go_btn);
            layout.add_child(surrender_btn);
        }
        let centing_layout = LinearLayout::horizontal()
            .child(ResizedView::with_full_width(DummyView {}))
            .child(layout)
            .child(ResizedView::with_full_width(DummyView {}));
        Self {
            game_state,
            interacting_player_id: action_queue.map(|action_queue| action_queue.player_id()),
            layout: centing_layout,
            go_btn_visible: false,
            surrender_btn_visible: false,
        }
    }

    fn needs_go(game_state: &State, interacting_player_id: Option<PlayerId>) -> bool {
        if let Some(interacting_player_id) = interacting_player_id {
            game_state.required_ready.contains(&interacting_player_id)
                || Logic::is_game_over(game_state)
        } else {
            false
        }
    }

    fn allows_surrender(game_state: &State, interacting_player_id: Option<PlayerId>) -> bool {
        if interacting_player_id.is_some() {
            game_state.phase == Phase::Inround
        } else {
            false
        }
    }

    fn btn_hidden_on_cb<F, S>(id: S, label: S, cb: F) -> NamedView<HideableView<Button>>
    where
        F: 'static + Fn(&mut Cursive),
        S: Into<String>,
    {
        let id = id.into();
        let mut btn = {
            let id = id.clone();
            HideableView::new(Button::new(label, move |tui| {
                cb(tui);
                assert!(tui
                    .focus_name(GameControlsView::FOCUS_HOLDER_ID)
                    .unwrap()
                    .is_consumed());
                tui.call_on_name(&id, |btn: &mut HideableView<Button>| {
                    btn.hide();
                });
            }))
        };
        btn.hide();
        btn.with_name(id)
    }

    #[allow(dead_code)]
    // TODO do I need this function?
    fn btn_disabled_on_cb<F, S>(id: S, label: S, cb: F) -> NamedView<Button>
    where
        F: 'static + Fn(&mut Cursive),
        S: Into<String>,
    {
        let id = id.into();
        let mut btn = {
            let id = id.clone();
            Button::new(label, move |tui| {
                cb(tui);
                assert!(tui
                    .focus_name(GameControlsView::FOCUS_HOLDER_ID)
                    .unwrap()
                    .is_consumed());
                tui.call_on_name(&id, |btn: &mut Button| {
                    btn.disable();
                });
            })
        };
        btn.disable();
        btn.with_name(id)
    }
}

impl View for GameControlsView {
    fn draw(&self, printer: &Printer) {
        self.layout.draw(printer);
    }

    fn layout(&mut self, view_size: Vec2) {
        let game_state = &self.game_state.borrow();
        {
            // handle GO_BTN_ID
            let need_go_confirmation =
                GameControlsView::needs_go(game_state, self.interacting_player_id);
            let switch_go_btn_visible = need_go_confirmation != self.go_btn_visible;
            self.go_btn_visible = self
                .layout
                .find_name::<HideableView<Button>>(GameControlsView::GO_BTN_ID)
                .unwrap()
                .is_visible();
            if switch_go_btn_visible {
                self.layout.call_on_name(
                    GameControlsView::GO_BTN_ID,
                    |btn: &mut HideableView<Button>| {
                        if need_go_confirmation {
                            btn.unhide();
                        } else {
                            btn.hide();
                        }
                    },
                );
            }
        }
        {
            // handle SURRENDER_BTN_ID
            let allows_surrender =
                GameControlsView::allows_surrender(game_state, self.interacting_player_id);
            let switch_surrender_btn_visible = allows_surrender != self.surrender_btn_visible;
            self.surrender_btn_visible = self
                .layout
                .find_name::<HideableView<Button>>(GameControlsView::SURRENDER_BTN_ID)
                .unwrap()
                .is_visible();
            if switch_surrender_btn_visible {
                self.layout.call_on_name(
                    GameControlsView::SURRENDER_BTN_ID,
                    |btn: &mut HideableView<Button>| {
                        if allows_surrender {
                            btn.unhide();
                        } else {
                            btn.hide();
                        }
                    },
                );
            }
        }
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
