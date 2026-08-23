use anyhow::Result;
use yazi_macro::{act, render, succ};
use yazi_parser::app::ReflowForm;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct Fullscreen;

impl Actor for Fullscreen {
	type Form = ReflowForm;

	const NAME: &str = "fullscreen";

	fn act(cx: &mut Ctx, form: Self::Form) -> Result<Data> {
		let preview = &mut cx.tab_mut().preview;
		if !preview.fullscreen && preview.lock.is_none() {
			succ!();
		}

		preview.fullscreen = !preview.fullscreen;
		render!();

		act!(app:reflow, cx, form)?;
		act!(mgr:peek, cx)
	}
}
