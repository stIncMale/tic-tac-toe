//! [`View`] forces us to have only `'static` references as fields,
//! which is why instead of referencing, all views in this module "own" things via [`Rc`].

use alloc::{borrow::Cow, rc::Rc};
use core::{cell::RefCell, time::Duration};
use std::{collections::HashMap, time::Instant};

use cursive::{
    align::HAlign,
    direction::{Direction, Orientation},
    event::{AnyCb, MouseButton, MouseEvent},
    traits::{Finder, Nameable, View},
    view::{CannotFocus, Selector, ViewNotFound},
    views::{
        Button, DummyView, EnableableView, NamedView, Panel, ResizedView, SliderView, TextContent,
    },
    Rect, Vec2, With,
};
use unicode_width::UnicodeWidthStr;
use xxhash_rust::xxh3::Xxh3Builder;
use EventResult::Ignored;
use LocalPlayerType::Ai;
use PlayerType::_Remote;

use crate::{
    ai,
    game::{
        Action, Cell,
        Phase::{Beginning, Inround, Outround},
    },
    tui::fx::{BlinkingAnimation, HIGHLIGHTED_COLOR_STYLE},
    util::time::AdvanceableClock,
    ActionQueue, Cursive, DefaultActionQueue, Event, EventResult,
    EventResult::Consumed,
    Human, LinearLayout, Local, LocalPlayerType, Logic, Player, PlayerId, PlayerType, Printer,
    TextView, World,
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
    const AI_COMMON_CTRLS_VIEW_ID: &'static str = "AI_COMMON_CTRLS_VIEW_ID";

    pub fn new(
        game_world: World<DefaultActionQueue>,
        action_queues: Vec<Rc<DefaultActionQueue>>,
    ) -> Self {
        assert_eq!(
            game_world.state().players.len(),
            2,
            "For more players TUI would have been implemented quite differently."
        );
        assert!(
            !action_queues
                .iter()
                .any(|aq| game_world.state().players[aq.player_id().idx].typ != Local(Human)),
            "The provided actions queues must correspond to `Local(Human)` players: {game_world:?}, {action_queues:?}."
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
            let mut layout = LinearLayout::vertical()
                .child(Panel::new(GameInfoView::new(&game_world)))
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
                );
            if players_local_human_first.first().unwrap().typ == Local(Ai) {
                // all players are AI
                layout.add_child(
                    Panel::new(
                        LocalAiCommonControlsView::new(&game_world)
                            .with_name(GameView::AI_COMMON_CTRLS_VIEW_ID),
                    )
                    .title("Common AI controls")
                    .title_position(HAlign::Left),
                );
            }
            layout
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
        let title = game_world.borrow().state().players[player_id.idx].to_string();
        match game_world.borrow().state().players[player_id.idx].typ {
            Local(Human) => Box::new(
                Panel::new(
                    LinearLayout::vertical()
                        .child(PlayerInfoView::new(player_id, game_world, clock))
                        .child(
                            Panel::new(LocalHumanControlsView::new(
                                game_world,
                                &action_queues[&player_id],
                            ))
                            .title("Controls")
                            .title_position(HAlign::Left),
                        ),
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

    fn advance(&mut self) {
        self.clock.borrow_mut().advance_to_real_now();
        let maximise_advance = self
            .layout
            .call_on_name(
                GameView::AI_COMMON_CTRLS_VIEW_ID,
                |view: &mut NamedView<LocalAiCommonControlsView>| {
                    view.get_mut().unleashed_promptness
                },
            )
            .unwrap();
        if maximise_advance {
            // TODO implement this based on FPS
            for _ in 0..200_000 {
                self.game_world.borrow_mut().advance();
            }
        } else {
            self.game_world.borrow_mut().advance();
        }
    }
}

impl View for GameView {
    fn draw(&self, printer: &Printer) {
        self.layout.draw(printer);
    }

    fn layout(&mut self, view_size: Vec2) {
        self.advance();
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

    fn call_on_any<'a>(&mut self, selector: &Selector<'_>, cb: AnyCb<'_>) {
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
        let txt_round = &format!("Round {} of {}", game_state.round + 1, game_state.rounds);
        printer.print(
            Vec2::new(
                HAlign::Center.get_offset(UnicodeWidthStr::width(txt_round.as_str()), self.size.x),
                HAlign::Center.get_offset(1, self.size.y),
            ),
            txt_round,
        );
    }

    fn layout(&mut self, view_size: Vec2) {
        self.size = view_size;
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        let max_height = 3usize;
        constraint.with_if(constraint.y > max_height, |v| v.y = max_height)
    }
}

#[derive(Debug)]
struct PlayerInfoView {
    player_id: PlayerId,
    game_world: GameWorld,
    clock: Clock,
    size: Vec2,
    status_anim: BlinkingAnimation,
}

impl PlayerInfoView {
    fn new(player_id: PlayerId, game_world: &GameWorld, clock: &Clock) -> Self {
        Self {
            player_id,
            game_world: Rc::clone(game_world),
            clock: Rc::clone(clock),
            size: Vec2::default(),
            status_anim: BlinkingAnimation::new(
                clock.borrow().now(),
                Duration::from_millis(200),
                None,
            ),
        }
    }
}

impl View for PlayerInfoView {
    fn draw(&self, printer: &Printer) {
        let game_world = self.game_world.borrow();
        let game_state = game_world.state();
        let player = &game_state.players[self.player_id.idx];
        let start = Vec2::new(2, 0);
        let txt_status = "Status: ";
        let start_status = start + Vec2::new(UnicodeWidthStr::width(txt_status), 0);
        printer.print(start, txt_status);
        let (txt_status, animated) = {
            let txt_waiting = "waiting";
            match game_state.phase {
                Beginning | Outround => {
                    if game_state.required_ready.contains(&self.player_id) {
                        (
                            match player.typ {
                                Local(Human) => "ready up",
                                Local(Ai) | _Remote => "readying up",
                            },
                            true,
                        )
                    } else if Logic::<DefaultActionQueue>::is_game_over(game_state) {
                        ("", false)
                    } else {
                        (txt_waiting, false)
                    }
                }
                Inround => {
                    if game_state.turn() == self.player_id {
                        (
                            match player.typ {
                                Local(Human) => "your turn",
                                Local(Ai) | _Remote => "thinking",
                            },
                            true,
                        )
                    } else {
                        (txt_waiting, false)
                    }
                }
            }
        };
        if animated {
            self.status_anim
                .draw(self.clock.borrow().now(), printer, |printer| {
                    printer.print(start_status, txt_status);
                });
        } else {
            printer.print(start_status, txt_status);
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
            if game_state.phase == Inround && game_state.board.get(&self.cell).is_none() {
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
            let txt_mark = &game_state.players[player_id.idx].mark().to_string();
            let draw = |printer: &Printer| {
                printer.print(
                    Vec2::new(
                        HAlign::Center
                            .get_offset(UnicodeWidthStr::width(txt_mark.as_str()), self.size.x),
                        HAlign::Center.get_offset(1, self.size.y),
                    ),
                    txt_mark,
                );
            };
            if game_state
                .win_line
                .map_or(false, |line| line.contains(&self.cell))
            {
                printer.with_color(HIGHLIGHTED_COLOR_STYLE, draw);
            } else if let Some(occupied_anim) = &self.occupied_anim {
                occupied_anim.draw(self.clock.borrow().now(), printer, draw);
            } else {
                draw(printer);
            }
        }
    }

    fn layout(&mut self, view_size: Vec2) {
        self.size = view_size;
        let game_world = self.game_world.borrow();
        let game_state = game_world.state();
        match &self.occupied_anim {
            None => {
                if let Some(player_id) = game_state.board.get(&self.cell) {
                    let turn = game_state.turn();
                    if turn != player_id && game_state.players[turn.idx].typ == Local(Human) {
                        self.occupied_anim = Some(BlinkingAnimation::new(
                            self.clock.borrow().now(),
                            Duration::from_millis(800),
                            Some(1),
                        ));
                    }
                }
            }
            Some(_) => {
                if game_state.board.get(&self.cell).is_none() {
                    self.occupied_anim = None;
                }
            }
        }
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

    fn btn_disabled_on_cb<S, F>(
        id: &'static str,
        label: S,
        cb: F,
    ) -> NamedView<EnableableView<Button>>
    where
        S: Into<String>,
        F: 'static + Fn(&mut Cursive),
    {
        let mut btn = {
            EnableableView::new(Button::new(label, move |tui| {
                cb(tui);
                tui.call_on_name(id, |btn: &mut NamedView<EnableableView<Button>>| {
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

    fn call_on_any<'a>(&mut self, selector: &Selector<'_>, cb: AnyCb<'_>) {
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

struct LocalAiCommonControlsView {
    game_world: GameWorld,
    unleashed_promptness: bool,
    promptness_view_content: TextContent,
    layout: LinearLayout,
}

impl LocalAiCommonControlsView {
    const PROMPTNESS_SLIDER_ID: &'static str = "PROMPTNESS_SLIDER_ID";

    fn new(game_world: &GameWorld) -> Self {
        let promtpness_slider = {
            let mut slider = SliderView::new(Orientation::Horizontal, 19);
            slider.set_value(slider.get_max_value() / 2);
            slider.with_name(LocalAiCommonControlsView::PROMPTNESS_SLIDER_ID)
        };
        let promptness_view_content = TextContent::new("");
        let centering_layout = LinearLayout::horizontal()
            .child(ResizedView::with_full_width(DummyView {}))
            .child(
                LinearLayout::horizontal()
                    .child(TextView::new("Promptness: "))
                    .child(promtpness_slider)
                    .child(TextView::new_with_content(promptness_view_content.clone())),
            )
            .child(ResizedView::with_full_width(DummyView {}));
        Self {
            game_world: Rc::clone(game_world),
            promptness_view_content,
            unleashed_promptness: false,
            layout: centering_layout,
        }
    }

    fn control(&mut self) {
        let mut game_world = self.game_world.borrow_mut();
        for ai in game_world.ais() {
            // promptness ∈ [-promptness_max, promptness_max]
            let (promptness, promptness_max) = self
                .layout
                .call_on_name(
                    LocalAiCommonControlsView::PROMPTNESS_SLIDER_ID,
                    |named_slider: &mut NamedView<SliderView>| {
                        let slider = named_slider.get_mut();
                        let slider_max_value = slider.get_max_value();
                        assert_eq!(
                            slider_max_value & 1,
                            1,
                            "The implementation expects an odd value."
                        );
                        let slider_half_max_value = slider.get_max_value() / 2;
                        (
                            i32::try_from(slider.get_value()).unwrap()
                                - i32::try_from(slider_half_max_value).unwrap(),
                            i32::try_from(slider_half_max_value).unwrap(),
                        )
                    },
                )
                .unwrap();
            let default_promptness = 0;
            // the space here is a filler in place of the `-` sign indicator
            let txt_default = " default";
            let txt_fps_based = " FPS-based";
            let txt_unleashed = " unleashed";
            let txt_promptness_max_width = UnicodeWidthStr::width(txt_default).max(
                UnicodeWidthStr::width(txt_fps_based).max(UnicodeWidthStr::width(txt_unleashed)),
            );
            self.unleashed_promptness = promptness == promptness_max;
            let (delay, txt_promptness) = if promptness == default_promptness {
                (ai::DEFAULT_BASE_DELAY, Cow::from(txt_default))
            } else if self.unleashed_promptness {
                (Duration::from_nanos(0), Cow::from(txt_unleashed))
            } else if promptness == promptness_max - 1 {
                (Duration::from_nanos(0), Cow::from(txt_fps_based))
            } else {
                let delay = ai::DEFAULT_BASE_DELAY
                    .mul_f32((if promptness < 0 { 1.25f32 } else { 1.8 }).powi(-promptness));
                let txt_promptness = if promptness < 0 {
                    promptness.to_string()
                } else {
                    // the space here is a filler in place of the `-` sign indicator
                    format!(" {promptness}")
                };
                let txt_promptness_width = UnicodeWidthStr::width(txt_promptness.as_str());
                assert!(
                    txt_promptness_width <= txt_promptness_max_width,
                    "Numeric promptness representation should not be longer than a verbal one: {txt_promptness_width:?}, {txt_promptness_max_width:?}."
                );
                (delay, Cow::from(txt_promptness))
            };
            ai.set_base_act_delay(delay);
            self.promptness_view_content
                // the space here is a spacer between the promptness slider and the text
                .set_content(format!(" {txt_promptness:<txt_promptness_max_width$}"));
        }
    }
}

impl View for LocalAiCommonControlsView {
    fn draw(&self, printer: &Printer) {
        self.layout.draw(printer);
    }

    fn layout(&mut self, view_size: Vec2) {
        self.control();
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

    fn call_on_any<'a>(&mut self, selector: &Selector<'_>, cb: AnyCb<'_>) {
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
