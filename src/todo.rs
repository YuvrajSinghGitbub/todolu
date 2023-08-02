use leptos::{ev::SubmitEvent, html::Input, *};
use uuid::Uuid;

#[allow(unused)]
#[derive(Debug, Clone)]
struct Todo {
    item: String,
    is_done: bool,
    id: Uuid,
}

impl Todo {
    fn new(item: String) -> Self {
        Todo {
            item,
            is_done: false,
            id: Uuid::new_v4(),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
struct Todos {
    items: Vec<Todo>,
}

#[allow(dead_code)]
impl Todos {
    fn new(items: Vec<&str>) -> Self {
        let mut todo_items = Vec::new();

        for item in items {
            todo_items.push(Todo::new(item.to_owned()))
        }

        Todos { items: todo_items }
    }

    fn add(&mut self, new_item: String) {
        self.items.push(Todo::new(new_item));
    }

    fn delete(&mut self, which: Uuid) {
        self.items = self
            .items
            .clone()
            .into_iter()
            .filter(|item| item.id != which)
            .collect();
    }
}

#[component]
fn AddTaskForm(cx: Scope, todo_items: ReadSignal<Todos>) -> impl IntoView {
    let add_items: WriteSignal<Todos> =
        use_context(cx).expect("no context provided while adding a new todo");
    let node_ref: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let new_task = node_ref
            .get()
            .expect("<input> to new todo items does not exist")
            .value();

        add_items.update(|old_todos| old_todos.add(new_task));

        log!("new task added \ntask list: {:?}", todo_items.get().items);
    };
    view! {cx,
        <form class="task-form" on:submit=on_submit>
            <input
                class="task-input"
                placeholder="enter a task..."
                type="text"
                node_ref=node_ref
            />
            <input class="task-add-btn" type="submit" value="Add task" />
        </form>

    }
}

#[component]
fn TaskList(cx: Scope, todo_items: ReadSignal<Todos>) -> impl IntoView {
    let remove_items: WriteSignal<Todos> =
        use_context(cx).expect("no context provided for deleting items");

    view! {cx,
        <div class="task-list">
            <For
                each=move || todo_items.get().items
                key=|task| task.id
                view=move |cx, task| {
                    view! {cx,
                        <div class="each-task">
                            <p class="task">{task.item}</p>
                            <button
                                class="task-remove-btn"
                                on:click=move |_| { remove_items.update(|old_todos| old_todos.delete(task.id)) }
                            >
                            "◯"
                            </button>
                        </div>
                    }
                }
            />
        </div>
    }
}

#[allow(unused)]
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (todo_items, mark_items) =
        create_signal(cx, Todos::new(vec!["task 1", "task 2", "task 3"]));

    provide_context(cx, mark_items);

    view! {cx,
        <div class="app">
            <AddTaskForm todo_items=todo_items/>
            <TaskList todo_items=todo_items/>
        </div>
    }
}
