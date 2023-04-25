use yew::prelude::*;
use web_sys::HtmlInputElement;


#[function_component(App)]
pub fn app() -> Html {
    let name = use_state(|| String::new());
    let items: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let items_clone: UseStateHandle<Vec<String>> = items.clone();
    let items_clone2: UseStateHandle<Vec<String>> = items.clone();

    let oninput = Callback::from({
        let name = name.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_unchecked_into();
            name.set(target.value());
        }
    });

    let onclick = Callback::from(move |_| {
        if (!name.is_empty()) {
            let mut cloned: Vec<String> = items.to_vec().clone();
            cloned.push(name.to_string());
            items.set(cloned);
        }
    });

    let remove = {
    Callback::from(move |index: usize| {
        let mut cloned: Vec<String> = items_clone2.to_vec().clone();
        cloned.remove(index);
        items_clone2.set(cloned);
        })
    };


    html! {
        <main>
            <h1>{ "To do list" }</h1>
            <input {oninput} />
            <button onclick={onclick}>
                { "Add task" }
            </button>
            <ul class="filters">
            <ul class="item-list">

            {
                items_clone.iter().enumerate().map(|(index, item)| {
                    let remove_list = {
                        let on_click = remove.clone();
                        Callback::from(move |_| {
                                on_click.emit(index);
                        })
                    };

                    html! {
                        <p onclick={remove_list}>{format!("{}: {}", index, item)}</p>
                    }
                }).collect::<Html>() 
            }

        </ul>
        </ul>
        </main>
    }
}
