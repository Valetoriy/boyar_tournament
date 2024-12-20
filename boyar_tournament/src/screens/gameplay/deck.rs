use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;
use common::Card;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    scaling::{DynamicScale, DynamicTransform},
    screens::{
        ui::{OnPress, UiHitbox},
        GameState,
    },
};

use super::{spawn_text, FontAssets};

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
        Musketeer, Musketeer, Musketeer, Musketeer, Musketeer, Musketeer, Musketeer, Musketeer,
    ];
    cards.shuffle(&mut thread_rng());
    app.insert_resource(Deck(cards));

    app.add_systems(OnEnter(GameState::Gameplay), spawn_card_hand);
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
            name: deck.0[5].tag(),
            aseprite: cards_assets.cards.clone(),
        },
        DeckIndex(5),
        StateScoped(GameState::Gameplay),
        DynamicScale(0.8),
        DynamicTransform(-3.63, -5.7),
    ));
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct SelectedCard(Option<u8>);

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
                scale.0 -= 0.2;
                selected_card.0 = None;

                if index.0 == pressed_index.0 {
                    return;
                }
            }
        }
    }

    let (_, mut pressed_scale) = query.get_mut(entity).unwrap();
    selected_card.0 = Some(pressed_index.0);
    pressed_scale.0 += 0.2;
}

trait IntoTag {
    fn tag(&self) -> String;
}
impl IntoTag for Card {
    fn tag(&self) -> String {
        let s = match self {
            Card::Musketeer => "musketeer",
        };
        s.into()
    }
}
