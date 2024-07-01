use super::*;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub enum PlayerAction {
	#[default]
	Walk,
	Push,
	Swim,
	Drown,
	Burn,
	Skate,
	Slide,
	Suction,
	Death,
	Win,
}

#[derive(Clone, Default)]
pub struct PlayerState {
	pub entity: EntityHandle,

	/// Player input manager.
	pub inbuf: InputBuffer,

	/// Current player action.
	pub action: PlayerAction,
	/// True if previous movement was involuntary.
	pub forced_move: bool,
	/// Total steps taken (for high score).
	pub steps: i32,
	/// Total chips collected.
	pub chips: i32,
	/// Keys collected.
	pub keys: [u8; 4],

	pub flippers: bool,
	pub fire_boots: bool,
	pub ice_skates: bool,
	pub suction_boots: bool,

	pub dev_wtw: bool,
}

pub fn ps_update_moves(s: &mut GameState, input: &Input) {
	if !(s.input.a && s.input.b) && input.a && input.b {
		s.ps.dev_wtw = !s.ps.dev_wtw;
	}
	s.ps.inbuf.handle(Dir::Left,  input.left,  s.input.left);
	s.ps.inbuf.handle(Dir::Right, input.right, s.input.right);
	s.ps.inbuf.handle(Dir::Up,    input.up,    s.input.up);
	s.ps.inbuf.handle(Dir::Down,  input.down,  s.input.down);
}

pub fn ps_action(s: &mut GameState, action: PlayerAction) {
	if s.ps.action != action {
		s.ps.action = action;
		s.events.push(GameEvent::PlayerAction { player: s.ps.entity });
		if matches!(action, PlayerAction::Win) {
			s.events.push(GameEvent::GameWin { player: s.ps.entity });
		}
		if matches!(action, PlayerAction::Burn | PlayerAction::Death | PlayerAction::Drown) {
			s.events.push(GameEvent::GameOver { player: s.ps.entity });
		}
	}
}
