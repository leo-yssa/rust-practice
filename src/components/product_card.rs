use crate::components::AtcButton;
use crate::route::Route;
use crate::types::Product;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct ProductCard {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub product: Product,
    pub on_add_to_cart: Callback<Product>,
}

impl Component for ProductCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        html! {
          <div class="product__card_container">
            <Anchor route=Route::ProductDetail(self.props.product.id) classes="product__card_anchor">
                <img class="product__card_image" src={&self.props.product.image}/>
                <div class="product__card_name">{&self.props.product.name}</div>
                <div class="product__card_price">{"$"}{&self.props.product.price}</div>
                <AtcButton product=self.props.product.clone() on_add_to_cart=self.props.on_add_to_cart.clone() />
            </Anchor>
          </div>
        }
    }
}