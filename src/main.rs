use std::collections::HashMap;

use yew::prelude::*;

use go::*;

mod go;

#[function_component(App)]
fn app() -> Html {
    let record = use_state_eq(|| GameRecord {
        rule_set: RuleSet::China,
        grid_size: 19,
        step: 1,
        steps: vec![],
        next: Stone::Black,
        capture: HashMap::new(),
    });

    html! {
        <ContextProvider<GameRecord> context={(*record).clone()}>
            <ContextProvider<UseStateSetter<GameRecord>> context={record.setter()}>
                <div class={classes!("board-container")}>
                    <go::GoBoard />
                </div>
            </ContextProvider<UseStateSetter<GameRecord>>>
        </ContextProvider<GameRecord>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
