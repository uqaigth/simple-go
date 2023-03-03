use std::collections::HashMap;

use ndarray::prelude::*;
use yew::prelude::*;

use crate::go::*;

#[function_component(GoBoard)]
pub fn go_board() -> Html {
    let game_record: GameRecord = use_context::<GameRecord>().expect("no ctx found");

    let shape = game_record.grid_size as usize;
    let base = Array::<(i8, i8), Ix2>::from_shape_fn((shape, shape), |ix| -> (i8, i8) { (ix.0 as i8, ix.1 as i8) });

    return html! {
        <div>
            {
                for base.rows().into_iter().map(|it| -> Html { html!{
                    <div class="board-row">
                        { for it.map(|it| -> Html {
                            html!{ <Point width={46} pos={it} grid_size={game_record.grid_size} /> }
                        }) }
                    </div>
                } })
            }
        </div>
    };
}
