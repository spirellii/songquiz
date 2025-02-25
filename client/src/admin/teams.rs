use types::{
    game::{Phase, Team},
    message::AdminInteraction,
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Properties {
    pub callback: Callback<AdminInteraction>,
    pub phase: Phase,
    pub teams: Vec<Team>,
}

#[function_component]
pub fn Teams(props: &Properties) -> Html {
    let onchange = {
        let callback = props.callback.clone();
        move |event: Event| {
            let target = event.target().unwrap_throw();
            let input = target.dyn_into::<HtmlInputElement>().ok().unwrap_throw();
            let team = input
                .get_attribute("data-team")
                .unwrap_throw()
                .parse::<usize>()
                .ok()
                .unwrap_throw();
            callback.emit(AdminInteraction::RenameTeam {
                team: team,
                name: input.value(),
            });
        }
    };
    let onclick = {
        let callback = props.callback.clone();
        move |event: MouseEvent| {
            let target = event.target().unwrap_throw();
            let element = target.dyn_into::<Element>().unwrap_throw();
            let team = element
                .get_attribute("data-team")
                .unwrap_throw()
                .parse::<usize>()
                .ok()
                .unwrap_throw();
            callback.emit(AdminInteraction::DeleteTeam { team: team });
        }
    };
    let teams: Html = props.teams.iter().enumerate().map(|(n, team)| {
        let inactive = if props.phase.is_active(n) {None} else {Some("team-inactive")};
        html! {
            <li class="admin-team">
                <button class="admin-team-remove" data-team={n.to_string()} onclick={onclick.clone()}>{"delete"}</button>
                <input class={classes!("admin-team-name", inactive)} data-team={n.to_string()} value={team.name.clone()} onchange={onchange.clone()}/> 
                <span class="admin-team-points">{team.points.to_string()}{ " Punkte" }</span>    
            </li>
        }
    }).collect();
    let add = {
        let callback = props.callback.clone();
        let onclick = move |_| {
            callback.emit(AdminInteraction::CreateTeam);
        };
        html! {
            <li class="admin-team-add">
                <button class="admin-team-add-button" {onclick}>{ "create" }</button>
                <span class="admin-team-add-text">{"Team hinzufügen"}</span>
            </li>
        }
    };
    html! {
        <ul class={classes!("admin-teams", "container")}>
            { teams }
            { add }
        </ul>
    }
}
