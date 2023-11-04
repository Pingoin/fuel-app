use std::collections::HashMap;

use serde::{ Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

use crate::utils::{fetch, create_stored_signal};
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
    let currency: Signal<HashMap<String, Currency>> =
        create_stored_signal(String::from("currencies"), HashMap::new());

    let price_nearby = create_stored_signal(String::from("price_nearby"), 1.779f64);

    let currency_nearby =
        create_stored_signal(String::from("currency_nearby"), String::from("eur"));
    let price_far = create_stored_signal(String::from("price_far"), 5.779f64);
    let currency_far = create_stored_signal(String::from("currency_far"), String::from("pln"));

    let fuel_usage=create_stored_signal(String::from("fuel_usage"), 5.0f64);
    let fueling_detour_km=create_stored_signal(String::from("fueling_detour_km"), 50.0f64);
    let fuel_amount=create_stored_signal(String::from("fuel amount"), 45.0f64);

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

    let price_far_converted = create_memo(move || price_far.get() * conversion_factor.get());

    let fuel_kosts_near=create_memo(move || {
        price_nearby.get()*fuel_amount.get()
    });

    let fuel_kosts_far=create_memo(move || {
        price_far_converted.get()*fuel_amount.get()
    });

    let detour_kosts=create_memo(move || {
        price_far_converted.get()*fuel_usage.get()*fueling_detour_km.get()/100.0
    });

    let savings=create_memo(move || {
       fuel_kosts_near.get()-fuel_kosts_far.get()-detour_kosts.get()
    });

    currency.track();
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
                    " "
                    select(bind:value=currency_nearby){
                        CurrencyOptions{}
                    }
                }
                span{"Fuel price far"}
                div{
                    input(bind:valueAsNumber=price_far, type="number", min="0", step="0.01")
                    " "
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
                span{" Fuel usage"}
                div{
                    input(bind:valueAsNumber=fuel_usage, type="number", min="0", step="0.1")
                    "l/100 km"
                }
                span{"Detour to cheaper fuelstation"}
                div{
                    input(bind:valueAsNumber=fueling_detour_km, type="number", min="0", step="1")
                    " km"
                }
                span{"fuel amount"}
                div{
                    input(bind:valueAsNumber=fuel_amount, type="number", min="0", step="1")
                    " l"
                }
                span{"fuel kosts near"}
                div{
                    ((fuel_kosts_near.get()*100.0).round() / 100.0)
                    " €"
                }
                span{"fuel kosts far"}
                div{
                    ((fuel_kosts_far.get()*100.0).round() / 100.0)
                    " €"
                }
                span{"detour kosts far"}
                div{
                    ((detour_kosts.get()*100.0).round() / 100.0)
                    " €"
                }
                span{"savings"}
                div{
                    ((savings.get()*100.0).round() / 100.0)
                    " €"
                }

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
