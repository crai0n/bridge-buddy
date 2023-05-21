use bridge_buddy_core::card::{Card, Suit};
use bridge_buddy_core::evaluator::ForumDPlus2015Evaluator;
use bridge_buddy_core::hand::Hand;
use std::str::FromStr;
use yew::{html, Component, Context, Html};

pub struct App {
    card_vec: Vec<Card>,
    hand_is_valid: bool,
    hand: Option<Hand>,
    hcp: f64,
}

pub enum Msg {
    Switch(Card),
    CreateHand,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            card_vec: vec![],
            hand_is_valid: false,
            hand: None,
            hcp: 0.0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Switch(card) => {
                if let Some(index) = self.card_vec.iter().position(|x| x == &card) {
                    self.card_vec.swap_remove(index);
                } else {
                    self.card_vec.push(card);
                }
                self.card_vec.sort_by(|x, y| y.cmp(x));
                self.hand_is_valid = self.card_vec.len() == 13;
                // console::log!("added {}", card); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::CreateHand => {
                self.hand = Some(Hand::new(self.card_vec[..].try_into().unwrap()));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let denominations = "AKQJT98765432";

        html! {
        <div>
            <div class="spades">
                <p style="color:#001EB4;font-size:20px;">{"♠:"}
                            {
            denominations.chars().into_iter().map(|char| {
                let card_str = format!("S{}", char);
                html! {
                    <button class="button" onclick={ctx.link().callback(move |_| Msg::Switch(Card::from_str(&card_str).unwrap()))}>
                    { char }
                    </button>
                }
            }
            ).collect::<Html>()
            }
                </p>
            </div>
            <div class="hearts">
                <p style="color:#CF0000;font-size:20px;">{"♥:"}
                // A button to send the Increment message
                            {
            denominations.chars().into_iter().map(|char| {
                let card_str = format!("H{}", char);
                html! {
                    <button class="button" onclick={ctx.link().callback(move |_| Msg::Switch(Card::from_str(&card_str).unwrap()))}>
                    { char }
                    </button>
                }
            }
            ).collect::<Html>()
            }
                </p>
            </div>
            <div class="diamonds">
                <p style="color:#FF9B00;font-size:20px;">{"♦:"}
                // A button to send the Increment message
                            {
            denominations.chars().into_iter().map(|char| {
                let card_str = format!("D{}", char);
                html! {
                    <button class="button" onclick={ctx.link().callback(move |_| Msg::Switch(Card::from_str(&card_str).unwrap()))}>
                    { char }
                    </button>
                }
            }
            ).collect::<Html>()
            }
                </p>
            </div>
            <div class="clubs">
                <p style="color:#006409;font-size:20px;">{"♣:"}
                // A button to send the Increment message
                            {
            denominations.chars().into_iter().map(|char| {
                let card_str = format!("C{}", char);
                html! {
                    <button class="button" onclick={ctx.link().callback(move |_| Msg::Switch(Card::from_str(&card_str).unwrap()))}>
                    { char }
                    </button>
                }
            }
            ).collect::<Html>()
            }
                </p>
            </div>

            <p>{ self.card_vec.clone() }</p>
            <p>
            <button class="evaluate" disabled={!self.hand_is_valid} onclick={ctx.link().callback(|_| Msg::CreateHand)}>
                    { "Evaluate" }
            </button>
            </p>

            <p>
            {
                match &self.hand {
                Some(hnd) => html!{<div>
                        <>{hnd}</>
                        <p>{"Hand Type: "}{hnd.hand_type()}</p>
                        <p>{"HCP: "}{ForumDPlus2015Evaluator::hcp(hnd)}</p>
                        <p>{"Length Points: "}{ForumDPlus2015Evaluator::length_points(hnd,None, &[])}</p>
                        <p>{"Adjustments:"}</p>
                        <p>{"Aces and Tens: "}{ForumDPlus2015Evaluator::adjustment_aces_and_tens(hnd)}</p>
                        <p>{"Unguarded Honors: "}{ForumDPlus2015Evaluator::adjustment_unguarded_honors(hnd)}</p>
                        <p>{"Suit Qualities:"}</p>
                        <p>{"Spades: "}{ForumDPlus2015Evaluator::suit_quality(hnd, Suit::Spades)}</p>
                    <p>{"Hearts: "}{ForumDPlus2015Evaluator::suit_quality(hnd, Suit::Hearts)}</p>
                    <p>{"Diamonds: "}{ForumDPlus2015Evaluator::suit_quality(hnd, Suit::Diamonds)}</p>
                    <p>{"Clubs: "}{ForumDPlus2015Evaluator::suit_quality(hnd, Suit::Clubs)}</p>
                    </div>
                },
                None => html!{ <>{"No evaluation exists"}</> },
                }
            }
            </p>
        </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
