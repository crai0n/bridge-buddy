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
        // let counter = use_state(|| 0);
        // let onclick = {
        //     let counter = counter.clone();
        //     move |_| {
        //         let value = *counter + 1;
        //         counter.set(value);
        //     }
        // };
        // let addcard = {
        //     let counter = counter.clone();
        //     move |_| {
        //         let value = *counter + 1;
        //         counter.set(value);
        //     }
        // };

        let hand = Hand::from_str("S:AKQJ, H:T98, D: 765, C: 432");

        let _hand_html: Html = match hand {
            Ok(hnd) => html! {
                <p>{"My hand is: "}{hnd}</p>
            },
            Err(_) => html! {
                <p>{"Invalid Hand"}</p>
            },
        };

        html! {
            <div>
            <div class="spades">
                <p style="color:#001EB4;font-size:20px;">{"♠:"}
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("SA").unwrap()))}>
                    { "A" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("SK").unwrap()))}>
                    { "K" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("SQ").unwrap()))}>
                    { "Q" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("SJ").unwrap()))}>
                    { "J" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("ST").unwrap()))}>
                    { "T" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("S9").unwrap()))}>
                    { "9" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("S8").unwrap()))}>
                    { "8" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("S7").unwrap()))}>
                    { "7" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("S6").unwrap()))}>
                    { "6" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("S5").unwrap()))}>
                    { "5" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("S4").unwrap()))}>
                    { "4" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("S3").unwrap()))}>
                    { "3" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("S2").unwrap()))}>
                    { "2" }
                </button>
                </p>
            </div>
            <div class="hearts">
                <p style="color:#CF0000;font-size:20px;">{"♥:"}
                // A button to send the Increment message
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("HA").unwrap()))}>
                    { "A" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("HK").unwrap()))}>
                    { "K" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("HQ").unwrap()))}>
                    { "Q" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("HJ").unwrap()))}>
                    { "J" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("HT").unwrap()))}>
                    { "T" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("H9").unwrap()))}>
                    { "9" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("H8").unwrap()))}>
                    { "8" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("H7").unwrap()))}>
                    { "7" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("H6").unwrap()))}>
                    { "6" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("H5").unwrap()))}>
                    { "5" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("H4").unwrap()))}>
                    { "4" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("H3").unwrap()))}>
                    { "3" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("H2").unwrap()))}>
                    { "2" }
                </button>
                </p>
            </div>
            <div class="diamonds">
                <p style="color:#FF9B00;font-size:20px;">{"♦:"}
                // A button to send the Increment message
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("DA").unwrap()))}>
                    { "A" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("DK").unwrap()))}>
                    { "K" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("DQ").unwrap()))}>
                    { "Q" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("DJ").unwrap()))}>
                    { "J" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("DT").unwrap()))}>
                    { "T" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("D9").unwrap()))}>
                    { "9" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("D8").unwrap()))}>
                    { "8" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("D7").unwrap()))}>
                    { "7" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("D6").unwrap()))}>
                    { "6" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("D5").unwrap()))}>
                    { "5" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("D4").unwrap()))}>
                    { "4" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("D3").unwrap()))}>
                    { "3" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("D2").unwrap()))}>
                    { "2" }
                </button>
                </p>
            </div>
            <div class="clubs">
                <p style="color:#006409;font-size:20px;">{"♣:"}
                // A button to send the Increment message
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("CA").unwrap()))}>
                    { "A" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("CK").unwrap()))}>
                    { "K" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("CQ").unwrap()))}>
                    { "Q" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("CJ").unwrap()))}>
                    { "J" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("CT").unwrap()))}>
                    { "T" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("C9").unwrap()))}>
                    { "9" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("C8").unwrap()))}>
                    { "8" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("C7").unwrap()))}>
                    { "7" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("C6").unwrap()))}>
                    { "6" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("C5").unwrap()))}>
                    { "5" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("C4").unwrap()))}>
                    { "4" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("C3").unwrap()))}>
                    { "3" }
                </button>
                <button class="button" onclick={ctx.link().callback(|_| Msg::Switch(Card::from_str("C2").unwrap()))}>
                    { "2" }
                </button>
            </p>
            </div>

                // <p>{ counter }</p>
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
