//! [`View`] forces us to have only `'static` references as fields,
//! which is why instead of referencing, all [`crate::tui`] views "own" things via [`Rc`].

use alloc::rc::Rc;
use core::{cell::RefCell, ops::Mul, time::Duration};
use std::{collections::HashMap, time::Instant};

use cursive::{
    align::HAlign,
    direction::Direction,
    event::{AnyCb, MouseButton, MouseEvent},
    theme::{ColorStyle, ColorType},
    traits::{Finder, Nameable, View},
    view::{CannotFocus, Selector, ViewNotFound},
    views::{Button, DummyView, EnableableView, NamedView, Panel, ResizedView},
    Rect, Vec2,
};
use xxhash_rust::xxh3::Xxh3Builder;
use EventResult::Ignored;
use LocalPlayerType::Ai;
use PlayerType::_Remote;

use crate::{
    game::{Action, Cell, Phase::Inround},
    util::AdvanceableClock,
    ActionQueue, Cursive, DefaultActionQueue, Event, EventResult,
    EventResult::Consumed,
    Human, LinearLayout, Local, LocalPlayerType, Logic, PaletteColor, Player, PlayerId, PlayerType,
    Printer, State, World,
};

type GameWorld = Rc<RefCell<World<DefaultActionQueue>>>;
type ActionQueues = Rc<HashMap<PlayerId, Rc<DefaultActionQueue>, Xxh3Builder>>;
type Clock = Rc<RefCell<AdvanceableClock>>;

pub struct GameView {
    game_world: GameWorld,
    clock: Clock,
    layout: LinearLayout,
}

impl GameView {
    pub fn new(
        game_world: World<DefaultActionQueue>,
        action_queues: Vec<Rc<DefaultActionQueue>>,
    ) -> Self {
        assert_eq!(
            State::PLAYER_COUNT,
            2,
            "for more players TUI would have been implemented quite differently"
        );
        assert!(
            !action_queues
                .iter()
                .any(|aq| game_world.state().players[aq.player_id().idx].typ != Local(Human)),
            "{:?}, {:?}",
            game_world,
            action_queues
        );
        let action_queues = {
            let mut map = HashMap::with_hasher(Xxh3Builder::new());
            for aq in action_queues {
                map.insert(aq.player_id(), aq);
            }
            map
        };
        let game_world = Rc::new(RefCell::new(game_world));
        let action_queues = Rc::new(action_queues);
        let clock = Rc::new(RefCell::new(AdvanceableClock::new(Instant::now())));
        let layout = {
            let game_world_ref = game_world.borrow();
            let players_local_human_first = {
                let mut vec = game_world_ref
                    .state()
                    .players
                    .iter()
                    .collect::<Vec<&Player>>();
                vec.sort_unstable_by_key(|p| p.typ);
                vec
            };
            LinearLayout::vertical()
                .child(GameInfoView::new(&game_world))
                .child(
                    LinearLayout::horizontal()
                        .child(GameView::player_layout(
                            players_local_human_first[0].id,
                            &game_world,
                            &action_queues,
                            &clock,
                        ))
                        .child(GameView::game_board_layout(
                            &game_world,
                            &action_queues,
                            &clock,
                        ))
                        .child(GameView::player_layout(
                            players_local_human_first[1].id,
                            &game_world,
                            &action_queues,
                            &clock,
                        )),
                )
        };
        Self {
            game_world,
            clock: Rc::clone(&clock),
            layout,
        }
    }

    fn game_board_layout(
        game_world: &GameWorld,
        action_queues: &ActionQueues,
        clock: &Clock,
    ) -> impl View {
        let game_world_ref = game_world.borrow();
        let game_state = game_world_ref.state();
        let board_size = game_state.board.size();
        let mut game_board_layout = LinearLayout::vertical();
        for x in 0..board_size {
            let mut column = LinearLayout::horizontal();
            for y in 0..board_size {
                column.add_child(Panel::new(CellView::new(
                    Cell::new(x, y),
                    game_world,
                    action_queues,
                    clock,
                )));
            }
            game_board_layout.add_child(column);
        }
        game_board_layout
    }

    fn player_layout(
        player_id: PlayerId,
        game_world: &GameWorld,
        action_queues: &ActionQueues,
        clock: &Clock,
    ) -> Box<dyn View> {
        let title = format!("{}", game_world.borrow().state().players[player_id.idx]);
        match game_world.borrow().state().players[player_id.idx].typ {
            Local(Human) => Box::new(
                Panel::new(
                    LinearLayout::vertical()
                        .child(PlayerInfoView::new(player_id, game_world, clock))
                        .child(Panel::new(LocalHumanControlsView::new(
                            game_world,
                            &action_queues[&player_id],
                        ))),
                )
                .title(title)
                .title_position(HAlign::Left),
            ),
            Local(Ai) | _Remote => Box::new(
                Panel::new(
                    LinearLayout::vertical()
                        .child(PlayerInfoView::new(player_id, game_world, clock)),
                )
                .title(title)
                .title_position(HAlign::Left),
            ),
        }
    }
}

impl View for GameView {
    fn draw(&self, printer: &Printer) {
        self.layout.draw(printer);
    }

    fn layout(&mut self, view_size: Vec2) {
        self.clock.borrow_mut().advance();
        self.game_world.borrow_mut().advance();
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
struct GameInfoView {
    game_world: GameWorld,
    size: Vec2,
}

impl GameInfoView {
    fn new(game_world: &GameWorld) -> Self {
        Self {
            game_world: Rc::clone(game_world),
            size: Vec2::default(),
        }
    }
}

impl View for GameInfoView {
    fn draw(&self, printer: &Printer) {
        let game_world = self.game_world.borrow();
        let game_state = game_world.state();
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

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        constraint
    }
}

#[derive(Debug)]
struct PlayerInfoView {
    player_id: PlayerId,
    game_world: GameWorld,
    size: Vec2,
    thinking_anim: BlinkingAnimation,
}

impl PlayerInfoView {
    fn new(player_id: PlayerId, game_world: &GameWorld, clock: &Clock) -> Self {
        Self {
            player_id,
            game_world: Rc::clone(game_world),
            size: Vec2::default(),
            thinking_anim: BlinkingAnimation::new(clock, Duration::from_millis(200), None),
        }
    }
}

impl View for PlayerInfoView {
    fn draw(&self, printer: &Printer) {
        let game_world = self.game_world.borrow();
        let game_state = game_world.state();
        let player = &game_state.players[self.player_id.idx];
        let start = Vec2::new(2, 0);
        if game_state.phase == Inround && game_state.turn() == self.player_id {
            self.thinking_anim.draw(printer, |printer| {
                printer.print(
                    start,
                    match player.typ {
                        Local(Human) => "Your turn!",
                        Local(Ai) | _Remote => "Thinking...",
                    },
                );
            });
        }
        printer.print(
            start + Vec2::new(0, 1),
            &format!("Rounds won: {}", player.wins),
        );
    }

    fn layout(&mut self, view_size: Vec2) {
        self.size = view_size;
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        constraint
    }
}

#[derive(Debug)]
struct CellView {
    cell: Cell,
    game_world: GameWorld,
    action_queues: ActionQueues,
    clock: Clock,
    size: Vec2,
    occupied_anim: Option<BlinkingAnimation>,
}

impl CellView {
    fn new(
        cell: Cell,
        game_world: &GameWorld,
        action_queues: &ActionQueues,
        clock: &Clock,
    ) -> Self {
        Self {
            cell,
            game_world: Rc::clone(game_world),
            action_queues: Rc::clone(action_queues),
            clock: Rc::clone(clock),
            size: Vec2::default(),
            occupied_anim: None,
        }
    }

    fn on_mouse_press_left(&self) -> EventResult {
        let game_world = self.game_world.borrow();
        let game_state = game_world.state();
        if let Some(action_queue) = self.action_queues.get(&game_state.turn()) {
            if game_state.phase == Inround && game_state.board.get(&self.cell) == None {
                action_queue.add(Action::Occupy(self.cell));
                Consumed(None)
            } else {
                Ignored
            }
        } else {
            Ignored
        }
    }
}

impl View for CellView {
    fn draw(&self, printer: &Printer) {
        let game_world = self.game_world.borrow();
        let game_state = game_world.state();
        if let Some(player_id) = game_state.board.get(&self.cell) {
            let txt_mark = &format!("{}", game_state.players[player_id.idx].mark());
            let draw = |printer: &Printer| {
                printer.print(
                    Vec2::new(
                        HAlign::Center.get_offset(txt_mark.chars().count(), self.size.x),
                        HAlign::Center.get_offset(1, self.size.y),
                    ),
                    txt_mark,
                );
            };
            // TODO render animation especially for AI/remote players; currently it's the opposite
            if let Some(occupied_anim) = &self.occupied_anim {
                occupied_anim.draw(printer, draw);
            } else {
                draw(printer);
            }
        }
    }

    fn layout(&mut self, view_size: Vec2) {
        self.size = view_size;
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
                let event_res = self.on_mouse_press_left();
                if event_res.is_consumed() {
                    self.occupied_anim = Some(BlinkingAnimation::new(
                        &self.clock,
                        Duration::from_millis(50),
                        Some(6),
                    ));
                }
                event_res
            }
            _ => Ignored,
        }
    }

    fn take_focus(&mut self, _source: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::consumed())
    }
}

struct LocalHumanControlsView {
    game_world: GameWorld,
    action_queue: Rc<DefaultActionQueue>,
    layout: LinearLayout,
}

impl LocalHumanControlsView {
    const GO_BTN_ID: &'static str = "GO_BTN_ID";
    const SURRENDER_BTN_ID: &'static str = "SURRENDER_BTN_ID";

    fn new(game_world: &GameWorld, action_queue: &Rc<DefaultActionQueue>) -> Self {
        let centering_layout = LinearLayout::horizontal()
            .child(ResizedView::with_full_width(DummyView {}))
            .child(
                LinearLayout::vertical()
                    .child(LocalHumanControlsView::go_btn(action_queue))
                    .child(LocalHumanControlsView::surrender_btn(action_queue)),
            )
            .child(ResizedView::with_full_width(DummyView {}));
        Self {
            game_world: Rc::clone(game_world),
            action_queue: Rc::clone(action_queue),
            layout: centering_layout,
        }
    }

    fn go_btn(action_queue: &Rc<DefaultActionQueue>) -> NamedView<EnableableView<Button>> {
        let action_queue = Rc::clone(action_queue);
        LocalHumanControlsView::btn_disabled_on_cb(
            LocalHumanControlsView::GO_BTN_ID,
            "Ready/Continue",
            move |_tui| {
                action_queue.add(Action::Ready);
            },
        )
    }

    fn surrender_btn(action_queue: &Rc<DefaultActionQueue>) -> NamedView<EnableableView<Button>> {
        let action_queue = Rc::clone(action_queue);
        LocalHumanControlsView::btn_disabled_on_cb(
            LocalHumanControlsView::SURRENDER_BTN_ID,
            "Surrender the round",
            move |_tui| {
                action_queue.add(Action::Surrender);
            },
        )
    }

    fn layout_go_btn(&mut self) {
        let enable = {
            let game_world = self.game_world.borrow();
            let game_state = game_world.state();
            game_state
                .required_ready
                .contains(&self.action_queue.player_id())
                || Logic::<DefaultActionQueue>::is_game_over(game_state)
        };
        self.layout.call_on_name(
            LocalHumanControlsView::GO_BTN_ID,
            |btn: &mut NamedView<EnableableView<Button>>| {
                if enable {
                    btn.get_mut().enable();
                } else {
                    btn.get_mut().disable();
                }
            },
        );
    }

    fn layout_surrender_btn(&mut self) {
        let enable = self.game_world.borrow().state().phase == Inround;
        self.layout.call_on_name(
            LocalHumanControlsView::SURRENDER_BTN_ID,
            |btn: &mut NamedView<EnableableView<Button>>| {
                if enable {
                    btn.get_mut().enable();
                } else {
                    btn.get_mut().disable();
                }
            },
        );
    }

    fn btn_disabled_on_cb<F, S>(id: S, label: S, cb: F) -> NamedView<EnableableView<Button>>
    where
        F: 'static + Fn(&mut Cursive),
        S: Into<String>,
    {
        let id = id.into();
        let mut btn = {
            let id = id.clone();
            EnableableView::new(Button::new(label, move |tui| {
                cb(tui);
                tui.call_on_name(&id, |btn: &mut NamedView<EnableableView<Button>>| {
                    btn.get_mut().disable();
                });
            }))
        };
        btn.disable();
        btn.with_name(id)
    }
}

impl View for LocalHumanControlsView {
    fn draw(&self, printer: &Printer) {
        self.layout.draw(printer);
    }

    fn layout(&mut self, view_size: Vec2) {
        self.layout_go_btn();
        self.layout_surrender_btn();
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
struct BlinkingAnimation {
    clock: Clock,
    t_start: Duration,
    period: Duration,
    duration_periods: Option<u32>,
}

impl BlinkingAnimation {
    fn new(clock: &Clock, period: Duration, duration_periods: Option<u32>) -> Self {
        let now = clock.borrow().now();
        Self {
            clock: Rc::clone(clock),
            t_start: now,
            period,
            duration_periods,
        }
    }

    fn draw<F>(&self, printer: &Printer, f: F)
    where
        F: FnOnce(&Printer),
    {
        let now = self.clock.borrow().now();
        let elapsed = now - self.t_start;
        if self
            .duration_periods
            .map_or(false, |n| elapsed > self.period.mul(n))
        {
            f(printer);
        } else {
            let even_period = (elapsed.as_nanos() / self.period.as_nanos()) & 1 == 0;
            if even_period {
                printer.with_color(
                    ColorStyle {
                        front: ColorType::Palette(PaletteColor::Primary),
                        back: ColorType::Palette(PaletteColor::Tertiary),
                    },
                    f,
                );
            } else {
                f(printer);
            }
        }
    }
}
