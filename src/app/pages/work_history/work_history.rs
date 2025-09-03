use leptos::prelude::*;

use super::detailed_experience::DetailedExperience;
use super::full_timeline::FullTimeline;
use super::work_history_header::WorkHistoryHeader;
use super::work_summary::WorkSummary;

#[component]
pub fn WorkHistory() -> impl IntoView {
    view! {
        <div>
            <WorkHistoryHeader />
            <div class="text-gray-900 leading-normal break-words">
                <WorkSummary />
                <DetailedExperience />
                <FullTimeline />
            </div>
        </div>
    }
}
