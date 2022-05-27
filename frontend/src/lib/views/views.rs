// app views

use yew::prelude::*;

use super::super::components::Header;
use super::super::components::Content;
use super::super::components::Footer;


// home view
pub struct Home;

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header />
                <Content />
                <Footer />
            </>
        }
    }
}