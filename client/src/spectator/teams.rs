use types::game::{Phase, Team};
use yew::prelude::*;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Properties {
    pub phase: Phase,
    pub teams: Vec<Team>,
}

#[function_component]
pub fn Teams(props: &Properties) -> Html {
    let teams: Html = props
        .teams
        .iter()
        .enumerate()
        .map(|(n, team)| {
            let inactive = if props.phase.is_active(n) {
                None
            } else {
                Some("team-inactive")
            };
            html! {
                <li class="team">
                    <span class={classes!("team-name", inactive)}>{team.name.clone()}</span>
                    <span class="team-points">{" "} {team.points.to_string()} {" Punkte"}</span>
                </li>
            }
        })
        .collect();
    if props.teams.len() > 0 {
        html! {
            <ul class={classes!("teams", "container")}>
                { teams }
            </ul>
        }
    } else {
        html! {}
    }
}
