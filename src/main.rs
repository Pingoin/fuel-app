use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

use crate::utils::fetch;
mod utils;

fn main() {
    sycamore::render(|cx| {
        view! {cx,
            App{}
        }
    });
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let currency: &Signal<HashMap<String, Currency>> = create_signal(cx, HashMap::new());
    let currency_vec:&Signal<Vec<String>> = create_signal(cx, Vec::new());

    let price_nearby:&Signal<f64>=create_signal(cx, 0.0);
    let price_far:&Signal<f64>=create_signal(cx, 0.0);

    spawn_local_scoped(cx, async move {
        fetch("https://www.floatrates.com/daily/eur.json", |response| {
            if let Ok(devs) = serde_json::from_str::<HashMap<String, Currency>>(&response) {
                let vec=devs.clone().into_keys().collect();

                currency.set(devs);
                currency_vec.set(vec)
            }
        })
        .await;
    });

    view! { cx,
        header{}
        main{
            article{
                ul {
                    Indexed(
                        iterable=currency_vec,
                        view=|cx, x| view! { cx,
                            li { (x) }
                        },
                    )
                }
            }
        }
        footer{"Hello, World!"}
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
struct Currency {
    code: String,
    rate: f64,
}
