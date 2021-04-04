use leaflet::{LatLng, Map, TileLayer};
use seed::{
    button, div,
    prelude::{ev, El, Ev, JsValue, Node, Orders, RenderInfo, UpdateEl},
    App, Url,
};

fn main() {
    App::start("surway", init, update, view);
}

type Model = i32;

#[derive(Debug, Clone, Copy)]
enum Msg {
    Increment,
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(init_map);

    Model::default()
}

fn init_map(_: RenderInfo) {
    let map = Map::new("map", &JsValue::NULL);
    map.setView(&LatLng::new(55.75, 37.6), 10.0);

    TileLayer::new(
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
        &JsValue::NULL,
    )
    .addTo(&map);
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        "Clicks: ",
        button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}
