use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ListType {
    Basic,
    Inline,
    Ordered(ListOrder),
}

impl Default for ListType {
    fn default() -> Self {
        Self::Basic
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ListOrder {
    Number,
    LowercaseLetter,
    UppercaseLetter,
    LowercaseRomanNumber,
    UppercaseRomanNumber,
}

impl Default for ListOrder {
    fn default() -> Self {
        Self::Number
    }
}

impl ToString for ListOrder {
    fn to_string(&self) -> String {
        match self {
            Self::Number => "1",
            Self::LowercaseLetter => "a",
            Self::UppercaseLetter => "A",
            Self::LowercaseRomanNumber => "i",
            Self::UppercaseRomanNumber => "I",
        }
        .into()
    }
}

impl IntoPropValue<Option<AttrValue>> for ListOrder {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(self.to_string().into())
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub r#type: ListType,
}

#[function_component(List)]
pub fn list(props: &Props) -> Html {
    let mut classes = Classes::from("pf-c-list");

    if let ListType::Inline = props.r#type {
        classes.push("pf-m-inline");
    }

    let l: Box<dyn FnOnce(Html) -> Html> = match props.r#type {
        ListType::Basic | ListType::Inline => {
            Box::new(|items| html! {<ul class={classes}>{ items }</ul>})
        }
        ListType::Ordered(n) => {
            Box::new(move |items| html! {<ol r#type={n} class={classes}>{ items }</ol>})
        }
    };

    l(html! {
        {
         for props.children.iter()
            .map(|item|html!{<li>{item}</li>})
        }
    })
}
