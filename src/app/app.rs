use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::app::footer::Footer;
use crate::app::header::Header;
use crate::app::pages::about::About;
use crate::app::pages::doodles::Doodles;
use crate::app::pages::doodles::increment_doodle::IncrementDoodle;
use crate::app::pages::home::home::Home;
use crate::app::pages::work_history::WorkHistory;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="flex flex-col justify-between h-screen overflow-y-scroll text-lg">
                <Header />

                <div class="w-full max-w-3xl px-4 sm:px-8 pb-12 mx-auto mb-auto">
                    <Routes fallback=|| "Page not found.">
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/about") view=About/>
                        <Route path=path!("/doodles") view=Doodles/>
                        <Route path=path!("/doodles/increment") view=IncrementDoodle/>
                        <Route path=path!("/work_history") view=WorkHistory/>
                    </Routes>
                </div>

                <Footer />
            </main>
        </Router>
    }
}
