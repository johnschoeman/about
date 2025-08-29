use leptos::prelude::*;

struct Link {
    path: String,
    label: String
}

#[component]
pub fn Header() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let links = vec![
        Link { path: "/".to_string(), label: "home".to_string() },
        Link { path: "/about".to_string(), label: "about".to_string() },
        Link { path: "/doodles".to_string(), label: "doodles".to_string() },
        Link { path: "/work_history".to_string(), label: "work history".to_string() },
    ];

    view! {
        <nav class="mb-12 sm:mb-24">
            <div class="max-w-4xl px-8 mx-auto">
                <div class="flex items-center justify-between h-24">
                    <h1 class="text-lg font-black"><a href="/">john schoeman</a></h1>
                    <div class="flex">
                        {links.into_iter()
                            .map(|link| view! {
                               <NavLink to=link.path label=link.label />
                            })
                            .collect_view()
                        }
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn NavLink(to: String, label: String) -> impl IntoView {
    let (is_active, set_is_active) = signal(false);

    let mobile_base_style = "items-center text-lg font-medium py-2 px-4 border-l-4 block";
    let desktop_base_style = "sm:inline-flex sm:border-b-4 sm:border-l-0 sm:px-2 sm:pt-2";
    let base_style = format!("{} {}", mobile_base_style, desktop_base_style);

    let mobile_active_style = "text-indigo-700 border-indigo-500 bg-indigo-50";
    let desktop_active_style = "sm:text-gray-900 sm:bg-white";
    let active_style = format!("{} {}", mobile_active_style, desktop_active_style);

    let mobile_inactive_style = "text-gray-500 border-transparent hover:border-gray-300 hover:text-gray-700 hover:bg-gray-50";
    let desktop_inactive_style = "sm:hover:bg-white";
    let inactive_style = format!("{} {}", mobile_inactive_style, desktop_inactive_style);

    let style = move || {
        if is_active.get() {
            return format!("{} {}", base_style, active_style)
        } else {
            return base_style
        }
    };

    view! {
        <div class=style()>
            <p>{label}</p>
        </div>
    }
}

//
//   function getProps({ location, href, isPartiallyCurrent, isCurrent }) {
//     isActive = href === "/" ? isCurrent : isPartiallyCurrent || isCurrent;
//     return {}
//   }



//   $: style = isActive ?  `${baseStyle} ${activeStyle}` : `${baseStyle}
//   ${inactiveStyle}`
// <
