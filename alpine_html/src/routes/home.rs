use axum::response::IntoResponse;
use rust_html::{rhtml, Template, TemplateGroup};

use crate::components::*;
use crate::main_layout;

pub async fn home_page() -> impl IntoResponse {
    let result: Template = rhtml! { r#"
        <div class="text-gray-900 dark:text-gray-100 space-y-6">
            <h1 class="text-3xl">Home</h1>
            <div x-data="{{ open: false }}" class="space-y-2">
                <button @click="open = ! open" type="button" class="{ButtonColor::Primary}">Do you want the juith? 🧃</button>
                <blockquote x-show="open" @click.outside="open = false" class="p-4 my-4 bg-gray-50 border-l-4 border-gray-300 dark:border-gray-500 dark:bg-gray-800">
                    The truth is that the universe has been answering you all of your life, but you cannot receive the answers unless you are awake.
                </blockquote>
            </div>
            <div class="space-y-4">
                <h3 class="text-lg">Some interesting boxes</h3>
                {grid_layout()}
            </div>
        </div>
    "# };

    main_layout(result, "/")
}

fn grid_layout() -> Template {
    rhtml! { r#"
    <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
        {TemplateGroup(grid_items())}
    </div>
    "# }
}

fn grid_items() -> Vec<Template> {
    (0..10)
        .map(|i| {
            rhtml! { r#"
            <div class="bg-blue-400 dark:bg-blue-700 dark:text-gray-100 rounded-md px-4 py-2">
                {i}
            </div>
        "# }
        })
        .collect::<Vec<Template>>()
}