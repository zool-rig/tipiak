use dioxus::prelude::*;

use crate::api::completion::completion;

#[derive(Props, Clone, PartialEq)]
pub struct SearchBarProps {
    pattern: Signal<String>,
    on_submit: EventHandler<()>,
}

#[component]
pub fn SearchBar(mut props: SearchBarProps) -> Element {
    let mut suggestions = use_signal(|| Vec::<String>::new());
    let mut show_dropdown = use_signal(|| false);
    let mut input_element: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let mut selected_index = use_signal(|| None::<usize>);
    let item_refs: Signal<Vec<Option<std::rc::Rc<MountedData>>>> = use_signal(|| Vec::new());

    use_effect(move || {
        if let Some(i) = selected_index() {
            if let Some(Some(element)) = item_refs().get(i) {
                let _ = element.scroll_to(ScrollBehavior::Smooth);
            }
        }
    });

    rsx! {
        div {
            class: "search-container",
            input {
                class: "search-bar",
                type: "text",
                placeholder: "🔎 Search files ...",
                value: "{props.pattern}",
                oninput: move |evt| {
                    let value = evt.value();
                    props.pattern.set(value.clone());

                    if value.is_empty() || value.chars().last().unwrap() == ' ' {
                        suggestions.set(vec![]);
                        show_dropdown.set(false);
                        return;
                    }

                    spawn({
                        let mut suggestions = suggestions.clone();
                        let mut show_dropdown = show_dropdown.clone();

                        async move {
                            match completion(value.split_whitespace().last().unwrap_or("").to_string()).await {
                                Ok(tokens) => {
                                    if !tokens.is_empty() {
                                        suggestions.set(tokens);
                                        show_dropdown.set(true);
                                    } else {
                                        suggestions.set(vec![]);
                                        show_dropdown.set(false);
                                    }
                                },
                                Err(_) => {
                                    suggestions.set(vec![]);
                                    show_dropdown.set(false);
                                }
                            }
                        }
                    });
                },
                onkeydown: move |evt| {
                    match evt.key() {
                        Key::Enter => {
                            if show_dropdown() {
                                if let Some(i) = selected_index() {
                                    if let Some(suggestion) = suggestions().get(i) {
                                        let words: Vec<String> = props
                                            .pattern
                                            .read()
                                            .split_whitespace()
                                            .map(|w| w.to_string())
                                            .collect();
                                        let mut new_pattern = words[..words.len() - 1].join(" ");
                                        new_pattern.push(' ');
                                        new_pattern.push_str(&suggestion);
                                        props.pattern.set(new_pattern.trim().to_string());
                                        show_dropdown.set(false);
                                        if let Some(input) = input_element() {
                                            let _ = input.set_focus(true);
                                        }
                                    }
                                } else {
                                    props.on_submit.call(());
                                }
                            } else {
                                props.on_submit.call(());
                            }
                        },
                        Key::ArrowDown => {
                            evt.prevent_default();
                            if !suggestions().is_empty() {
                                let suggestions_count = suggestions().len();
                                selected_index.set(Some(match selected_index() {
                                    Some(i) => (i + 1) % suggestions_count,
                                    None => 0
                                }))
                            }
                        },
                        Key::ArrowUp => {
                            evt.prevent_default();
                            if !suggestions().is_empty() {
                                let suggestions_count = suggestions().len();
                                selected_index.set(Some(match selected_index() {
                                    Some(i) => if i == 0 { suggestions_count - 1 } else { i - 1 },
                                    None => suggestions_count - 1
                                }))
                            }
                        }
                        _ => {}
                    }
                },
                onmount: move |evt| {
                    input_element.set(Some(evt.data()));
                }
            }

            if show_dropdown() && !suggestions().is_empty() {
                ul {
                    class: "autocomplete",

                    for (i, suggestion) in suggestions.iter().enumerate() {
                        li {
                            class: if Some(i) == selected_index() {
                                "suggestion-selected"
                            } else {
                                "suggestion"
                            },
                            onclick: {
                                let suggestion = suggestion.clone();
                                move |_| {
                                    let words: Vec<String> = props
                                        .pattern
                                        .read()
                                        .split_whitespace()
                                        .map(|w| w.to_string())
                                        .collect();
                                    let mut new_pattern = words[..words.len() - 1].join(" ");
                                    new_pattern.push(' ');
                                    new_pattern.push_str(&suggestion);
                                    props.pattern.set(new_pattern.trim().to_string());
                                    show_dropdown.set(false);
                                    if let Some(input) = input_element() {
                                        let _ = input.set_focus(true);
                                    }
                                }
                            },
                            onmount: {
                                let mut item_refs = item_refs.clone();
                                move |evt| {
                                    item_refs.with_mut(|refs| {
                                        if refs.len() <= i {
                                            refs.resize(i + 1, None);
                                        }
                                        refs[i] = Some(evt.data());
                                    });
                                }
                            },
                            "{suggestion}"
                        }
                    }
                }
            }
        }
    }
}
