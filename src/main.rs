use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

use crate::utils::fetch;
mod utils;

fn main() {
    sycamore::render(|| {
        view! {
            App{}
        }
    });
}

#[component]
fn App<G: Html>() -> View<G> {
    let currency: Signal<HashMap<String, Currency>> = create_signal(HashMap::new());

    let price_nearby: Signal<f64> = create_signal(1.779);
    let currency_nearby = create_signal(String::from("eur"));
    let price_far: Signal<f64> = create_signal(5.99);
    let currency_far = create_signal(String::from("pln"));

    let conversion_factor = create_memo(move || {
        let near_string = currency_nearby.with(|cur| cur.clone());
        let far_string = currency_far.with(|cur| cur.clone());

        let near = currency.with(|cur| {
            if let Some(val) = cur.get_key_value(&near_string) {
                val.1.rate
            } else {
                1.0
            }
        });

        let far = currency.with(|cur| {
            if let Some(val) = cur.get_key_value(&far_string) {
                val.1.rate
            } else {
                1.0
            }
        });
        near / far
    });

    let price_far_converted=create_memo(move ||{
        price_far.get()*conversion_factor.get()
    });

    spawn_local_scoped(async move {
        fetch("https://www.floatrates.com/daily/eur.json", |response| {
            if let Ok(devs) = serde_json::from_str::<HashMap<String, Currency>>(&response) {
                currency.set(devs);
            }
        })
        .await;
    });

    view! {
        header{}
        main{
            article(class="dual-column"){
                span{"Fuel price Nearby"}
                div{
                    input(bind:valueAsNumber=price_nearby, type="number", min="0", step="0.01")
                    select(bind:value=currency_nearby){
                        CurrencyOptions{}
                    }
                }
                span{"Fuel price far"}
                div{
                    input(bind:valueAsNumber=price_far, type="number", min="0", step="0.01")
                    select(bind:value=currency_far){
                        CurrencyOptions{}
                    }
                }
                (if conversion_factor.get() !=1.0{
                    view!{
                        span{"Conversion Factor"}
                        span{(conversion_factor.get())}
                        span{"Price converted"}
                        span{(price_far_converted.get())}
                    }

                } else {
                    view! { } // Now you don't
                })
  

            }
        }
        footer{
            a(href="https://www.floatrates.com"){
                "Currency values from https://www.floatrates.com"
            }
        }
    }
}

#[component]
fn CurrencyOptions<G: Html>() -> View<G> {
    let supported_currencies = vec![("€", "eur"), ("zł", "pln")];
    View::new_fragment(
        supported_currencies
            .iter()
            .map(|&x| view! { option(value=x.1) { (x.0) } })
            .collect(),
    )
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
struct Currency {
    code: String,
    rate: f64,
}
