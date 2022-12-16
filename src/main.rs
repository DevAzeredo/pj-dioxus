mod table_product;


use dioxus::prelude::*;


fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
        } 
        ,table_product::auto{}
        , table_product::tabela_product{}
    
    , div{
        class:"buttons is-right mr-4",
        button{
            class:"button is-success is-medium",
            onclick: move |_| {println!("deu bom")},
            "Enviar Pedido"
        }
    }
    ))
}
