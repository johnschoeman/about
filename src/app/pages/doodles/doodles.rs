use leptos::prelude::*;

use super::increment_doodle::IncrementDoodle;

#[component]
pub fn Doodles() -> impl IntoView {
    let doodles = vec![
        ("increment", "/doodles/increment"),
        (
            "triangle subdivision",
            "https://boring-hypatia-5625e0.netlify.app/doodles/triangle_subdivision/",
        ),
        (
            "modular",
            "https://clinquant-chebakia-3013c1.netlify.app/modular_times_table/",
        ),
        (
            "black sheep",
            "https://clinquant-chebakia-3013c1.netlify.app/black_sheep_jump/",
        ),
        (
            "lock puzzle",
            "https://clinquant-chebakia-3013c1.netlify.app/lock_puzzle/",
        ),
        (
            "squares",
            "https://clinquant-chebakia-3013c1.netlify.app/squares/",
        ),
        (
            "dots",
            "https://clinquant-chebakia-3013c1.netlify.app/dots/",
        ),
        (
            "recaman",
            "https://clinquant-chebakia-3013c1.netlify.app/recaman/",
        ),
        (
            "bpad",
            "https://boring-hypatia-5625e0.netlify.app/doodles/b_pad/",
        ),
        (
            "tic tac toe",
            "https://boring-hypatia-5625e0.netlify.app/doodles/tic_tac_toe/",
        ),
        (
            "solar system 2",
            "https://boring-hypatia-5625e0.netlify.app/doodles/solar_system_2/",
        ),
        (
            "asteroids",
            "https://boring-hypatia-5625e0.netlify.app/doodles/asteroids/",
        ),
        (
            "tree",
            "https://boring-hypatia-5625e0.netlify.app/doodles/tree/",
        ),
        (
            "solar system",
            "https://boring-hypatia-5625e0.netlify.app/doodles/solar_system/",
        ),
        (
            "mandlebrot",
            "https://boring-hypatia-5625e0.netlify.app/doodles/mandlebrot/",
        ),
        (
            "koch curve",
            "https://boring-hypatia-5625e0.netlify.app/doodles/koch_curve/",
        ),
        (
            "mirror primes",
            "https://boring-hypatia-5625e0.netlify.app/doodles/mirror_primes/",
        ),
    ];

    let length = doodles.len();

    view! {
        <div>
            <h1 class="text-6xl font-black text-gray-900">"doodles"</h1>
            <p class="mt-2 text-sm text-gray-700">"a collection of programming 'doodles'"</p>

            <ul class="mt-8">
                {doodles
                    .into_iter()
                    .enumerate()
                    .map(|(idx, (label, url))| {
                        let idx_str = format!("{:02}", length - idx - 1);
                        view! {
                            <li class="flex flex-row items-center mt-8">
                                <p class="w-12 h-full text-2xl font-bold text-gray-400 align-middle">
                                    {idx_str}
                                </p>
                                <a
                                    class="text-2xl align-middle text-blue-600 underline hover:text-blue-800 visited:text-indigo-600"
                                    href={url}
                                >
                                    {label}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}
