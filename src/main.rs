use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        header{}
        main{
            article{
                "pinn"
            }
        }
        footer{"Hello, World!"}
    });
}