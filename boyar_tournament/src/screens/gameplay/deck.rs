use bevy::{input::common_conditions::input_just_released, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_quinnet::client::QuinnetClient;
use common::{ArenaPos, Card, ClientChannel, ClientMessage};
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    scaling::{DynamicScale, DynamicTransform},
    screens::{
        ui::{OnPress, UiHitbox},
        GameState,
    },
};

use super::{arena::MouseArenaPos, spawn_text, FontAssets};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Deck>();
    app.register_type::<DeckIndex>();
    app.register_type::<SelectedCard>();

    app.init_resource::<SelectedCard>();

    app.configure_loading_state(
        LoadingStateConfig::new(GameState::Loading).load_collection::<CardsAssets>(),
    );

    use Card::*;
    let mut cards = [
        Rus,
        Musketeer,
        ThreeMusketeers,
        Priest,
        Bats,
        BatHorde,
        Giant,
        Bomber,
    ];
    cards.shuffle(&mut thread_rng());
    app.insert_resource(Deck(cards));

    app.add_systems(
        Update,
        play_card.run_if(
            in_state(GameState::Gameplay).and(
                input_just_released(MouseButton::Left)
                    .or(|touch: Res<Touches>| touch.any_just_released()),
            ),
        ),
    );

    app.add_systems(OnEnter(GameState::Gameplay), spawn_card_hand);
    app.add_observer(update_card_hand);
}

#[derive(AssetCollection, Resource)]
struct CardsAssets {
    #[asset(path = "cards.aseprite")]
    cards: Handle<Aseprite>,
    #[asset(path = "screens/gameplay/card_select.ogg")]
    card_select: Handle<AudioSource>,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct Deck([Card; 8]);

#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
struct DeckIndex(u8);

fn spawn_card_hand(
    mut cmd: Commands,
    cards_assets: ResMut<CardsAssets>,
    deck: Res<Deck>,
    font: Res<FontAssets>,
) {
    for (i, (pos, card)) in [-2.05, -0.22, 1.62, 3.45].iter().zip(deck.0).enumerate() {
        cmd.spawn((
            Name::new(format!("Карта {}", i + 1)),
            AseSpriteSlice {
                name: card.tag(),
                aseprite: cards_assets.cards.clone(),
            },
            DeckIndex(i as _),
            StateScoped(GameState::Gameplay),
            DynamicScale(1.8),
            DynamicTransform(*pos, -6.28),
            UiHitbox(1.8, 2.3),
        ))
        .observe(on_card_select);
    }

    spawn_text(
        &mut cmd,
        "След.",
        font.font.clone(),
        35.,
        1.,
        (-3.63, -5.05),
        GameState::Gameplay,
    );
    cmd.spawn((
        Name::new("Следующая карта"),
        AseSpriteSlice {
            name: deck.0[4].tag(),
            aseprite: cards_assets.cards.clone(),
        },
        DeckIndex(4),
        StateScoped(GameState::Gameplay),
        DynamicScale(0.8),
        DynamicTransform(-3.63, -5.7),
    ));
}

trait IntoTag {
    fn tag(&self) -> String;
}
impl IntoTag for Card {
    fn tag(&self) -> String {
        let s = match self {
            Card::Musketeer => "musketeer",
            Card::Rus => "rus",
            Card::ThreeMusketeers => "three_musketeers",
            Card::Priest => "priest",
            Card::Bats => "bats",
            Card::BatHorde => "bat_horde",
            Card::Giant => "giant",
            Card::Bomber => "bomber",
        };
        s.into()
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct SelectedCard(Option<u8>);

const SELECTED_CARD_SCALE_AMOUNT: f32 = 0.2;

fn on_card_select(
    trigger: Trigger<OnPress>,
    mut selected_card: ResMut<SelectedCard>,
    mut query: Query<(&DeckIndex, &mut DynamicScale)>,
    mut cmd: Commands,
    cards_assets: ResMut<CardsAssets>,
) {
    cmd.spawn((
        AudioPlayer::new(cards_assets.card_select.clone()),
        PlaybackSettings::DESPAWN,
    ));

    let entity = trigger.entity();
    let (&pressed_index, _) = query.get(entity).unwrap();

    if let Some(selected_index) = selected_card.0 {
        for (index, mut scale) in &mut query {
            if index.0 == selected_index {
                scale.0 -= SELECTED_CARD_SCALE_AMOUNT;
                selected_card.0 = None;

                if index.0 == pressed_index.0 {
                    return;
                }
            }
        }
    }

    let (_, mut pressed_scale) = query.get_mut(entity).unwrap();
    selected_card.0 = Some(pressed_index.0);
    pressed_scale.0 += SELECTED_CARD_SCALE_AMOUNT;
}

fn play_card(
    mouse_pos: Res<MouseArenaPos>,
    selected_card: Res<SelectedCard>,
    mut deck: ResMut<Deck>,
    mut client: ResMut<QuinnetClient>,
    mut cmd: Commands,
) {
    let Some(mouse_pos) = mouse_pos.0 else {
        return;
    };
    let Some(index) = selected_card.0 else {
        return;
    };
    let index = index as usize;
    let card = deck.0[index];

    // Ставим точку в центр клетки
    let x = mouse_pos.0.floor() + 0.5;
    let y = mouse_pos.1.floor() + 0.5;

    client
        .connection_mut()
        .send_message_on(
            ClientChannel::OrderedReliable,
            ClientMessage::PlayCard {
                card,
                placement: ArenaPos(x, y),
            },
        )
        .unwrap();

    // Передвигаем карты в колоде на 1
    deck.0[index] = deck.0[4];
    deck.0[4] = deck.0[5];
    deck.0[5] = deck.0[6];
    deck.0[6] = deck.0[7];
    deck.0[7] = card;

    cmd.trigger(UpdateCardHand);
}

#[derive(Event)]
struct UpdateCardHand;

fn update_card_hand(
    _: Trigger<UpdateCardHand>,
    deck: Res<Deck>,
    mut query: Query<(&DeckIndex, &mut AseSpriteSlice, &mut DynamicScale)>,
    mut selected_card: ResMut<SelectedCard>,
) {
    for (index, mut sprite, mut scale) in &mut query {
        if index.0 == selected_card.0.unwrap() {
            scale.0 -= SELECTED_CARD_SCALE_AMOUNT;
        }

        let card = deck.0[index.0 as usize];
        sprite.name = card.tag();
    }

    selected_card.0 = None;
}
