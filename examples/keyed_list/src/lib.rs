#![recursion_limit="512"]

use rand::Rng;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    persons: Vec<Person>,
    last_id: usize,
    keyed: bool,
}

struct Person {
    id: usize,
    name: String,
    address: String,
    age: usize,
}

pub enum Msg {
    CreatePersons(usize),
    DeletePersonById(usize),
    DeleteEverybody,
    ReverseList,
    SortById,
    SortByName,
    SortByAge,
    SortByAddress,
    ToggleKeyed,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::CreatePersons(20));
        Model {
            link,
            persons: Vec::with_capacity(200),
            last_id: 0,
            keyed: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreatePersons(n) => {
                for _ in 0..n {
                    self.last_id += 1;
                    self.persons.push(Person::new_random(self.last_id));
                }
                true
            }
            Msg::DeletePersonById(id) => {
                if let Some(idx) = self.persons.iter().position(|p| p.id == id) {
                    self.persons.remove(idx);
                    true
                } else {
                    false
                }
            }
            Msg::DeleteEverybody => {
                self.persons.clear();
                true
            }
            Msg::ReverseList => {
                self.persons.reverse();
                true
            }
            Msg::SortById => {
                self.persons.sort_by_key(|p| p.id);
                true
            }
            Msg::SortByName => {
                self.persons.sort_by_cached_key(|p| p.name.clone());
                true
            }
            Msg::SortByAge => {
                self.persons.sort_by_key(|p| p.age);
                true
            }
            Msg::SortByAddress => {
                self.persons.sort_by_cached_key(|p| p.address.clone());
                true
            }
            Msg::ToggleKeyed => {
                self.keyed = !self.keyed;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="buttons">
                    <button onclick=self.link.callback(|_| Msg::DeleteEverybody)>
                        { "Delete everybody" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::CreatePersons(1))>
                        { "Create 1" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::CreatePersons(20))>
                        { "Create 20" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::CreatePersons(100))>
                        { "Create 100" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::ReverseList)>
                        { "Reverse list" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::SortById)>
                        { "Sort by id" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::SortByName)>
                        { "Sort by name" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::SortByAge)>
                        { "Sort by age" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::SortByAddress)>
                        { "Sort by address" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::ToggleKeyed)>
                        { if self.keyed { "Disable keys" } else { "Enable keys" } }
                    </button>
                </div>
                <div class="persons">
                    { for self.persons.iter().map(|p| view_person(p, self.keyed)) }
                </div>
            </>
        }
    }
}

fn view_person(person: &Person, keyed: bool) -> Html {
    if keyed {
        html! {
            <div class="person" key=person.age.to_string()>
                <h1>{ &person.id }{ " - " }{ &person.name }</h1>
                <p>{ "Age: " }{ &person.age }</p>
                <p>{ "Address: " }{ &person.address }</p>
            </div>
        }
    } else {
        html! {
            <div class="person">
                <h1>{ &person.id }{ " - " }{ &person.name }</h1>
                <p>{ "Age: " }{ &person.age }</p>
                <p>{ "Address: " }{ &person.address }</p>
            </div>
        }
    }
}

// fn square_class(this: (u32, u32), selected: Option<(u32, u32)>) -> &'static str {
//     match selected {
//         Some(xy) if xy == this => "square_green",
//         _ => "square_red",
//     }
// }

impl Person {
    fn new_random(id: usize) -> Self {
        Person {
            id,
            name: Person::gen_name(),
            age: Person::gen_age(),
            address: Person::gen_address(),
        }
    }

    fn gen_number(min: usize, max: usize) -> usize {
        let len: usize = rand::thread_rng().gen();
        len % (max - min) + min
    }

    fn gen_string(len: usize) -> String {
        let mut rng = rand::thread_rng();
        (0..len)
            .map(|_| rng.sample(rand::distributions::Alphanumeric))
            .collect()
    }

    fn gen_words(n_words: usize, min_len: usize, max_len: usize) -> Vec<String> {
        (0..n_words)
            .map(|_| Person::gen_string(Person::gen_number(min_len, max_len)))
            .collect()
    }

    fn gen_name() -> String {
        Person::gen_words(2, 4, 15).join(" ")
    }

    fn gen_age() -> usize {
        Person::gen_number(7, 77)
    }

    fn gen_address() -> String {
        let n_words = Person::gen_number(3, 6);
        Person::gen_words(n_words, 5, 12).join(" ")
    }
}
