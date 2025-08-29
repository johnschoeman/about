use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::hooks::use_location;

#[derive(Clone)]
struct NavItem {
    path: &'static str,
    label: &'static str,
}

#[component]
pub fn Header() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let handle_on_click = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    let links = vec![
        NavItem {
            path: "/",
            label: "home",
        },
        NavItem {
            path: "/about",
            label: "about",
        },
        NavItem {
            path: "/doodles",
            label: "doodles",
        },
        NavItem {
            path: "/work_history",
            label: "work history",
        },
    ];

    view! {
        <nav class="mb-12 sm:mb-24">
            <div class="max-w-4xl px-8 mx-auto">
                <div class="flex items-center justify-between h-24">
                    <h1 class="text-lg font-black">
                        <A href="/">"john schoeman"</A>
                    </h1>
                    <div class="flex">
                        <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                            {links.clone().into_iter()
                                .map(|link| view! {
                                   <NavLink path=link.path label=link.label />
                                })
                                .collect_view()
                            }
                        </div>
                    </div>

                    <div class="flex items-center -mr-2 sm:hidden">
                        <button
                            type="button"
                            on:click=handle_on_click
                            class="inline-flex items-center justify-center p-2 text-gray-400 rounded-md hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
                            aria-controls="mobile-menu"
                            aria-expanded="false"
                        >
                            <span class="sr-only">"Open main menu"</span>
                            <svg
                                class="block w-6 h-6"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>

            {move || is_open.get().then_some(view! {
                <div class="sm:hidden" id="mobile-menu">
                    <div class="pt-2 pb-3">
                        {links.clone().into_iter()
                            .map(|link| view! {
                               <NavLink path=link.path label=link.label />
                            })
                            .collect_view()
                        }
                    </div>
                </div>
            })}
        </nav>
    }
}

#[component]
fn NavLink(path: &'static str, label: &'static str) -> impl IntoView {
    let location = use_location();

    let mobile_base_style = "items-center text-lg font-medium py-2 px-4 border-l-4 block";
    let desktop_base_style = "sm:inline-flex sm:border-b-4 sm:border-l-0 sm:px-2 sm:pt-2";
    let base_style = format!("{} {}", mobile_base_style, desktop_base_style);

    let mobile_inactive_style = "text-gray-500 border-transparent hover:border-gray-300 hover:text-gray-700 hover:bg-gray-50";
    let desktop_inactive_style = "sm:hover:bg-white";
    let inactive_style = format!("{} {}", mobile_inactive_style, desktop_inactive_style);

    let mobile_active_style = "text-indigo-700 border-indigo-500 bg-indigo-50";
    let desktop_active_style = "sm:text-gray-900 sm:bg-white";
    let active_style = format!("{} {}", mobile_active_style, desktop_active_style);

    let is_active = move || {
        let current_path = location.pathname.get();
        if path == "/" {
            current_path == "/"
        } else {
            current_path.starts_with(path)
        }
    };

    let link_classes = move || {
        if is_active() {
            format!("{} {}", base_style, active_style)
        } else {
            format!("{} {}", base_style, inactive_style)
        }
    };

    view! {
        <A
            href=path
            attr:class=link_classes
            // exact=if path == "/" { true } else { false }
        >
            {label}
        </A>
    }
}
