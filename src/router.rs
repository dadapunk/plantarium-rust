use crate::pages::*;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Dashboard {},

    #[route("/garden/:id")]
    GardenDetail { id: String },

    #[route("/bed/:id")]
    BedEditor { id: String },

    #[route("/calendar")]
    Calendar {},

    #[route("/journal")]
    Journal {},

    #[route("/tasks")]
    Tasks {},
}
