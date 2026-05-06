use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "bg-gray-800 text-white p-4 flex gap-4",
            Link {
                to: Route::Home {},
                class: "hover:text-orange-400",
                "🏠 Accueil"
            }
            Link {
                to: Route::Blog { id: 1 },
                class: "hover:text-orange-400",
                "📝 Blog"
            }
        }
        Outlet::<Route> {}
    }
}