use anyhow::Result;
use yazi_config::LAYOUT;
use yazi_macro::{act, succ};
use yazi_parser::{ArrowForm, mgr::PeekForm};
use yazi_shared::data::Data;
use yazi_widgets::Step;

use crate::{Actor, Ctx};

pub struct Seek;

impl Actor for Seek {
	type Form = ArrowForm;

	const NAME: &str = "seek";

	fn act(cx: &mut Ctx, form: Self::Form) -> Result<Data> {
		let preview = &cx.tab().preview;
		let old = preview.skip;
		let step = match form.step {
			Step::Prev => Step::Offset(-1),
			Step::Next => Step::Offset(1),
			step => step,
		};
		let new = step.add(old, i32::MAX as usize, LAYOUT.get().preview.height as usize, old, 0);

		if new == old {
			succ!();
		}
		act!(mgr:peek, cx, PeekForm { skip: Some(new), ..Default::default() })
	}
}
